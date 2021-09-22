use crate::network::client::ClientHandler;
use crate::shutdown::ShutdownHandle;

use crossbeam::channel::Sender;

use std::net::SocketAddr;

use tokio::sync::mpsc::Receiver;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use bytes::BytesMut;

pub(crate) async fn start_network_listening(
    mut shutdown_handle: ShutdownHandle,
    client_instance_tx: Sender<ClientHandler>,
) {
    debug!("Started network listening!");
    let listener = match TcpListener::bind(("127.0.0.1", 30000)).await {
        Ok(listener) => listener,
        Err(error) => {
            error!(
                "Could not bind to the port, following error was received: \"{}\"",
                error
            );
            shutdown_handle.send_shutdown();
            return;
        }
    };

    loop {
        tokio::select! {
            _ = shutdown_handle.wait_for_shutdown() => {
                break;
            }
            connection = listener.accept() => {
                if let Ok((socket, addr)) = connection {
                    let (client_handler, packet_out_rx) = ClientHandler::new();
                    if let Ok(_) = client_instance_tx.try_send(client_handler) {
                        let shutdown_handle2 = shutdown_handle.clone();
                        tokio::spawn(async move {
                            debug!("Accepted connection!!");
                            process_socket(shutdown_handle2, socket, addr, packet_out_rx).await;
                        });
                    }
                }
            }
        }
    }
    debug!("Stopped listening for connections");
}

async fn process_socket(mut shutdown_handle: ShutdownHandle, mut socket: TcpStream, addr: SocketAddr, mut packet_out_rx: Receiver<()>) {
    let mut buffer = BytesMut::with_capacity(4096);

    loop {
        tokio::select! {
            _ = shutdown_handle.wait_for_shutdown() => {
                break;
            }
            Some(_) = packet_out_rx.recv() => {
                debug!("Got packet to send!");
            }
            Ok(n) = socket.read_buf(&mut buffer) => {
                if n == 0 {
                    break;
                }
                debug!("Received {} bytes!", n);
            }
            else => {
                debug!("Triggered else branch!");
            }
        }
    }
}









