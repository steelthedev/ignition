use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TxJob {
    pub tx_id: Uuid,
    pub payload: String,
    pub retries: u32,
}

#[derive(Clone)]
pub struct Queue {
    sender: mpsc::Sender<TxJob>,
}

impl Queue {
    pub fn new(buffer: usize) -> (Self, mpsc::Receiver<TxJob>) {
        let (sender, receiver) = mpsc::channel(buffer);
        (Self { sender }, receiver)
    }

    pub async fn enqueue(&self, job: TxJob) {
        self.sender.send(job).await.unwrap();
    }
}
