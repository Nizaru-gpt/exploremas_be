use axum::{
    extract::Multipart,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct UploadResponse {
    pub url: String,
}

pub async fn upload_handler(mut multipart: Multipart) -> impl IntoResponse {
    // cari field "file"
    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        if name != "file" {
            continue;
        }

        let file_name = field.file_name().unwrap_or("upload.bin").to_string();
        let ext = std::path::Path::new(&file_name)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("jpg");

        let new_name = format!("{}.{}", Uuid::new_v4(), ext);
        let save_path = format!("uploads/{}", new_name);

        let data = match field.bytes().await {
            Ok(b) => b,
            Err(_) => return (StatusCode::BAD_REQUEST, "Gagal baca file").into_response(),
        };

        if let Err(_) = tokio::fs::write(&save_path, data).await {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Gagal simpan file").into_response();
        }

        // URL yang akan dipakai FE untuk <img src="...">
        let url = format!("http://localhost:7860/uploads/{}", new_name);

        return (StatusCode::OK, Json(UploadResponse { url })).into_response();
    }

    (StatusCode::BAD_REQUEST, "Field 'file' tidak ditemukan").into_response()
}
