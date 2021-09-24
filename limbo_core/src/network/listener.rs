use std::io::Cursor;
use std::net::SocketAddr;

use bytes::{Buf, Bytes, BytesMut};
use crossbeam::channel::Sender;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::Receiver;

use limbo_protocol::{ByteLimitCheck, PacketBufferRead};

use crate::network::client::ClientHandler;
use crate::shutdown::ShutdownHandle;

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

async fn process_socket(
    mut shutdown_handle: ShutdownHandle,
    mut socket: TcpStream,
    addr: SocketAddr,
    mut packet_out_rx: Receiver<()>,
) {
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
                    //if buffer.is_empty() {
                        break;
                    //}
                }

                if let Some(mut frame) = parse_frame(&buffer) {
                    if let Ok(id) = frame.read_var_i32() {
                        debug!("Packet id = {}", id);
                    }
                    if let Ok(version) = frame.read_var_i32() {
                        debug!("Protocol version = {}", version);
                    }
                    if let Ok(server_addr) = frame.read_string(32767) {
                        debug!("Server address = {}", server_addr);
                    }
                    if let Ok(port) = frame.read_u16() {
                        debug!("Server port = {}", port);
                    }
                    if let Ok(next_state) = frame.read_var_i32() {
                        debug!("Next state = {}", next_state);
                    }
                    debug!("Bytes in frame left = {}", frame.remaining());
                }

                debug!("Received {} bytes!", n);
            }
            else => {}
        }
    }
}

fn parse_frame(buffer: &BytesMut) -> Option<Bytes> {
    let mut buf = Cursor::new(&buffer[..]);
    let mut length_bytes: [u8; 3] = [0; 3];
    for i in 0..3 {
        if buf.remaining() == 0 {
            return None;
        }

        length_bytes[i] = buf.get_u8();

        if length_bytes[i] & 0b1000_0000 == 0 {
            let mut length_buf = Cursor::new(&length_bytes[..]);
            if let Ok(length) = length_buf.read_var_i32() {
                debug!("Frame is {} bytes long", length);
                if let Ok(_) = buf.ensure_bytes_available(length as usize) {
                    return Some(buf.copy_to_bytes(length as usize));
                }
            }
        }
    }
    None
}
