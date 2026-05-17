use async_trait::async_trait;
use uuid::Uuid;

#[derive(Debug)]
pub struct ExecutionResult {
    pub success: bool,
    pub error: Option<String>,
}

#[async_trait]
pub trait TxExecutor: Send + Sync {
    async fn execute(&self, tx_id: Uuid) -> ExecutionResult;
}
