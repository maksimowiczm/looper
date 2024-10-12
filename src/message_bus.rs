use std::ops::Deref;
use tokio::sync::mpsc;

pub struct MessageBus<M>(mpsc::Sender<M>);

impl<M> MessageBus<M>
where
    M: Send + 'static,
{
    pub fn with_subscriber<S>(capacity: usize, subscriber: S) -> Self
    where
        S: Fn(M) + Send + 'static,
    {
        let (sender, mut receiver) = mpsc::channel(capacity);

        tokio::spawn(async move {
            while let Some(message) = receiver.recv().await {
                subscriber(message);
            }
        });

        Self(sender)
    }
}

impl<M> Deref for MessageBus<M> {
    type Target = mpsc::Sender<M>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
