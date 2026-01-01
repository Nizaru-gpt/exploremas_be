use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::app_state::AppState;

#[derive(Debug, Serialize, FromRow)]
pub struct Kuliner {
    pub id: i32,
    pub nama_tempat: String,
    pub kategori: String,
    pub alamat: String,
    pub htm: i32,
    pub link_gmaps: String,
    pub link_foto: String,
}

#[derive(Debug, Deserialize)]
pub struct KulinerPayload {
    pub nama_tempat: String,
    pub kategori: String,
    pub alamat: String,
    pub htm: i32,
    pub link_gmaps: String,
    pub link_foto: String,
}

pub async fn get_kuliner(State(state): State<AppState>) -> Result<Json<Vec<Kuliner>>, (StatusCode, String)> {
    let rows = sqlx::query_as::<_, Kuliner>(
        r#"SELECT id, nama_tempat, kategori, alamat, htm, link_gmaps, link_foto FROM kuliner ORDER BY id"#,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {e:?}")))?;

    Ok(Json(rows))
}

pub async fn get_kuliner_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Kuliner>, (StatusCode, String)> {
    let row = sqlx::query_as::<_, Kuliner>(
        r#"SELECT id, nama_tempat, kategori, alamat, htm, link_gmaps, link_foto FROM kuliner WHERE id = $1"#,
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::NOT_FOUND, format!("Not found / DB error: {e:?}")))?;

    Ok(Json(row))
}

pub async fn create_kuliner(
    State(state): State<AppState>,
    Json(payload): Json<KulinerPayload>,
) -> Result<(StatusCode, Json<Kuliner>), (StatusCode, String)> {
    let inserted = sqlx::query_as::<_, Kuliner>(
        r#"
        INSERT INTO kuliner (nama_tempat, kategori, alamat, htm, link_gmaps, link_foto)
        VALUES ($1,$2,$3,$4,$5,$6)
        RETURNING id, nama_tempat, kategori, alamat, htm, link_gmaps, link_foto
        "#,
    )
    .bind(payload.nama_tempat)
    .bind(payload.kategori)
    .bind(payload.alamat)
    .bind(payload.htm)
    .bind(payload.link_gmaps)
    .bind(payload.link_foto)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {e:?}")))?;

    Ok((StatusCode::CREATED, Json(inserted)))
}

// --- BARU: UPDATE KULINER ---
pub async fn update_kuliner(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<KulinerPayload>,
) -> Result<Json<String>, (StatusCode, String)> {
    let result = sqlx::query(
        r#"
        UPDATE kuliner 
        SET nama_tempat=$1, kategori=$2, alamat=$3, htm=$4, link_gmaps=$5, link_foto=$6 
        WHERE id=$7
        "#
    )
    .bind(payload.nama_tempat)
    .bind(payload.kategori)
    .bind(payload.alamat)
    .bind(payload.htm)
    .bind(payload.link_gmaps)
    .bind(payload.link_foto)
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Update Error: {e:?}")))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "ID tidak ditemukan".to_string()));
    }

    Ok(Json("Update Berhasil".to_string()))
}

// --- BARU: DELETE KULINER ---
pub async fn delete_kuliner(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<String>, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM kuliner WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Delete Error: {e:?}")))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "ID tidak ditemukan".to_string()));
    }

    Ok(Json("Delete Berhasil".to_string()))
}