use crate::network::client::ClientHandler;
use crate::network::listener::start_network_listening;
use crate::shutdown::ShutdownHandle;

use crossbeam::channel::{unbounded, Receiver};

use std::io;
use std::thread;
use std::time::{Duration, Instant};

pub struct MainServer {
    shutdown_handle: ShutdownHandle,
    #[doc(hidden)]
    client_connections: Vec<ClientHandler>,
    #[doc(hidden)]
    client_instance_rx: Option<Receiver<ClientHandler>>
}

impl MainServer {
    pub fn new(shutdown: ShutdownHandle) -> MainServer {
        MainServer {
            shutdown_handle: shutdown,
            client_connections: Vec::new(),
            client_instance_rx: None
        }
    }

    pub async fn start_server_thread(mut self) -> io::Result<()> {
        info!("Starting server thread...");

        let (client_instance_tx, client_instance_rx) = unbounded();
        let shutdown_handle2 = self.shutdown_handle.clone();

        self.client_instance_rx = Some(client_instance_rx);
        thread::Builder::new()
            .name("Main Server Thread".to_string())
            .spawn(|| {
                self.start_server_logic();
            })?;

        tokio::spawn(start_network_listening(
            shutdown_handle2,
            client_instance_tx,
        ));

        Ok(())
    }

    #[doc(hidden)]
    fn start_server_logic(mut self) {
        debug!("Starting server logic!");
        let mut old_time = Instant::now();
        let mut time_measured = 0;

        while self.shutdown_handle.is_no_shutdown() {
            let current_time = Instant::now();
            let mut time_since_last_cycle = current_time.duration_since(old_time).as_millis();

            if time_since_last_cycle > 2000 {
                warn!(
                    "Can't keep up! Skipping {} tick(s)",
                    time_since_last_cycle / 50
                );
                time_since_last_cycle = 2000;
            }

            time_measured += time_since_last_cycle;
            old_time = current_time;

            while time_measured > 50 {
                time_measured -= 50;
                self.tick();
            }

            thread::sleep(Duration::from_millis(
                (u64::MAX as u128).min(1.max(50 - time_measured)) as u64,
            ));
        }

        debug!("Stopping server logic!");
    }

    #[doc(hidden)]
    fn tick(&mut self) {
    }
}
