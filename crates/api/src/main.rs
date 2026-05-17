mod executor;
mod routes;
mod rpc;
mod state;
mod types;

use std::{env, sync::Arc};

use axum::Router;
use common::tx_status::TxStatus;
use queue::{Queue, TxJob};

use solana_client::nonblocking::rpc_client::RpcClient;
use sqlx::postgres::PgPoolOptions;
use tokio::{net::TcpListener, sync::broadcast};
use tracing::info;

use crate::{executor::MockExecutor, rpc::SolanaRpcManager, state::AppState};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().unwrap();

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(env::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap();

    let (queue, reciever) = Queue::new(100);

    let (broadcaster, _) = broadcast::channel(100);

    let rpc = Arc::new(SolanaRpcManager {
        client: RpcClient::new(env::var("RPC_URL").unwrap()),
    });
    let executor = Arc::new(MockExecutor { rpc: rpc.clone() });

    let state = AppState {
        db,
        queue,
        broadcaster,
        executor,
        rpc,
    };

    tokio::spawn(worker_loop(reciever, state.clone()));

    let app = Router::new().merge(routes::routes()).with_state(state);

    let listener = TcpListener::bind(format!(
        "{}:{}",
        env::var("IP").unwrap().as_str(),
        env::var("PORT").unwrap().as_str()
    ))
    .await
    .unwrap();

    info!("App is running on port 8000");

    axum::serve(listener, app).await.unwrap();
}

async fn worker_loop(mut receiver: tokio::sync::mpsc::Receiver<queue::TxJob>, state: AppState) {
    while let Some(job) = receiver.recv().await {
        if let Err(err) = state.rpc.healthcheck().await {
            println!("Rpc healthcheck failed: {:?}", err);

            continue;
        }

        println!("Processing tx: {:?}", job.tx_id);

        sqlx::query(
            r#"
            UPDATE transactions
            SET status = $1
            WHERE id = $2
            
            "#,
        )
        .bind(TxStatus::Processing.as_str())
        .bind(job.tx_id)
        .execute(&state.db)
        .await
        .unwrap();
        state
            .broadcaster
            .send(common::events::TxEvent {
                tx_id: job.tx_id,
                status: TxStatus::Processing.as_str().into(),
            })
            .unwrap();

        let result = state.executor.execute(job.tx_id, job.payload.clone()).await;

        if result.success {
            sqlx::query(
                r#"
            UPDATE transactions
            SET status = $1
            WHERE id = $2
        "#,
            )
            .bind(TxStatus::Confirmed.as_str())
            .bind(job.tx_id)
            .execute(&state.db)
            .await
            .unwrap();

            state
                .broadcaster
                .send(common::events::TxEvent {
                    tx_id: job.tx_id,
                    status: TxStatus::Confirmed.as_str().into(),
                })
                .unwrap();
            println!("Transaction confirmed: {:?}", job.tx_id);
        } else {
            sqlx::query(
                r#"
    UPDATE transactions
    SET
        status = $1,
        retry_count = $2,
        last_error = $3
    WHERE id = $4
    "#,
            )
            .bind(TxStatus::Failed.as_str())
            .bind(job.retries as i32)
            .bind("Simulated failure")
            .bind(job.tx_id)
            .execute(&state.db)
            .await
            .unwrap();
            state
                .broadcaster
                .send(common::events::TxEvent {
                    tx_id: job.tx_id,
                    status: TxStatus::Failed.as_str().into(),
                })
                .unwrap();
            println!("Transaction failed: {:?}", job.tx_id)
        }

        if job.retries < 3 {
            println!("Retrying tx: {:?}", job.tx_id);

            tokio::time::sleep(std::time::Duration::from_secs(2_u64.pow(job.retries))).await;

            state
                .queue
                .enqueue(TxJob {
                    tx_id: job.tx_id,
                    retries: job.retries + 1,
                    payload: job.payload,
                })
                .await;
        }
    }
}
