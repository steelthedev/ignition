mod routes;
mod state;
mod types;

use axum::Router;
use queue::Queue;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing::info;

use crate::state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://ignition:ignitionpassword@localhost:5432/ignition")
        .await
        .unwrap();

    let (queue, reciever) = Queue::new(100);

    let state = AppState { db, queue };

    tokio::spawn(async move {
        worker_loop(reciever).await;
    });

    let app = Router::new().merge(routes::routes()).with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    info!("App is running on port 8000");

    axum::serve(listener, app).await.unwrap();
}

async fn worker_loop(mut receiver: tokio::sync::mpsc::Receiver<queue::TxJob>) {
    while let Some(job) = receiver.recv().await {
        println!("Processing tx: {:?}", job.tx_id);
    }
}
