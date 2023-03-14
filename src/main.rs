mod models;
mod infrastructure;
mod server_manager;
mod server_thread;
mod aesgcm;
mod net_chooser;

use std::io::{self};
use crate::infrastructure::json_repository::get_relay;
use crate::server_manager::ServerManager;


fn main() -> io::Result<()> {
    let mut net_chooser = net_chooser::NetWorkInterfacesList::new();
    let network_interface = net_chooser.choose();
    let relay = get_relay();
    println!("Relay {}", relay.to_string());

    let server_manager = ServerManager::new();

    server_manager.start_listening(relay, network_interface.get_multicast_address().clone()).expect("Failed to start relay");

    Ok(())
}
