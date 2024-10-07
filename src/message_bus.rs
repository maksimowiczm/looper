use std::sync::{Arc, PoisonError, RwLock, RwLockWriteGuard};
use tokio::sync::mpsc::error::SendError;

pub type Subscriber<M> = Box<dyn Fn(M) + Send + Sync>;
pub type SubscribeError<'a, M> =
    PoisonError<RwLockWriteGuard<'a, Vec<Box<dyn Fn(M) + Send + Sync>>>>;
pub struct MessageBus<M> {
    sender: tokio::sync::mpsc::Sender<M>,
    subscribers: Arc<RwLock<Vec<Subscriber<M>>>>,
}

impl<M> MessageBus<M>
where
    M: Send + Sync + Copy + 'static,
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
                    .for_each(|receiver| receiver(message));
            }
        });

        bus
    }
}

impl<M> MessageBus<M> {
    pub async fn send(&self, message: M) -> Result<(), SendError<M>> {
        self.sender.send(message).await
    }

    pub fn subscribe(&self, subscriber: Subscriber<M>) -> Result<(), SubscribeError<M>> {
        self.subscribers.write()?.push(subscriber);
        Ok(())
    }
}
