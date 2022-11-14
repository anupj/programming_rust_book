use crate::connection::Outbound;
use async_std::task;
use std::sync::Arc;
use tokio::sync::broadcast;

/// Represents a broadcast channel carrying
/// `Arc<String>` values: posting a message to
/// the group broadcasts it to all current members
pub struct Group {
    name: Arc<String>,
    // Represents the sending end of the
    // group's `broadcast` channel
    sender: broadcast::Sender<Arc<String>>,
}

impl Group {
    pub fn new(name: Arc<String>) -> Group {
        // We have no need of the receiver so
        // we ignore it.
        let (sender, _receiver) = broadcast::channel(1000);
        Group { name, sender }
    }

    pub fn join(&self, outbound: Arc<Outbound>) {
        // Create a new receiver for the channel
        let receiver = self.sender.subscribe();

        // Spawn a new async task to monitor
        // that receiver for messages
        // and write them back to the client
        task::spawn(handle_subscriber(self.name.clone(), receiver, outbound));
    }

    pub fn post(&self, message: Arc<String>) {
        // This only returns an error when there are no subscribers.
        // A connection's outgoing side can exit, dropping its
        // subscription, slightly before its incoming side, which may end up
        // trying to send a message to an empty group
        let _ignored = self.sender.send(message);
    }
}

use async_chat::FromServer;
use tokio::sync::broadcast::error::RecvError;

async fn handle_subscriber(
    group_name: Arc<String>,
    mut receiver: broadcast::Receiver<Arc<String>>,
    outbound: Arc<Outbound>,
) {
    loop {
        let packet = match receiver.recv().await {
            Ok(message) => FromServer::Message {
                group_name: group_name.clone(),
                message: message.clone(),
            },

            Err(RecvError::Lagged(n)) => {
                FromServer::Error(format!("Dropped {} messages from {}.", n, group_name))
            }

            Err(RecvError::Closed) => break,
        };

        if outbound.send(packet).await.is_err() {
            break;
        }
    }
}
