use crate::app_state::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Struct Input (Create)
#[derive(Deserialize)]
pub struct WisataSql {
    name: String,
    category: String,
    address: String,
    open: String,
    close: String,
    htm: i32,
    gmaps: String,
    pictures: String,
}

// Struct Output (Response) - SUDAH DITAMBAH ID
#[derive(Serialize, FromRow)]
pub struct WisataResponseModel {
    pub id: i32,
    pub nama_tempat: String,
    pub kategori: String,
    pub alamat: String,
    pub jam_buka: String,
    pub jam_tutup: String,
    pub htm: i32,
    pub link_gmaps: String,
    pub link_foto: String,
}

#[derive(Serialize)]
pub struct WisataResponse {
    pub message: String,
}

#[debug_handler]
pub async fn create_wisata_pendidikan(
    State(state): State<AppState>,
    Json(payload): Json<WisataSql>,
) -> impl IntoResponse {
    let result = sqlx::query(
        "insert into wisata_pendidikan(nama_tempat, kategori, alamat, jam_buka, jam_tutup, htm, link_gmaps, link_foto)
        values ($1, $2, $3, $4, $5, $6, $7, $8)")
        .bind(&payload.name)
        .bind(&payload.category)
        .bind(&payload.address)
        .bind(&payload.open)
        .bind(&payload.close)
        .bind(&payload.htm)
        .bind(&payload.gmaps)
        .bind(&payload.pictures)
        .execute(&state.pool)
        .await;

    match result {
        Ok(_) => (
            StatusCode::OK,
            Json(WisataResponse {
                message: "Wisata created".to_string(),
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(WisataResponse {
                message: format!("error: {}", e),
            }),
        ),
    }
}

// PERBAIKAN: fetch_one -> fetch_all
#[debug_handler]
pub async fn get_wisata_pendidikan(State(state): State<AppState>) -> impl IntoResponse {
    let result = sqlx::query_as::<_, WisataResponseModel>("select * from wisata_pendidikan")
        .fetch_all(&state.pool) // GANTI fetch_one JADI fetch_all
        .await;

    match result {
        Ok(data) => Json(data).into_response(),
        Err(err) => {
            eprintln!("Db error get_wisata_pendidikan: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_wisata_pendidikan_by_id(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    let result = sqlx::query_as::<_, WisataResponseModel>(
        "SELECT * FROM wisata_pendidikan WHERE id = $1"
    ).bind(id).fetch_optional(&state.pool).await;

    match result {
        Ok(Some(data)) => Json(data).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Not found").into_response(),
        Err(err) => {
            eprintln!("DB error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}