mod network;

#[macro_use]
extern crate log;

use std::{thread, time};
use log4rs;

use network::server::MainServer;

fn main() {
    log4rs::init_file("config/log4rs.yml", Default::default()).unwrap();

    info!("Launching Limbo Rust!");

    let main_server = MainServer::new();
    thread::Builder::new()
        .name("Server Main Thread".to_string())
        .spawn(move || {
            main_server.start_server_thread();
        }).unwrap();

    loop {
        
    }
}
