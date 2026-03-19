use std::{collections::HashMap, sync::Mutex};

use tokio::sync::broadcast;

use crate::dto::MessageEventProps;

#[derive(Debug, Default)]
pub struct MessageEventBroadcaster {
    senders: Mutex<HashMap<uuid::Uuid, broadcast::Sender<MessageEventProps>>>,
}

impl MessageEventBroadcaster {
    pub fn publish(&self, user_id: uuid::Uuid, event: MessageEventProps) {
        let sender = self.sender_for_user(user_id);
        let _ = sender.send(event);
    }

    pub fn subscribe(&self, user_id: uuid::Uuid) -> broadcast::Receiver<MessageEventProps> {
        self.sender_for_user(user_id).subscribe()
    }

    fn sender_for_user(&self, user_id: uuid::Uuid) -> broadcast::Sender<MessageEventProps> {
        let mut senders = self.senders.lock().expect("message broadcaster lock");
        senders
            .entry(user_id)
            .or_insert_with(|| broadcast::channel(128).0)
            .clone()
    }
}
