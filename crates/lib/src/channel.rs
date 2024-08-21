use std::sync::mpsc::{self, Receiver, SendError, Sender, SyncSender};

pub trait ChannelSender<T>: Clone {
    fn send(&self, val: T) -> Result<(), SendError<T>>;
}

struct AsyncChannelSender<T>(Sender<T>);

impl<T> Clone for AsyncChannelSender<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> ChannelSender<T> for AsyncChannelSender<T> {
    fn send(&self, val: T) -> Result<(), SendError<T>> {
        self.0.send(val)
    }
}

struct SyncChannelSender<T>(SyncSender<T>);

impl<T> Clone for SyncChannelSender<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> ChannelSender<T> for SyncChannelSender<T> {
    fn send(&self, val: T) -> Result<(), SendError<T>> {
        self.0.send(val)
    }
}

#[derive(Clone)]
struct NoopChannelSender;

impl<T> ChannelSender<T> for NoopChannelSender {
    fn send(&self, _val: T) -> Result<(), SendError<T>> {
        Ok(())
    }
}

pub fn channel<T>() -> (impl ChannelSender<T>, Receiver<T>) {
    let (tx, rx) = mpsc::channel();
    (AsyncChannelSender(tx), rx)
}

pub fn sync_channel<T>(size: usize) -> (impl ChannelSender<T>, Receiver<T>) {
    let (tx, rx) = mpsc::sync_channel(size);
    (SyncChannelSender(tx), rx)
}

pub fn noop_sender<T>() -> impl ChannelSender<T> {
    NoopChannelSender
}
