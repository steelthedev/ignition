use async_trait::async_trait;
use common::rpc::RpcManager;

use solana_client::nonblocking::rpc_client::RpcClient;

pub struct SolanaRpcManager {
    pub client: RpcClient,
}

#[async_trait]
impl RpcManager for SolanaRpcManager {
    async fn healthcheck(&self) -> anyhow::Result<()> {
        self.client.get_latest_blockhash().await?;
        Ok(())
    }
}
