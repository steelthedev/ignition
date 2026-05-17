use std::sync::Arc;

use async_trait::async_trait;
use base64::Engine;

use bincode_next::{config, serde::decode_from_slice};
use common::{
    executor::{ExecutionResult, TxExecutor},
    rpc::RpcManager,
};
use solana_client::rpc_response::transaction::versioned::VersionedTransaction;
use uuid::Uuid;

pub struct MockExecutor {
    pub rpc: Arc<dyn RpcManager>,
}

#[async_trait]
impl TxExecutor for MockExecutor {
    async fn execute(&self, _tx_id: Uuid, payload: String) -> ExecutionResult {
        let decoded = match base64::engine::general_purpose::STANDARD.decode(payload) {
            Ok(bytes) => bytes,
            Err(err) => {
                return ExecutionResult {
                    success: false,
                    error: Some(format!("Base64 decode failed {}", err)),
                };
            }
        };

        let bincode_config = config::legacy()
            .with_fixed_int_encoding()
            .with_little_endian();

        let transaction: VersionedTransaction = match decode_from_slice(&decoded, bincode_config) {
            Ok((tx, _bytes_read)) => tx,
            Err(err) => {
                return ExecutionResult {
                    success: false,
                    error: Some(format!("Decoding of tx failed: {}", err)),
                };
            }
        };

        if let Err(err) = self.rpc.simulate_transaction(transaction).await {
            return ExecutionResult {
                success: false,
                error: Some(format!("Error simulating transaction {}", err)),
            };
        };
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
