#![allow(dead_code)]

use crate::shutdown;

use std::io;
use std::thread;
use tokio::net;

pub struct MainServer {
    shutdown_handle: shutdown::ShutdownHandle
}

impl MainServer {
    pub fn new(shutdown: shutdown::ShutdownHandle) -> MainServer {
        MainServer {
            shutdown_handle: shutdown
        }
    }

    pub async fn start_server_thread(&mut self) -> io::Result<()> {
        info!("Starting server thread...");

        thread::Builder::new()
            .name("Main Server Thread".to_string())
            .spawn(|| {
                start_server_logic();
            })?;
        Ok(())
    }
}

#[doc(hidden)]
fn start_server_logic() {
    debug!("Starting server logic!");
}

#[doc(hidden)]
async fn start_client_listening() -> io::Result<()> {
    let listener = net::TcpListener::bind("127.0.0.1:30000").await?;

    debug!("Started listening for connections!");

    loop {
        let (socket, addr) = listener.accept().await?;
    }
}
