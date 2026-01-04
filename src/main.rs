use axum::{
    http::Method,
    routing::{delete, get, post, put},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

// --- DAFTAR MODUL ---
mod admin;
mod app_state;
mod kuliner;
mod tempat_nongkrong;
mod user;
mod wisata_alam;
mod wisata_pendidikan;
mod chatbot;
mod news;
mod upload; // ✅ TAMBAH INI

use crate::app_state::AppState;

// ADMIN + USER HANDLERS
use crate::admin::{admin_login_handler, admin_register_handler};
use crate::user::{login_user, register_user};

// WISATA ALAM HANDLERS
use crate::wisata_alam::{
    create_wisata, delete_wisata_alam, get_wisata_alam, get_wisata_alam_by_id, update_wisata_alam,
};

// WISATA PENDIDIKAN HANDLERS
use crate::wisata_pendidikan::{
    create_wisata_pendidikan, get_wisata_pendidikan, get_wisata_pendidikan_by_id,
    update_wisata_pendidikan, delete_wisata_pendidikan
};

// KULINER HANDLERS
use crate::kuliner::{create_kuliner, delete_kuliner, get_kuliner, get_kuliner_id, update_kuliner};

// TEMPAT NONGKRONG HANDLERS
use crate::tempat_nongkrong::{
    create_tempat_nongkrong, delete_tempat_nongkrong, get_tempat_nongkrong, get_tempat_nongkrong_id,
    update_tempat_nongkrong,
};

// CHATBOT HANDLERS
use crate::chatbot::{save_chat_log, get_chat_stats};

// NEWS HANDLERS
use crate::news::{get_all_news, add_news, delete_news};

// ✅ UPLOAD HANDLER
use crate::upload::upload_handler;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // bikin folder uploads kalau belum ada
    let _ = std::fs::create_dir_all("uploads");

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    let pool = sqlx::postgres::PgPool::connect(&db_url)
        .await
        .expect("Failed to create postgre database pool");

    let state = AppState { pool };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        // ===== AUTH USER =====
        .route("/register", post(register_user))
        .route("/login", post(login_user))

        // ===== AUTH ADMIN =====
        .route("/admin_register", post(admin_register_handler))
        .route("/admin_login", post(admin_login_handler))

        // ✅ UPLOAD (multipart) -> balikin { url }
        .route("/api/upload", post(upload_handler))

        // ✅ STATIC FILES: biar URL http://localhost:7860/uploads/<file> bisa diakses
        .nest_service("/uploads", ServeDir::new("uploads"))

        // =========================================================
        // REST CLEAN ROUTES
        // =========================================================

        // WISATA ALAM
        .route("/wisata_alam", get(get_wisata_alam).post(create_wisata))
        .route(
            "/wisata_alam/{id}",
            get(get_wisata_alam_by_id)
                .put(update_wisata_alam)
                .delete(delete_wisata_alam),
        )

        // WISATA PENDIDIKAN
        .route("/wisata_pendidikan", get(get_wisata_pendidikan).post(create_wisata_pendidikan))
        .route(
            "/wisata_pendidikan/{id}",
            get(get_wisata_pendidikan_by_id)
                .put(update_wisata_pendidikan)
                .delete(delete_wisata_pendidikan),
        )

        // KULINER
        .route("/kuliner", get(get_kuliner).post(create_kuliner))
        .route(
            "/kuliner/{id}",
            get(get_kuliner_id)
                .put(update_kuliner)
                .delete(delete_kuliner),
        )

        // TEMPAT NONGKRONG
        .route("/tempat_nongkrong", get(get_tempat_nongkrong).post(create_tempat_nongkrong))
        .route(
            "/tempat_nongkrong/{id}",
            get(get_tempat_nongkrong_id)
                .put(update_tempat_nongkrong)
                .delete(delete_tempat_nongkrong),
        )

        // CHATBOT
        .route("/api/chat/log", post(save_chat_log))
        .route("/api/chat/stats", get(get_chat_stats))

        // NEWS
        .route("/api/news", get(get_all_news).post(add_news))
        .route("/api/news/{id}", delete(delete_news))

        // =========================================================
        // BACKWARD COMPAT ROUTES
        // =========================================================
        .route("/api/add_wisata", post(create_wisata))
        .route("/add_wisata", post(create_wisata))
        .route("/api/update_wisata/{id}", put(update_wisata_alam))
        .route("/api/delete_wisata/{id}", delete(delete_wisata_alam))

        .route("/add_wisata_pendidikan", post(create_wisata_pendidikan))
        .route("/api/update_wisata_pendidikan/{id}", put(update_wisata_pendidikan))
        .route("/api/delete_wisata_pendidikan/{id}", delete(delete_wisata_pendidikan))

        .route("/get_kuliner", get(get_kuliner))
        .route("/api/add_kuliner", post(create_kuliner))
        .route("/add_kuliner", post(create_kuliner))
        .route("/api/update_kuliner/{id}", put(update_kuliner))
        .route("/api/delete_kuliner/{id}", delete(delete_kuliner))

        .route("/api/add_tempat_nongkrong", post(create_tempat_nongkrong))
        .route("/add_tempat_nongkrong", post(create_tempat_nongkrong))
        .route("/api/update_cafe/{id}", put(update_tempat_nongkrong))
        .route("/api/delete_cafe/{id}", delete(delete_tempat_nongkrong))

        .with_state(state)
        .layer(cors);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(7860);

    println!("Server is running on port {}", port);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
