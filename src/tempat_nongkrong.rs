// src/tempat_nongkrong.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::app_state::AppState;

#[derive(Debug, Serialize, FromRow)]
pub struct TempatNongkrong {
    pub id: i32,
    pub nama_tempat: String,
    pub kategori: String,
    pub alamat: String,
    pub jam_buka: String,
    pub jam_tutup: String,

    // ✅ range baru (disimpan di DB)
    pub htm_min: i32,
    pub htm_max: i32,

    // ✅ tetap ada untuk kompatibilitas (avg)
    pub htm: i32,

    pub link_gmaps: String,
    pub link_foto: String,

    pub deskripsi: Option<String>,
    pub fasilitas: Option<Vec<String>>,
    pub menu_populer: Option<Vec<String>>,
    pub cocok_untuk: Option<Vec<String>>,

    pub trans_kode: Option<String>,
    pub trans_jarak_meter: Option<i32>,
    pub trans_tarif_min: Option<i32>,
    pub trans_tarif_max: Option<i32>,
    pub trans_rute: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct TempatNongkrongPayload {
    #[serde(alias = "name")]
    pub nama_tempat: String,
    #[serde(alias = "category")]
    pub kategori: String,
    #[serde(alias = "address")]
    pub alamat: String,
    #[serde(alias = "open")]
    pub jam_buka: String,
    #[serde(alias = "close")]
    pub jam_tutup: String,

    // ✅ input lama (opsional) biar request lama tetap jalan
    pub htm: Option<i32>,

    // ✅ input baru (range)
    pub htm_min: Option<i32>,
    pub htm_max: Option<i32>,

    #[serde(alias = "gmaps")]
    pub link_gmaps: String,
    #[serde(alias = "pictures")]
    pub link_foto: String,

    pub deskripsi: Option<String>,
    pub fasilitas: Option<Vec<String>>,
    pub menu_populer: Option<Vec<String>>,
    pub cocok_untuk: Option<Vec<String>>,

    pub trans_kode: Option<String>,
    pub trans_jarak_meter: Option<i32>,
    pub trans_tarif_min: Option<i32>,
    pub trans_tarif_max: Option<i32>,
    pub trans_rute: Option<Vec<String>>,
}

fn resolve_htm_range(p: &TempatNongkrongPayload) -> (i32, i32, i32) {
    // Prioritas: pakai htm_min/max kalau ada.
    // Kalau tidak ada, pakai htm (lama) untuk min=max.
    let min = p.htm_min.or(p.htm).unwrap_or(0);
    let max = p.htm_max.or(p.htm).unwrap_or(min);

    // normalisasi kalau kebalik
    let (mn, mx) = if min <= max { (min, max) } else { (max, min) };

    // avg (buat kolom lama / output htm)
    let avg = ((mn as i64 + mx as i64) / 2) as i32;
    (mn, mx, avg)
}

pub async fn get_tempat_nongkrong(
    State(state): State<AppState>,
) -> Result<Json<Vec<TempatNongkrong>>, (StatusCode, String)> {
    let rows = sqlx::query_as::<_, TempatNongkrong>(
        r#"
        SELECT
            id,
            nama_tempat,
            kategori,
            alamat,
            jam_buka,
            jam_tutup,

            COALESCE(htm_min, harga_rata_rata, 0) AS htm_min,
            COALESCE(htm_max, harga_rata_rata, 0) AS htm_max,
            COALESCE(ROUND((COALESCE(htm_min, harga_rata_rata, 0) + COALESCE(htm_max, harga_rata_rata, 0)) / 2.0)::int, harga_rata_rata, 0) AS htm,

            link_gmaps,
            link_foto,
            deskripsi,
            fasilitas,
            menu_populer,
            cocok_untuk,
            trans_kode,
            trans_jarak_meter,
            trans_tarif_min,
            trans_tarif_max,
            trans_rute
        FROM tempat_nongkrong
        ORDER BY id
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {e:?}")))?;

    Ok(Json(rows))
}

pub async fn get_tempat_nongkrong_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<TempatNongkrong>, (StatusCode, String)> {
    let row = sqlx::query_as::<_, TempatNongkrong>(
        r#"
        SELECT
            id,
            nama_tempat,
            kategori,
            alamat,
            jam_buka,
            jam_tutup,

            COALESCE(htm_min, harga_rata_rata, 0) AS htm_min,
            COALESCE(htm_max, harga_rata_rata, 0) AS htm_max,
            COALESCE(ROUND((COALESCE(htm_min, harga_rata_rata, 0) + COALESCE(htm_max, harga_rata_rata, 0)) / 2.0)::int, harga_rata_rata, 0) AS htm,

            link_gmaps,
            link_foto,
            deskripsi,
            fasilitas,
            menu_populer,
            cocok_untuk,
            trans_kode,
            trans_jarak_meter,
            trans_tarif_min,
            trans_tarif_max,
            trans_rute
        FROM tempat_nongkrong
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::NOT_FOUND, format!("Not found / DB error: {e:?}")))?;

    Ok(Json(row))
}

pub async fn create_tempat_nongkrong(
    State(state): State<AppState>,
    Json(payload): Json<TempatNongkrongPayload>,
) -> Result<(StatusCode, Json<TempatNongkrong>), (StatusCode, String)> {
    let (mn, mx, avg) = resolve_htm_range(&payload);

    let inserted = sqlx::query_as::<_, TempatNongkrong>(
        r#"
        INSERT INTO tempat_nongkrong (
            nama_tempat, kategori, alamat,
            jam_buka, jam_tutup,

            harga_rata_rata,
            htm_min,
            htm_max,

            link_gmaps, link_foto,
            deskripsi, fasilitas, menu_populer, cocok_untuk,
            trans_kode, trans_jarak_meter, trans_tarif_min, trans_tarif_max, trans_rute
        )
        VALUES (
            $1,$2,$3,
            $4,$5,

            $6,
            $7,
            $8,

            $9,$10,
            $11,$12,$13,$14,
            $15,$16,$17,$18,$19
        )
        RETURNING
            id,
            nama_tempat,
            kategori,
            alamat,
            jam_buka,
            jam_tutup,

            COALESCE(htm_min, harga_rata_rata, 0) AS htm_min,
            COALESCE(htm_max, harga_rata_rata, 0) AS htm_max,
            COALESCE(ROUND((COALESCE(htm_min, harga_rata_rata, 0) + COALESCE(htm_max, harga_rata_rata, 0)) / 2.0)::int, harga_rata_rata, 0) AS htm,

            link_gmaps,
            link_foto,
            deskripsi,
            fasilitas,
            menu_populer,
            cocok_untuk,
            trans_kode,
            trans_jarak_meter,
            trans_tarif_min,
            trans_tarif_max,
            trans_rute
        "#,
    )
    .bind(payload.nama_tempat)
    .bind(payload.kategori)
    .bind(payload.alamat)
    .bind(payload.jam_buka)
    .bind(payload.jam_tutup)
    .bind(avg) // harga_rata_rata (legacy)
    .bind(mn)  // htm_min
    .bind(mx)  // htm_max
    .bind(payload.link_gmaps)
    .bind(payload.link_foto)
    .bind(payload.deskripsi)
    .bind(payload.fasilitas)
    .bind(payload.menu_populer)
    .bind(payload.cocok_untuk)
    .bind(payload.trans_kode)
    .bind(payload.trans_jarak_meter)
    .bind(payload.trans_tarif_min)
    .bind(payload.trans_tarif_max)
    .bind(payload.trans_rute)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {e:?}")))?;

    Ok((StatusCode::CREATED, Json(inserted)))
}

pub async fn update_tempat_nongkrong(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<TempatNongkrongPayload>,
) -> Result<Json<String>, (StatusCode, String)> {
    let (mn, mx, avg) = resolve_htm_range(&payload);

    let result = sqlx::query(
        r#"
        UPDATE tempat_nongkrong
        SET
            nama_tempat = $1,
            kategori = $2,
            alamat = $3,
            jam_buka = $4,
            jam_tutup = $5,

            harga_rata_rata = $6,
            htm_min = $7,
            htm_max = $8,

            link_gmaps = $9,
            link_foto = $10,
            deskripsi = $11,
            fasilitas = $12,
            menu_populer = $13,
            cocok_untuk = $14,
            trans_kode = $15,
            trans_jarak_meter = $16,
            trans_tarif_min = $17,
            trans_tarif_max = $18,
            trans_rute = $19
        WHERE id = $20
        "#,
    )
    .bind(payload.nama_tempat)
    .bind(payload.kategori)
    .bind(payload.alamat)
    .bind(payload.jam_buka)
    .bind(payload.jam_tutup)
    .bind(avg)
    .bind(mn)
    .bind(mx)
    .bind(payload.link_gmaps)
    .bind(payload.link_foto)
    .bind(payload.deskripsi)
    .bind(payload.fasilitas)
    .bind(payload.menu_populer)
    .bind(payload.cocok_untuk)
    .bind(payload.trans_kode)
    .bind(payload.trans_jarak_meter)
    .bind(payload.trans_tarif_min)
    .bind(payload.trans_tarif_max)
    .bind(payload.trans_rute)
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Update Error: {e:?}")))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "ID tidak ditemukan".to_string()));
    }

    Ok(Json("Update Berhasil".to_string()))
}

pub async fn delete_tempat_nongkrong(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<String>, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM tempat_nongkrong WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Delete Error: {e:?}")))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "ID tidak ditemukan".to_string()));
    }

    Ok(Json("Delete Berhasil".to_string()))
}
