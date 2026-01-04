use crate::app_state::AppState;
use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

// ✅ OTP deps
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256};

// ✅ email sender
use crate::mail::send_otp_email;

#[derive(Serialize)]
pub struct UserResponse {
    pub message: String,
}

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 4, max = 16, message = "username min 4 and max 16 characters"))]
    pub username: String,

    #[validate(length(min = 8, max = 16, message = "password min 8 and max 16 characters"))]
    pub password: String,

    #[validate(email)]
    pub email: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(FromRow)]
pub struct UserSql {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[debug_handler]
pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    if let Err(err) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(UserResponse {
                message: format!("{}", err),
            }),
        )
            .into_response();
    }

    let hashed = hash(&payload.password, DEFAULT_COST).unwrap();

    let result =
        sqlx::query("INSERT INTO users(username, password, email) VALUES ($1, $2, $3)")
            .bind(&payload.username)
            .bind(&hashed)
            .bind(&payload.email)
            .execute(&state.pool)
            .await;

    match result {
        Ok(_) => (
            StatusCode::OK,
            Json(UserResponse {
                message: "Successfully registered user".to_string(),
            }),
        )
            .into_response(),

        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(UserResponse {
                message: format!("Failed to register user: {}", e),
            }),
        )
            .into_response(),
    }
}

#[debug_handler]
pub async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, UserSql>("SELECT * FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_optional(&state.pool)
        .await;

    match result {
        Ok(Some(user)) => {
            if verify(&payload.password, &user.password).unwrap_or(false) {
                (StatusCode::OK, "Logged in").into_response()
            } else {
                (StatusCode::UNAUTHORIZED, "Login failed").into_response()
            }
        }
        Ok(None) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(err) => {
            eprintln!("DB error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}

/* =========================================================
   ✅ FORGOT PASSWORD (OTP) - Email OTP
   ========================================================= */

#[derive(Serialize)]
pub struct ApiMsg {
    pub message: String,
}

#[derive(Deserialize)]
pub struct ForgotPasswordReq {
    pub email: String,
}

#[derive(Deserialize)]
pub struct ResetPasswordReq {
    pub email: String,
    pub otp: String,
    pub new_password: String,
}

#[derive(FromRow)]
struct UserIdRow {
    id: i32,
    email: String,
}

fn gen_otp_6() -> String {
    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(0..=999_999);
    format!("{:06}", n)
}

fn otp_hash(otp: &str) -> String {
    let mut h = Sha256::new();
    h.update(otp.as_bytes());
    format!("{:x}", h.finalize())
}

#[debug_handler]
pub async fn forgot_password(
    State(state): State<AppState>,
    Json(payload): Json<ForgotPasswordReq>,
) -> impl IntoResponse {
    let email = payload.email.trim().to_lowercase();
    if email.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiMsg {
                message: "Email required".to_string(),
            }),
        )
            .into_response();
    }

    let user = sqlx::query_as::<_, UserIdRow>("SELECT id, email FROM users WHERE LOWER(email) = $1")
        .bind(&email)
        .fetch_optional(&state.pool)
        .await;

    let user = match user {
        Ok(Some(u)) => u,
        Ok(None) => {
            // jangan bocorin email ada/tidak
            return (
                StatusCode::OK,
                Json(ApiMsg {
                    message: "Jika email terdaftar, OTP akan dikirim".to_string(),
                }),
            )
                .into_response();
        }
        Err(e) => {
            eprintln!("DB error forgot_password: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiMsg {
                    message: "DB error".to_string(),
                }),
            )
                .into_response();
        }
    };

    let otp = gen_otp_6();
    let otp_h = otp_hash(&otp);

    // invalidate OTP lama
    let _ = sqlx::query(
        "UPDATE password_reset_otps
         SET used_at = NOW()
         WHERE user_id = $1 AND used_at IS NULL",
    )
    .bind(user.id)
    .execute(&state.pool)
    .await;

    // insert OTP baru (expire 10 menit)
    let ins = sqlx::query(
        "INSERT INTO password_reset_otps (user_id, otp_hash, expires_at)
         VALUES ($1, $2, NOW() + INTERVAL '10 minutes')",
    )
    .bind(user.id)
    .bind(&otp_h)
    .execute(&state.pool)
    .await;

    if let Err(e) = ins {
        eprintln!("Insert OTP error: {:?}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiMsg {
                message: "Failed to create OTP".to_string(),
            }),
        )
            .into_response();
    }

    // ✅ kirim email OTP beneran
    if let Err(e) = send_otp_email(&user.email, &otp).await {
        eprintln!("send_otp_email error: {e}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiMsg {
                message: "Failed to send email".to_string(),
            }),
        )
            .into_response();
    }

    (StatusCode::OK, Json(ApiMsg { message: "OTP sent".to_string() })).into_response()
}

#[derive(FromRow)]
struct OtpRow {
    id: i32,
    otp_hash: String,
    attempts: i32,
}

#[debug_handler]
pub async fn reset_password(
    State(state): State<AppState>,
    Json(payload): Json<ResetPasswordReq>,
) -> impl IntoResponse {
    let email = payload.email.trim().to_lowercase();
    let otp = payload.otp.trim().to_string();
    let new_password = payload.new_password.clone();

    if email.is_empty() || otp.len() != 6 || new_password.len() < 8 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiMsg {
                message: "Invalid payload".to_string(),
            }),
        )
            .into_response();
    }

    let user = sqlx::query_as::<_, UserIdRow>("SELECT id, email FROM users WHERE LOWER(email) = $1")
        .bind(&email)
        .fetch_optional(&state.pool)
        .await;

    let user = match user {
        Ok(Some(u)) => u,
        Ok(None) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiMsg {
                    message: "OTP invalid".to_string(),
                }),
            )
                .into_response();
        }
        Err(e) => {
            eprintln!("DB error reset_password(user): {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiMsg {
                    message: "DB error".to_string(),
                }),
            )
                .into_response();
        }
    };

    // otp terbaru yg belum dipakai & belum expired
    let row = sqlx::query_as::<_, OtpRow>(
        r#"
        SELECT id, otp_hash, attempts
        FROM password_reset_otps
        WHERE user_id = $1
          AND used_at IS NULL
          AND expires_at > NOW()
        ORDER BY id DESC
        LIMIT 1
        "#,
    )
    .bind(user.id)
    .fetch_optional(&state.pool)
    .await;

    let row = match row {
        Ok(Some(r)) => r,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiMsg {
                    message: "OTP invalid".to_string(),
                }),
            )
                .into_response();
        }
    };

    if row.attempts >= 5 {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            Json(ApiMsg {
                message: "Too many attempts".to_string(),
            }),
        )
            .into_response();
    }

    let given_h = otp_hash(&otp);
    if given_h != row.otp_hash {
        let _ = sqlx::query("UPDATE password_reset_otps SET attempts = attempts + 1 WHERE id = $1")
            .bind(row.id)
            .execute(&state.pool)
            .await;

        return (
            StatusCode::BAD_REQUEST,
            Json(ApiMsg {
                message: "OTP invalid".to_string(),
            }),
        )
            .into_response();
    }

    let hashed = hash(&new_password, DEFAULT_COST).unwrap();

    let mut tx = match state.pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("TX begin error: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiMsg {
                    message: "DB error".to_string(),
                }),
            )
                .into_response();
        }
    };

    let used = sqlx::query("UPDATE password_reset_otps SET used_at = NOW() WHERE id = $1")
        .bind(row.id)
        .execute(&mut *tx)
        .await;

    if let Err(e) = used {
        eprintln!("mark used error: {:?}", e);
        let _ = tx.rollback().await;
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiMsg {
                message: "Failed reset password".to_string(),
            }),
        )
            .into_response();
    }

    let upd = sqlx::query("UPDATE users SET password = $1 WHERE id = $2")
        .bind(&hashed)
        .bind(user.id)
        .execute(&mut *tx)
        .await;

    if let Err(e) = upd {
        eprintln!("update password error: {:?}", e);
        let _ = tx.rollback().await;
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiMsg {
                message: "Failed reset password".to_string(),
            }),
        )
            .into_response();
    }

    let _ = tx.commit().await;

    (
        StatusCode::OK,
        Json(ApiMsg {
            message: "Password updated".to_string(),
        }),
    )
        .into_response()
}
