use crate::app_state::AppState;
use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use bcrypt::{hash, DEFAULT_COST};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::FromRow;
use std::env;
use time::{Duration, OffsetDateTime};

use lettre::{
    message::Mailbox,
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

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
struct UserRow {
    id: i32,
    email: String,
}

fn otp_hash(otp: &str) -> String {
    let mut h = Sha256::new();
    h.update(otp.as_bytes());
    format!("{:x}", h.finalize())
}

fn gen_otp_6() -> String {
    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(0..=999_999);
    format!("{:06}", n)
}

async fn send_otp_email(to_email: &str, otp: &str) -> Result<(), String> {
    // ===== env SMTP =====
    // SMTP_HOST, SMTP_PORT, SMTP_USERNAME, SMTP_PASSWORD, SMTP_FROM_EMAIL, SMTP_FROM_NAME
    let host = env::var("SMTP_HOST").map_err(|_| "SMTP_HOST not set")?;
    let port: u16 = env::var("SMTP_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(587);

    let username = env::var("SMTP_USERNAME").map_err(|_| "SMTP_USERNAME not set")?;
    let password = env::var("SMTP_PASSWORD").map_err(|_| "SMTP_PASSWORD not set")?;
    let from_email = env::var("SMTP_FROM_EMAIL").map_err(|_| "SMTP_FROM_EMAIL not set")?;
    let from_name = env::var("SMTP_FROM_NAME").unwrap_or("ExploreMas".to_string());

    let from: Mailbox = format!("{} <{}>", from_name, from_email)
        .parse()
        .map_err(|e| format!("bad from mailbox: {e}"))?;

    let to: Mailbox = to_email
        .parse()
        .map_err(|e| format!("bad to mailbox: {e}"))?;

    let subject = "Kode OTP Reset Password - ExploreMas";

    // body singkat (real)
    hint::body_style();
    let body = format!(
        "Halo!\n\nKode OTP reset password kamu adalah: {otp}\n\nBerlaku 10 menit. Jangan bagikan kode ini ke siapa pun.\n\n- ExploreMas"
    );

    let email = Message::builder()
        .from(from)
        .to(to)
        .subject(subject)
        .body(body)
        .map_err(|e| format!("build email error: {e}"))?;

    let creds = Credentials::new(username, password);

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&host)
        .map_err(|e| format!("smtp relay error: {e}"))?
        .port(port)
        .credentials(creds)
        .build();

    mailer
        .send(email)
        .await
        .map_err(|e| format!("send email error: {e}"))?;

    Ok(())
}

// ====== REQUEST OTP ======
#[debug_handler]
pub async fn forgot_password(
    State(state): State<AppState>,
    Json(payload): Json<ForgotPasswordReq>,
) -> impl IntoResponse {
    let email = payload.email.trim().to_lowercase();
    if email.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(ApiMsg { message: "Email required".to_string() })).into_response();
    }

    // cari user berdasarkan email
    let user = sqlx::query_as::<_, UserRow>("SELECT id, email FROM users WHERE LOWER(email) = $1")
        .bind(&email)
        .fetch_optional(&state.pool)
        .await;

    let user = match user {
        Ok(Some(u)) => u,
        Ok(None) => {
            // biar aman (jangan bocorin email ada/tidak), tetap balikin OK
            return (StatusCode::OK, Json(ApiMsg { message: "Jika email terdaftar, OTP akan dikirim".to_string() })).into_response();
        }
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiMsg { message: "DB error".to_string() })).into_response(),
    };

    // generate OTP
    let otp = gen_otp_6();
    let otp_h = otp_hash(&otp);

    // invalidate OTP lama yang belum dipakai (opsional, biar rapi)
    let _ = sqlx::query("UPDATE password_reset_otps SET used_at = NOW() WHERE user_id = $1 AND used_at IS NULL")
        .bind(user.id)
        .execute(&state.pool)
        .await;

    let expires_at = OffsetDateTime::now_utc() + Duration::minutes(10);

    let ins = sqlx::query(
        "INSERT INTO password_reset_otps (user_id, otp_hash, expires_at) VALUES ($1, $2, $3)"
    )
        .bind(user.id)
        .bind(&otp_h)
        .bind(expires_at)
        .execute(&state.pool)
        .await;

    if ins.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiMsg { message: "Failed to create OTP".to_string() })).into_response();
    }

    // kirim email OTP real
    if let Err(e) = send_otp_email(&user.email, &otp).await {
        eprintln!("send_otp_email error: {e}");
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiMsg { message: "Failed to send email".to_string() })).into_response();
    }

    (StatusCode::OK, Json(ApiMsg { message: "OTP sent".to_string() })).into_response()
}

// ====== VERIFY OTP + RESET PASSWORD ======
#[debug_handler]
pub async fn reset_password(
    State(state): State<AppState>,
    Json(payload): Json<ResetPasswordReq>,
) -> impl IntoResponse {
    let email = payload.email.trim().to_lowercase();
    let otp = payload.otp.trim().to_string();
    let new_password = payload.new_password.clone();

    if email.is_empty() || otp.len() != 6 || new_password.len() < 8 {
        return (StatusCode::BAD_REQUEST, Json(ApiMsg { message: "Invalid payload".to_string() })).into_response();
    }

    let user = sqlx::query_as::<_, UserRow>("SELECT id, email FROM users WHERE LOWER(email) = $1")
        .bind(&email)
        .fetch_optional(&state.pool)
        .await;

    let user = match user {
        Ok(Some(u)) => u,
        Ok(None) => return (StatusCode::BAD_REQUEST, Json(ApiMsg { message: "OTP invalid".to_string() })).into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiMsg { message: "DB error".to_string() })).into_response(),
    };

    // ambil otp terbaru yang belum dipakai
    #[derive(FromRow)]
    struct OtpRow {
        id: i32,
        otp_hash: String,
        expires_at: time::OffsetDateTime,
        attempts: i32,
    }

    let row = sqlx::query_as::<_, OtpRow>(
        r#"
        SELECT id, otp_hash, expires_at, attempts
        FROM password_reset_otps
        WHERE user_id = $1 AND used_at IS NULL
        ORDER BY id DESC
        LIMIT 1
        "#
    )
        .bind(user.id)
        .fetch_optional(&state.pool)
        .await;

    let row = match row {
        Ok(Some(r)) => r,
        _ => return (StatusCode::BAD_REQUEST, Json(ApiMsg { message: "OTP invalid".to_string() })).into_response(),
    };

    // cek expire
    if row.expires_at < OffsetDateTime::now_utc() {
        let _ = sqlx::query("UPDATE password_reset_otps SET used_at = NOW() WHERE id = $1")
            .bind(row.id)
            .execute(&state.pool)
            .await;

        return (StatusCode::BAD_REQUEST, Json(ApiMsg { message: "OTP expired".to_string() })).into_response();
    }

    // batas attempts
    if row.attempts >= 5 {
        return (StatusCode::TOO_MANY_REQUESTS, Json(ApiMsg { message: "Too many attempts".to_string() })).into_response();
    }

    let given_h = otp_hash(&otp);
    if given_h != row.otp_hash {
        let _ = sqlx::query("UPDATE password_reset_otps SET attempts = attempts + 1 WHERE id = $1")
            .bind(row.id)
            .execute(&state.pool)
            .await;

        return (StatusCode::BAD_REQUEST, Json(ApiMsg { message: "OTP invalid".to_string() })).into_response();
    }

    // OTP valid -> mark used, update password
    let hashed = hash(&new_password, DEFAULT_COST).unwrap();

    let mut tx = match state.pool.begin().await {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiMsg { message: "DB error".to_string() })).into_response(),
    };

    let _ = sqlx::query("UPDATE password_reset_otps SET used_at = NOW() WHERE id = $1")
        .bind(row.id)
        .execute(&mut *tx)
        .await;

    let upd = sqlx::query("UPDATE users SET password = $1 WHERE id = $2")
        .bind(&hashed)
        .bind(user.id)
        .execute(&mut *tx)
        .await;

    if upd.is_err() {
        let _ = tx.rollback().await;
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiMsg { message: "Failed reset password".to_string() })).into_response();
    }

    let _ = tx.commit().await;

    (StatusCode::OK, Json(ApiMsg { message: "Password updated".to_string() })).into_response()
}

// tiny trick biar compiler gak ngeluh kalau hint gak kepake (opsional)
mod hint {
    pub fn body_style() {}
}
