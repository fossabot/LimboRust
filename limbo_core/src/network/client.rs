use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct ClientHandler {
	packet_out: Sender<()>
}

impl ClientHandler {
	pub fn new() -> (ClientHandler, Receiver<()>) {
		let (out_tx, out_rx) = channel(32);
		(ClientHandler {
			packet_out: out_tx
		}, out_rx)
	}
}
