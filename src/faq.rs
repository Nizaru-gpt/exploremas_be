use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::app_state::AppState;

#[derive(Serialize, FromRow)]
pub struct FaqRow {
    pub id: i64,
    pub question: String,
    pub answer: String,
    pub times_used: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct CreateFaqRequest {
    pub question: String,
    pub answer: String,
}

#[derive(Serialize)]
pub struct BasicMessage {
    pub message: String,
}

pub async fn get_faqs(
    State(state): State<AppState>,
) -> Result<Json<Vec<FaqRow>>, (StatusCode, String)> {
    let rows = sqlx::query_as::<_, FaqRow>(
        r#"SELECT id, question, answer, times_used, created_at
           FROM faqs
           ORDER BY id DESC"#,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Error: {e}")))?;

    Ok(Json(rows))
}

pub async fn create_faq(
    State(state): State<AppState>,
    Json(payload): Json<CreateFaqRequest>,
) -> Result<(StatusCode, Json<FaqRow>), (StatusCode, String)> {
    let q = payload.question.trim().to_lowercase();
    let a = payload.answer.trim().to_string();
    if q.is_empty() || a.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "question & answer required".to_string()));
    }

    // optional: prevent duplicates by same question keyword
    let row = sqlx::query_as::<_, FaqRow>(
        r#"
        INSERT INTO faqs (question, answer)
        VALUES ($1, $2)
        RETURNING id, question, answer, times_used, created_at
        "#,
    )
    .bind(q)
    .bind(a)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Error: {e}")))?;

    Ok((StatusCode::CREATED, Json(row)))
}

pub async fn delete_faq(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<BasicMessage>, (StatusCode, String)> {
    let res = sqlx::query("DELETE FROM faqs WHERE id=$1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Error: {e}")))?;

    if res.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "FAQ not found".to_string()));
    }
    Ok(Json(BasicMessage { message: "Deleted".to_string() }))
}

#[derive(Serialize)]
pub struct HitResponse {
    pub id: i64,
    pub times_used: i64,
}

pub async fn hit_faq(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<HitResponse>, (StatusCode, String)> {
    let times_used: i64 = sqlx::query_scalar(
        r#"UPDATE faqs SET times_used = times_used + 1 WHERE id=$1 RETURNING times_used"#,
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB Error: {e}")))?;

    Ok(Json(HitResponse { id, times_used }))
}
