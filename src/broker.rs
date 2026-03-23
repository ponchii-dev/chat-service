// /src/broker.rs


use crate::models::ChatMessage;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct Broker {
    tx: broadcast::Sender<ChatMessage>,
}

impl Broker {
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(1024);
        Broker { tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ChatMessage> {
        self.tx.subscribe()
    }

    pub fn send(&self, msg: ChatMessage) {
        let _ = self.tx.send(msg);
    }
}
