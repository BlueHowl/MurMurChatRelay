mod models;

use std::net::{TcpListener, TcpStream, UdpSocket, ToSocketAddrs};
use std::thread;
use std::str;
use std::io::{self, Read};
use regex::Regex;
use std::path::Path;
use std::fs::File;
use crate::models::Relay;

fn handle_client(stream: TcpStream) {
    println!("New client: {}", stream.peer_addr().unwrap());
    // TODO: Implement your client handling logic here
}

fn main() -> io::Result<()> {

    let path = Path::new("./data/relay.json");
    let mut file  = File::open(path).expect("File not found");
    let mut contents = String::new();
    File::read_to_string(&mut file, &mut contents).expect("Something went wrong reading the file");
    let mut relay : Relay = serde_json::from_str(&contents).expect("JSON parsing error");
    println!("{} {} {} {}", &relay.get_multicast_address(), &relay.get_multicast_port(), &relay.get_network_interface(), &relay.get_network_interface());

    // Start UDP multicast listener on address 224.1.1.255:23845
    let multicast_socket = UdpSocket::bind("0.0.0.0:23502")?;
    let multicast_addr:std::net::SocketAddrV4 = "224.1.1.255:23502".parse().unwrap();
    multicast_socket.join_multicast_v4(&multicast_addr.ip(), &"0.0.0.0".parse().unwrap())?;

    // Start TCP server on 127.0.0.1:23847
    let tcp_listener = TcpListener::bind("127.0.0.1:23502")?;

    // Spawn thread to handle multicast messages
    thread::spawn(move || {
        let mut buf = [0; 1024];
        let regex = Regex::new(r"^ECHO[\x20](\d{1,5})[\x20]([a-zA-Z\d.]{5,200})").unwrap();
        loop {
            match multicast_socket.recv_from(&mut buf) {
                Ok((size, src)) => {
                    let msg = str::from_utf8(&buf[..size]).unwrap();
                    println!("Received multicast message from {}: {}", src, msg);

                    // Parse port and domain from multicast message using regex
                    match regex.captures(msg) {
                        Some(captures) => {
                            let port = captures.get(1).unwrap().as_str();
                            let domain = captures.get(2).unwrap().as_str();
                            println!("Port: {}, Domain: {}", port, domain);

                            // Connect to TCP server using unicast IP address and port
                            match TcpStream::connect(format!("{}:{}", domain, port)) {
                                Ok(stream) => {
                                    thread::spawn(move || {
                                        handle_client(stream);
                                    });
                                }
                                Err(e) => {
                                    println!("Error connecting to {}: {}", src.ip(), e);
                                }
                            }
                        }
                        None => {
                            println!("Invalid multicast message format: {}", msg);
                        }
                    }
                }
                Err(e) => {
                    println!("Error receiving multicast message: {}", e);
                }
            }
        }
    });

    // Wait for incoming TCP connections
    for stream in tcp_listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client: {}", stream.peer_addr().unwrap());
                // TODO: Add client to some kind of client list and handle communication
            }
            Err(e) => {
                println!("Error accepting client: {}", e);
            }
        }
    }

    Ok(())
}