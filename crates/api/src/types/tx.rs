use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SubmitTxRequest {
    pub payload: String,
}

#[derive(Debug, Serialize)]
pub struct SubmitTxResponse {
    pub id: Uuid,
    pub status: String,
    pub created_at: DateTime<Utc>,
}
