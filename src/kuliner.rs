// src/kuliner.rs
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
    // TAMBAHAN: tags
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct KulinerPayload {
    pub nama_tempat: String,
    pub kategori: String,
    pub alamat: String,
    pub htm: i32,
    pub link_gmaps: String,
    pub link_foto: String,
    // TAMBAHAN: tags (opsional dari frontend)
    pub tags: Option<Vec<String>>, 
    // Field 'deskripsi' dikirim frontend tapi tidak disimpan di tabel kuliner saat ini, 
    // jadi kita abaikan atau tambahkan jika tabelnya punya. 
    // Jika tabel tidak punya kolom deskripsi, serde akan mengabaikannya (default) atau error (jika deny_unknown_fields).
    // Untuk aman, kita bisa tambahkan Option<String> jika mau menangkapnya, tapi query SQL di bawah tidak menyimpannya.
}

pub async fn get_kuliner(State(state): State<AppState>) -> Result<Json<Vec<Kuliner>>, (StatusCode, String)> {
    let rows = sqlx::query_as::<_, Kuliner>(
        r#"SELECT id, nama_tempat, kategori, alamat, htm, link_gmaps, link_foto, tags FROM kuliner ORDER BY id"#,
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
        r#"SELECT id, nama_tempat, kategori, alamat, htm, link_gmaps, link_foto, tags FROM kuliner WHERE id = $1"#,
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
    // INSERT tags juga
    let inserted = sqlx::query_as::<_, Kuliner>(
        r#"
        INSERT INTO kuliner (nama_tempat, kategori, alamat, htm, link_gmaps, link_foto, tags)
        VALUES ($1,$2,$3,$4,$5,$6,$7)
        RETURNING id, nama_tempat, kategori, alamat, htm, link_gmaps, link_foto, tags
        "#,
    )
    .bind(payload.nama_tempat)
    .bind(payload.kategori)
    .bind(payload.alamat)
    .bind(payload.htm)
    .bind(payload.link_gmaps)
    .bind(payload.link_foto)
    .bind(payload.tags) // Bind array tags
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {e:?}")))?;

    Ok((StatusCode::CREATED, Json(inserted)))
}

pub async fn update_kuliner(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<KulinerPayload>,
) -> Result<Json<String>, (StatusCode, String)> {
    // UPDATE tags juga
    let result = sqlx::query(
        r#"
        UPDATE kuliner 
        SET nama_tempat=$1, kategori=$2, alamat=$3, htm=$4, link_gmaps=$5, link_foto=$6, tags=$7
        WHERE id=$8
        "#
    )
    .bind(payload.nama_tempat)
    .bind(payload.kategori)
    .bind(payload.alamat)
    .bind(payload.htm)
    .bind(payload.link_gmaps)
    .bind(payload.link_foto)
    .bind(payload.tags)
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Update Error: {e:?}")))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "ID tidak ditemukan".to_string()));
    }

    Ok(Json("Update Berhasil".to_string()))
}

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