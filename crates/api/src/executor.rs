use async_trait::async_trait;
use common::executor::{ExecutionResult, TxExecutor};
use uuid::Uuid;

pub struct MockExecutor;

#[async_trait]
impl TxExecutor for MockExecutor {
    async fn execute(&self, _tx_id: Uuid) -> ExecutionResult {
        let success = rand::random::<bool>();
        if success {
            ExecutionResult {
                success,
                error: None,
            }
        } else {
            ExecutionResult {
                success: false,
                error: Some("Simulated Failure".into()),
            }
        }
    }
}
