use tokio::sync::{broadcast, mpsc};

/// A handle to help asynchronously shutting down our application
pub struct ShutdownHandle {
    signal_sender: broadcast::Sender<()>,
    signal_receiver: broadcast::Receiver<()>,
    shutdown_finished: mpsc::Sender<()>
}

impl ShutdownHandle {
    /// Returns a Shutdown handle with the sender ("hook") given.
    /// 
    /// # Arguments
    ///
    /// * `finished_hook` - A Tokio sender from which the corresponding receiver can be used
    /// to know whether the process owning this handle is finished.
    ///
    /// # Note
    /// This handle is also able to signal all other clones of this handle
    /// when the program wants to shut down in order for necessary cleanup to happen.
    ///
    /// More info under `ShutdownHandle::send_shutdown` and `ShutdownHandle::wait_for_shutdown`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::mpsc::channel;
    /// use std::mem::drop;
    ///
    /// use shutdown::ShutdownHandle;
    ///
    /// async fn process() {
    ///     let (send, mut recv) = channel(1);
    ///     let shutdown_handle = ShutdownHandle::new(send);
    ///
    ///     // Pass this handle by cloning to other parts of the program that should be cleaned up when shutting down
    ///
    ///     // Drop the local scope handle before waiting
    ///     drop(shutdown_handle);
    ///     let _ = recv.recv().await;
    ///     // everything has shut down
    /// }
    /// ```
    pub fn new(finished_hook: mpsc::Sender<()>) -> ShutdownHandle {
        let (sender, receiver) = broadcast::channel(1);
        ShutdownHandle {
            signal_sender: sender,
            signal_receiver: receiver,
            shutdown_finished: finished_hook
        }
    }

    pub fn send_shutdown(&self) {
        let _ = self.signal_sender.send(());
    }

    pub fn is_shutdown(&mut self) -> bool {
        if let Err(error) = self.signal_receiver.try_recv() {
            if error == broadcast::error::TryRecvError::Empty {
                return false;
            }
        }
        true
    }

    pub async fn wait_for_shutdown(&mut self) -> Result<(), broadcast::error::RecvError> {
        self.signal_receiver.recv().await
    }
}

impl std::clone::Clone for ShutdownHandle {
    fn clone(&self) -> Self {
        ShutdownHandle {
            signal_sender: self.signal_sender.clone(),
            signal_receiver: self.signal_sender.subscribe(),
            shutdown_finished: self.shutdown_finished.clone()
        }
    }
}
