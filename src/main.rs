mod models;
mod infrastructure;
mod server_manager;
mod server_thread;
mod aesgcm;

use std::net::{TcpStream};
use std::thread;
use std::str;
use std::io::{self, Read, Write};
use regex::Regex;
use crate::infrastructure::json_repository::get_relay;
use crate::server_manager::ServerManager;


fn main() -> io::Result<()> {

    let relay = get_relay();
    println!("Relay {}", relay.to_string());

    let server_manager = ServerManager::new();

    server_manager.start_listening(relay).expect("TODO: panic message");

    Ok(())
}
