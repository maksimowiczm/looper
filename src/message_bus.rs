use std::sync::{Arc, RwLock};
use tokio::sync::mpsc::error::SendError;

pub type Subscriber<M> = Box<dyn Fn(M) + Send + Sync>;
pub struct MessageBus<M> {
    sender: tokio::sync::mpsc::Sender<M>,
    // consider using an async RwLock because subscribers might lock for a long time
    subscribers: Arc<RwLock<Vec<Subscriber<M>>>>,
}

impl<M> MessageBus<M>
where
    M: Send + Sync + Clone + 'static,
{
    pub fn new(capacity: usize) -> Self {
        let (sender, mut receiver) = tokio::sync::mpsc::channel(capacity);
        let subscribers = Arc::new(RwLock::new(Vec::new()));

        let bus = Self {
            sender,
            subscribers: subscribers.clone(),
        };

        // Spawn a task that will listen for messages and send them to all subscribers
        tokio::spawn(async move {
            while let Some(message) = receiver.recv().await {
                subscribers
                    .read()
                    .expect("This is the only place where the lock is held by this thread")
                    .iter()
                    .for_each(|subscriber| subscriber(message.clone()));
            }
        });

        bus
    }
}

impl<M> MessageBus<M> {
    pub async fn send(&self, message: M) -> Result<(), SendError<M>> {
        self.sender.send(message).await
    }

    pub fn subscribe(&self, subscriber: Subscriber<M>) {
        self.subscribers
            .write()
            .expect("If writer panics we are cooked anyway 💀")
            .push(subscriber);
    }
}
