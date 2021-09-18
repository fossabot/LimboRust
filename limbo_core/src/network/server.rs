#![allow(dead_code)]

use tokio::net;
use tokio::runtime::{Runtime, Builder};
use std::thread;
use std::io;

pub struct MainServer {
}

impl MainServer {
	pub fn new() -> MainServer {
		MainServer {
			
		}
	}

	pub fn start_server_thread(&self) {
		info!("Starting server thread...");

		thread::Builder::new()
			.name("Connection Listener".to_string())
			.spawn(|| {
				Builder::new_multi_thread()
				.enable_all()
				.build()
				.unwrap()
				.block_on(start_client_listening());
			})
			.unwrap();
	}

}

async fn start_client_listening() -> io::Result<()> {
	let listener = net::TcpListener::bind("127.0.0.1:30000").await?;

	debug!("Started listening for connections!");

	loop {
		let (socket, addr) = listener.accept().await?;
	}
}