use std::sync::mpsc;
use std::thread::JoinHandle;

pub struct MessageBus<M> {
    sender: Option<mpsc::Sender<M>>,
    listener: JoinHandle<()>,
}

impl<M> MessageBus<M>
where
    M: Send + 'static,
{
    pub fn with_subscriber<S>(subscriber: S) -> Self
    where
        S: Fn(M) + Send + 'static,
    {
        let (sender, receiver) = mpsc::channel();

        let listener = std::thread::spawn(move || {
            while let Ok(message) = receiver.recv() {
                subscriber(message);
            }
        });

        MessageBus {
            sender: Some(sender),
            listener,
        }
    }

    pub fn close(&mut self) {
        self.sender = None
    }

    pub fn join(self) -> std::thread::Result<()> {
        self.listener.join()
    }
}

impl<M> AsRef<Option<mpsc::Sender<M>>> for MessageBus<M> {
    fn as_ref(&self) -> &Option<mpsc::Sender<M>> {
        &self.sender
    }
}
