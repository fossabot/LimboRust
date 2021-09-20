mod network;
mod shutdown;

#[macro_use]
extern crate log;

use log4rs;

use network::server::MainServer;
use shutdown::ShutdownHandle;

#[tokio::main]
async fn main() {
    log4rs::init_file("config/log4rs.yml", Default::default()).unwrap();

    info!("Launching Limbo Rust!");

    let (finished_tx, mut finished_rx) = tokio::sync::mpsc::channel(1);
    let mut shutdown_handle = ShutdownHandle::new(finished_tx);

    {
        let mut main_server = MainServer::new(shutdown_handle.clone());
        if let Err(error) = main_server.start_server_thread().await {
            error!("Couldn't start server thread due to {}", error);
            return;
        }
    }

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            debug!("Shutting down server!");
            shutdown_handle.send_shutdown();
        },
        _ = shutdown_handle.wait_for_shutdown() => {},
    }

    std::mem::drop(shutdown_handle);
    let _ = finished_rx.recv().await;
    debug!("End of application!");
}
