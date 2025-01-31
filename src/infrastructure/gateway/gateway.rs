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

type Clients = Arc<Mutex<HashMap<Snowflake, Vec<mpsc::Sender<String>>>>>;

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

        let mut clients = self.clients.lock().unwrap();
        clients.entry(user_id).or_insert_with(Vec::new).push(tx.clone());

        tracing::info!("Client connected to gateway: {}", user_id);

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
            let json_event = match serde_json::to_string(event) {
                Ok(json) => json,
                Err(err) => {
                    tracing::error!("Error serializing event: {:?}", err);
                    return;
                }
            };

            let mut still_alive = Vec::new();

            for sender in senders.iter() {
                let sender_clone = sender.clone();
                let event_clone = json_event.clone();

                if sender_clone.send(event_clone).await.is_ok() {
                    still_alive.push(sender_clone);
                }
            }

            *senders = still_alive;
        }
    }
}
