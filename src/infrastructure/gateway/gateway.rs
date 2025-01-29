use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix_web::web::Bytes;
use actix_web::Error;
use futures_util::stream::{self, Stream};
use tokio::sync::mpsc;

use crate::infrastructure::gateway::events::GatewayEvent;
use crate::shared::types::snowflake::Snowflake;

type Clients = Arc<Mutex<HashMap<Snowflake, Vec<Arc<mpsc::Sender<String>>>>>>;

#[derive(Clone)]
pub struct Gateway {
    clients: Clients,
}

impl Gateway {
    pub fn new() -> Self {
        Gateway {
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn register_client(&self, user_id: Snowflake) -> impl Stream<Item = Result<Bytes, Error>> {
        let (tx, mut rx) = mpsc::channel::<String>(100);
        let tx = Arc::new(tx);

        let mut clients = self.clients.lock().unwrap();
        clients.entry(user_id).or_insert_with(Vec::new).push(tx.clone());

        stream::unfold(rx, |mut rx| async move {
            match rx.recv().await {
                Some(msg) => Some((Ok(Bytes::from(format!("data: {}\n\n", msg))), rx)),
                None => None,
            }
        })
    }

    pub async fn send_event(&self, user_id: &Snowflake, event: &GatewayEvent) {
        let mut clients = self.clients.lock().unwrap();

        if let Some(senders) = clients.get_mut(user_id) {
            let json_event = serde_json::to_string(event).unwrap();

            senders.retain(|sender| {
                let sender_clone = Arc::clone(sender);
                let event_clone = json_event.clone();

                tokio::spawn(async move { sender_clone.send(event_clone).await.is_ok() });

                true
            });
        }
    }
}
