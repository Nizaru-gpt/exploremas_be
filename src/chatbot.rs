use axum::{extract::Path, extract::State, http::StatusCode, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::app_state::AppState;

//
// =========================
// CHAT LOG + STATS
// =========================
//

#[derive(Deserialize)]
pub struct ChatLogRequest {
    pub session_id: String,
    pub question: String,
    pub answer: String,
    pub from_faq: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatStatsResponse {
    pub total_sessions: i64,
    pub total_user_messages: i64,
    pub total_bot_messages: i64,
    pub total_faq_matched: i64,
}

// POST /api/chat/log
pub async fn save_chat_log(
    State(state): State<AppState>,
    Json(payload): Json<ChatLogRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    if payload.session_id.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "session_id is required".to_string()));
    }
    if payload.question.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "question is required".to_string()));
    }
    if payload.answer.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "answer is required".to_string()));
    }

    let result = sqlx::query(
        r#"
        INSERT INTO chat_logs (session_id, user_question, bot_answer, from_faq)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(payload.session_id)
    .bind(payload.question)
    .bind(payload.answer)
    .bind(payload.from_faq)
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("DB Error: {}", e))),
    }
}

// GET /api/chat/stats
pub async fn get_chat_stats(
    State(state): State<AppState>,
) -> Result<Json<ChatStatsResponse>, (StatusCode, String)> {
    let total_user_messages: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM chat_logs")
        .fetch_one(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Error: {}", e)))?;

    let total_bot_messages: i64 = total_user_messages;

    let total_faq_matched: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM chat_logs WHERE from_faq = TRUE")
            .fetch_one(&state.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Error: {}", e)))?;

    let total_sessions: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(DISTINCT session_id)
        FROM chat_logs
        WHERE session_id IS NOT NULL AND session_id <> ''
        "#,
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Error: {}", e)))?;

    Ok(Json(ChatStatsResponse {
        total_sessions,
        total_user_messages,
        total_bot_messages,
        total_faq_matched,
    }))
}

//
// =========================
// FAQ (persist di DB)
// =========================
//

#[derive(FromRow)]
pub struct FaqRow {
    pub id: i64,
    pub question: String,
    pub answer: String,
    pub times_used: i64,
    pub created_at: DateTime<Utc>,
}

// ✅ response khusus FE (camelCase)
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FaqResponse {
    pub id: i64,
    pub question: String,
    pub answer: String,
    pub times_used: i64,
    pub created_at: DateTime<Utc>,
}

impl From<FaqRow> for FaqResponse {
    fn from(row: FaqRow) -> Self {
        Self {
            id: row.id,
            question: row.question,
            answer: row.answer,
            times_used: row.times_used,
            created_at: row.created_at,
        }
    }
}

#[derive(Deserialize)]
pub struct UpsertFaqRequest {
    pub question: String,
    pub answer: String,
}

// GET /api/faqs
pub async fn get_faqs(
    State(state): State<AppState>,
) -> Result<Json<Vec<FaqResponse>>, (StatusCode, String)> {
    let rows = sqlx::query_as::<_, FaqRow>(
        r#"
        SELECT id, question, answer, times_used, created_at
        FROM faqs
        ORDER BY times_used DESC, id DESC
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Error: {}", e)))?;

    let out = rows.into_iter().map(FaqResponse::from).collect();
    Ok(Json(out))
}

// POST /api/faqs  (upsert by unique question)
pub async fn upsert_faq(
    State(state): State<AppState>,
    Json(payload): Json<UpsertFaqRequest>,
) -> Result<Json<FaqResponse>, (StatusCode, String)> {
    let q = payload.question.trim().to_lowercase();
    let a = payload.answer.trim().to_string();

    if q.is_empty() || a.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "question & answer required".to_string()));
    }

    let row = sqlx::query_as::<_, FaqRow>(
        r#"
        INSERT INTO faqs (question, answer)
        VALUES ($1, $2)
        ON CONFLICT (question)
        DO UPDATE SET answer = EXCLUDED.answer
        RETURNING id, question, answer, times_used, created_at
        "#,
    )
    .bind(q)
    .bind(a)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Error: {}", e)))?;

    Ok(Json(FaqResponse::from(row)))
}

// DELETE /api/faqs/{id}
pub async fn delete_faq(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query("DELETE FROM faqs WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Error: {}", e)))?;

    Ok(StatusCode::NO_CONTENT)
}

// POST /api/faqs/{id}/hit  -> times_used++
pub async fn hit_faq(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query("UPDATE faqs SET times_used = times_used + 1 WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Error: {}", e)))?;

    Ok(StatusCode::OK)
}

//
// =========================
// GROQ LLM
// =========================
//

#[derive(Deserialize)]
pub struct LlmRequest {
    pub prompt: String,
}

#[derive(Serialize)]
pub struct LlmResponse {
    pub answer: String,
}

// POST /api/chat/llm
pub async fn llm_answer(Json(payload): Json<LlmRequest>) -> Result<Json<LlmResponse>, (StatusCode, String)> {
    let prompt = payload.prompt.trim().to_string();
    if prompt.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "prompt required".to_string()));
    }

    let api_key = std::env::var("GROQ_API_KEY")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "GROQ_API_KEY not set".to_string()))?;

    // ✅ pakai env model (default aman)
    let model = std::env::var("GROQ_MODEL").unwrap_or_else(|_| "llama-3.1-8b-instant".to_string());

    // Groq OpenAI-compatible endpoint
    let url = "https://api.groq.com/openai/v1/chat/completions";

    let system = r#"
Kamu adalah MasBot, asisten wisata & kuliner khusus Purwokerto/Banyumas.
Jawab ringkas, jelas, dan ramah.
Kalau ditanya di luar Purwokerto/Banyumas, tetap sopan dan arahkan balik ke konteks lokal.
"#;

    let body = serde_json::json!({
        "model": model,
        "temperature": 0.4,
        "max_tokens": 300,
        "messages": [
            { "role": "system", "content": system },
            { "role": "user", "content": prompt }
        ]
    });

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Groq request error: {}", e)))?;

    let status = res.status();
    let text = res
        .text()
        .await
        .unwrap_or_else(|_| "failed to read response body".to_string());

    if !status.is_success() {
        // ✅ balikin status asli biar gampang debug (400/401/404)
        return Err((StatusCode::BAD_GATEWAY, format!("Groq error ({}): {}", status, text)));
    }

    let v: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Groq JSON parse error: {}", e)))?;

    let answer = v["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("Maaf, aku belum bisa jawab itu.")
        .to_string();

    Ok(Json(LlmResponse { answer }))
}
