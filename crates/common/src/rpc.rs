use async_trait::async_trait;

#[async_trait]
pub trait RpcManager: Send + Sync {
    async fn healthcheck(&self) -> anyhow::Result<()>;
}
