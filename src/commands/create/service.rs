use std::sync::Arc;

use axum::{
    extract::{State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use crate::{
    db::models::PetModel,
    dto::request::{CreatePetDTO},
    AppState,
};

pub async fn create(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreatePetDTO>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        PetModel,
        "INSERT INTO pets (name,category,breed) VALUES ($1, $2, $3) RETURNING *",
        body.name.to_string(),
        body.category.to_owned().unwrap_or("".to_string()),
        body.breed
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(note) => {
            let note_response = json!({"status": "success","data": json!({
                "note": note
            })});

            return Ok((StatusCode::CREATED, Json(note_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Note with that title already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}