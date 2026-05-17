use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::Utc;
use common::tx_status::TxStatus;
use uuid::Uuid;

use crate::{
    state::AppState,
    types::tx::{SubmitTxRequest, SubmitTxResponse, TxStatusResponse},
};

pub async fn submit_tx(
    State(state): State<AppState>,
    Json(payload): Json<SubmitTxRequest>,
) -> Json<SubmitTxResponse> {
    let id = Uuid::new_v4();

    let status = TxStatus::Pending;

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
    .bind(&status.as_str())
    .bind(created)
    .execute(&state.db)
    .await
    .unwrap();

    state
        .queue
        .enqueue(queue::TxJob {
            tx_id: id,
            retries: 0,
        })
        .await;

    Json(SubmitTxResponse {
        id,
        status,
        created_at: created,
    })
}

pub async fn get_tx_status(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> anyhow::Result<Json<TxStatusResponse>, StatusCode> {
    let tx = sqlx::query_as!(
        TxStatusResponse,
        r#"
            SELECT id, status, retry_count,last_error,created_at FROM transactions WHERE id = $1
            "#,
        id,
    )
    .fetch_optional(&state.db)
    .await
    .unwrap();

    match tx {
        Some(tx) => Ok(Json(tx)),
        None => Err(StatusCode::NOT_FOUND),
    }
}
