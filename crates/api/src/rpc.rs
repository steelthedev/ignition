use anyhow::Ok;
use async_trait::async_trait;
use common::rpc::RpcManager;
use solana_client::nonblocking::rpc_client::RpcClient;

use solana_client::rpc_config::{CommitmentConfig, RpcSimulateTransactionConfig};

use solana_sdk::transaction::VersionedTransaction;

pub struct SolanaRpcManager {
    pub client: RpcClient,
}

#[async_trait]
impl RpcManager for SolanaRpcManager {
    async fn healthcheck(&self) -> anyhow::Result<()> {
        self.client.get_latest_blockhash().await?;
        Ok(())
    }

    async fn simulate_transaction(&self, transaction: VersionedTransaction) -> anyhow::Result<()> {
        self.client
            .simulate_transaction_with_config(
                &transaction,
                RpcSimulateTransactionConfig {
                    sig_verify: false,
                    commitment: Some(CommitmentConfig::processed()),
                    ..Default::default()
                },
            )
            .await?;

        Ok(())
    }
}
