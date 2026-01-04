use crate::app_state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize)]
pub struct ApiResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,

    // kompatibilitas: kalau FE masih ngirim email, aman (di-ignore)
    pub email: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(FromRow)]
struct AdminSql {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[debug_handler]
pub async fn admin_register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    let hashed = match hash(&payload.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    message: "Failed to hash password".to_string(),
                }),
            )
                .into_response();
        }
    };

    // FIX: table name sesuai schema -> admins (bukan admin)
    let query_result = sqlx::query(r#"INSERT INTO admins (username, password) VALUES ($1, $2)"#)
        .bind(&payload.username)
        .bind(&hashed)
        .execute(&state.pool)
        .await;

    match query_result {
        Ok(_) => (
            StatusCode::CREATED,
            Json(ApiResponse {
                message: "Success create new admin".to_string(),
            }),
        )
            .into_response(),

        Err(err) => {
            eprintln!("DB Insert Error: {:?}", err);
            (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse {
                    message: "Failed create new admin".to_string(),
                }),
            )
                .into_response()
        }
    }
}

pub async fn admin_login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    // FIX: table -> admins + struct -> AdminSql (bukan UserSql)
    let result = sqlx::query_as::<_, AdminSql>(
        r#"SELECT id, username, password FROM admins WHERE username = $1"#,
    )
    .bind(&payload.username)
    .fetch_optional(&state.pool)
    .await;

    match result {
        Ok(Some(admin)) => {
            if verify(&payload.password, &admin.password).unwrap_or(false) {
                (
                    StatusCode::OK,
                    Json(ApiResponse {
                        message: "Logged in".to_string(),
                    }),
                )
                    .into_response()
            } else {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(ApiResponse {
                        message: "Failed to login".to_string(),
                    }),
                )
                    .into_response()
            }
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                message: "Admin not found".to_string(),
            }),
        )
            .into_response(),
        Err(err) => {
            eprintln!("DB error: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    message: "Internal server error".to_string(),
                }),
            )
                .into_response()
        }
    }
}
