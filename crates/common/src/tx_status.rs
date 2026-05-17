use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum TxStatus {
    Pending,
    Processing,
    Confirmed,
    Failed,
}

impl TxStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Processing => "processing",
            Self::Confirmed => "confirmed",
            Self::Failed => "failed",
        }
    }
}
