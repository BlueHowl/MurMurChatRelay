mod models;
mod infrastructure;
mod server_manager;
mod server_thread;

use std::net::{TcpStream};
use std::thread;
use std::str;
use std::io::{self, Read, Write};
use regex::Regex;
use crate::infrastructure::json_repository::get_relay;
use crate::models::{Clients};
use crate::server_manager::ServerManager;


fn main() -> io::Result<()> {
    let relay = get_relay();
    println!("Relay {}", relay.to_string());

    ServerManager::new(relay).expect("TODO: panic message");

    Ok(())
}
