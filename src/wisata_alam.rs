// src/wisata_alam.rs
use crate::app_state::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct WisataPayload {
    // support payload lama:
    // name/category/address/open/close/gmaps/pictures
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

    pub htm: i32,

    #[serde(alias = "gmaps")]
    pub link_gmaps: String,

    #[serde(alias = "pictures")]
    pub link_foto: String,

    // tambahan UI (nullable)
    pub deskripsi: Option<String>,
    pub fasilitas: Option<Vec<String>>,
    pub cocok_untuk: Option<Vec<String>>,

    // trans (nullable)
    pub trans_kode: Option<String>,
    pub trans_jarak_meter: Option<i32>,
    pub trans_tarif_min: Option<i32>,
    pub trans_tarif_max: Option<i32>,
    pub trans_rute: Option<Vec<String>>,
}

#[derive(Debug, Serialize, FromRow)]
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

    pub deskripsi: Option<String>,
    pub fasilitas: Option<Vec<String>>,
    pub cocok_untuk: Option<Vec<String>>,

    pub trans_kode: Option<String>,
    pub trans_jarak_meter: Option<i32>,
    pub trans_tarif_min: Option<i32>,
    pub trans_tarif_max: Option<i32>,
    pub trans_rute: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct WisataResponse {
    pub message: String,
}

#[debug_handler]
pub async fn create_wisata(
    State(state): State<AppState>,
    Json(payload): Json<WisataPayload>,
) -> impl IntoResponse {
    let result = sqlx::query(
        r#"
        INSERT INTO wisata_alam (
            nama_tempat, kategori, alamat, jam_buka, jam_tutup, htm, link_gmaps, link_foto,
            deskripsi, fasilitas, cocok_untuk,
            trans_kode, trans_jarak_meter, trans_tarif_min, trans_tarif_max, trans_rute
        )
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16)
        "#,
    )
    .bind(&payload.nama_tempat)
    .bind(&payload.kategori)
    .bind(&payload.alamat)
    .bind(&payload.jam_buka)
    .bind(&payload.jam_tutup)
    .bind(payload.htm)
    .bind(&payload.link_gmaps)
    .bind(&payload.link_foto)
    .bind(&payload.deskripsi)
    .bind(&payload.fasilitas)
    .bind(&payload.cocok_untuk)
    .bind(&payload.trans_kode)
    .bind(payload.trans_jarak_meter)
    .bind(payload.trans_tarif_min)
    .bind(payload.trans_tarif_max)
    .bind(&payload.trans_rute)
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

#[debug_handler]
pub async fn get_wisata_alam(State(state): State<AppState>) -> impl IntoResponse {
    let result = sqlx::query_as::<_, WisataResponseModel>(
        r#"
        SELECT
            id, nama_tempat, kategori, alamat, jam_buka, jam_tutup, htm, link_gmaps, link_foto,
            deskripsi, fasilitas, cocok_untuk,
            trans_kode, trans_jarak_meter, trans_tarif_min, trans_tarif_max, trans_rute
        FROM wisata_alam
        ORDER BY id
        "#,
    )
    .fetch_all(&state.pool)
    .await;

    match result {
        Ok(data) => Json(data).into_response(),
        Err(err) => {
            eprintln!("Db error get_wisata_alam: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_wisata_alam_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, WisataResponseModel>(
        r#"
        SELECT
            id, nama_tempat, kategori, alamat, jam_buka, jam_tutup, htm, link_gmaps, link_foto,
            deskripsi, fasilitas, cocok_untuk,
            trans_kode, trans_jarak_meter, trans_tarif_min, trans_tarif_max, trans_rute
        FROM wisata_alam
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await;

    match result {
        Ok(Some(data)) => Json(data).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Not found").into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Error: {:?}", err)).into_response(),
    }
}

pub async fn update_wisata_alam(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<WisataPayload>,
) -> impl IntoResponse {
    let result = sqlx::query(
        r#"
        UPDATE wisata_alam
        SET
            nama_tempat=$1, kategori=$2, alamat=$3, jam_buka=$4, jam_tutup=$5, htm=$6, link_gmaps=$7, link_foto=$8,
            deskripsi=$9, fasilitas=$10, cocok_untuk=$11,
            trans_kode=$12, trans_jarak_meter=$13, trans_tarif_min=$14, trans_tarif_max=$15, trans_rute=$16
        WHERE id=$17
        "#,
    )
    .bind(&payload.nama_tempat)
    .bind(&payload.kategori)
    .bind(&payload.alamat)
    .bind(&payload.jam_buka)
    .bind(&payload.jam_tutup)
    .bind(payload.htm)
    .bind(&payload.link_gmaps)
    .bind(&payload.link_foto)
    .bind(&payload.deskripsi)
    .bind(&payload.fasilitas)
    .bind(&payload.cocok_untuk)
    .bind(&payload.trans_kode)
    .bind(payload.trans_jarak_meter)
    .bind(payload.trans_tarif_min)
    .bind(payload.trans_tarif_max)
    .bind(&payload.trans_rute)
    .bind(id)
    .execute(&state.pool)
    .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                (StatusCode::NOT_FOUND, Json(WisataResponse { message: "ID Not Found".to_string() }))
            } else {
                (StatusCode::OK, Json(WisataResponse { message: "Updated successfully".to_string() }))
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(WisataResponse { message: format!("Error: {}", e) }),
        ),
    }
}

pub async fn delete_wisata_alam(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let result = sqlx::query("DELETE FROM wisata_alam WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                (StatusCode::NOT_FOUND, Json(WisataResponse { message: "ID Not Found".to_string() }))
            } else {
                (StatusCode::OK, Json(WisataResponse { message: "Deleted successfully".to_string() }))
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(WisataResponse { message: format!("Error: {}", e) }),
        ),
    }
}
