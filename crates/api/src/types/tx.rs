use chrono::{DateTime, Utc};
use common::tx_status::TxStatus;
use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SubmitTxRequest {
    pub payload: String,
}

#[derive(Debug, Serialize)]
pub struct SubmitTxResponse {
    pub id: Uuid,
    pub status: TxStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct TxStatusResponse {
    pub id: Uuid,
    pub status: String,
    pub retry_count: i32,
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
}
