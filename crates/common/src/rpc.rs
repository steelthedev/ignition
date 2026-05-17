use async_trait::async_trait;
use solana_sdk::transaction::VersionedTransaction;

#[async_trait]
pub trait RpcManager: Send + Sync {
    async fn healthcheck(&self) -> anyhow::Result<()>;

    async fn simulate_transaction(&self, transaction: VersionedTransaction) -> anyhow::Result<()>;
}
