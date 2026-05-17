use std::sync::Arc;

use common::{executor::TxExecutor, rpc::RpcManager};
use queue::Queue;
use sqlx::PgPool;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub queue: Queue,

    pub broadcaster: broadcast::Sender<common::events::TxEvent>,
    pub executor: Arc<dyn TxExecutor>,
    pub rpc: Arc<dyn RpcManager>,
}
