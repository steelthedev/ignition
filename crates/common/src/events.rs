use serde::Serialize;

use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct TxEvent {
    pub tx_id: Uuid,
    pub status: String,
}
