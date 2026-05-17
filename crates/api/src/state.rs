use queue::Queue;
use sqlx::PgPool;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub queue: Queue,

    pub broadcaster: broadcast::Sender<common::events::TxEvent>,
}
