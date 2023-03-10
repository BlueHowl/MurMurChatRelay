mod models;
mod infrastructure;
mod server_manager;
mod server_thread;
mod aesgcm;
mod net_chooser;

use std::any::Any;
use std::net::{TcpStream};
use std::thread;
use std::str;
use std::io::{self, Read, Write};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use regex::Regex;
use crate::infrastructure::json_repository::get_relay;
use crate::server_manager::ServerManager;


fn main() -> io::Result<()> {
    let mut net_chooser = net_chooser::NetWorkInterfacesList::new();
    let network_interface = net_chooser.choose();
    let relay = get_relay();
    println!("Relay {}", relay.to_string());

    let server_manager = ServerManager::new();

    server_manager.start_listening(relay, network_interface.get_multicast_address().clone()).expect("TODO: panic message");

    Ok(())
}
