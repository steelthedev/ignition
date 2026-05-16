use axum::{Json, extract::State};
use serde_json::{Value, json};

use crate::state::AppState;

pub async fn get_health(State(state): State<AppState>) -> Json<Value> {
    let _ = state.db.acquire().await.unwrap();
    Json(json!({"message":"Good health"}))
}
