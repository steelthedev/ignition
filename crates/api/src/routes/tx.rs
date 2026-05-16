use std::os::linux::raw::stat;

use axum::{Json, extract::State};
use chrono::Utc;
use uuid::Uuid;

use crate::{
    state::AppState,
    types::tx::{SubmitTxRequest, SubmitTxResponse},
};

pub async fn submit_tx(
    State(state): State<AppState>,
    Json(payload): Json<SubmitTxRequest>,
) -> Json<SubmitTxResponse> {
    let id = Uuid::new_v4();

    let status = "Pending".to_string();

    let created = Utc::now();

    sqlx::query(
        r#"
        INSERT INTO transactions(
        id,
        payload,
        status,
        created_at
        )
        VALUES($1,$2,$3,$4)
        "#,
    )
    .bind(id)
    .bind(payload.payload)
    .bind(&status)
    .bind(created)
    .execute(&state.db)
    .await
    .unwrap();

    state.queue.enqueue(queue::TxJob { tx_id: id }).await;

    Json(SubmitTxResponse {
        id,
        status,
        created_at: created,
    })
}
