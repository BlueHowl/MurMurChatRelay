mod models;
mod infrastructure;

use std::net::{TcpListener, TcpStream, UdpSocket, ToSocketAddrs};
use std::thread;
use std::str;
use std::io::{self, Read, Write};
use regex::Regex;
use std::path::Path;
use std::fs::File;
use crate::infrastructure::json_repository::get_relay;
use crate::models::{Domains, Relay};

fn handle_client(mut stream: TcpStream) {
    println!("New client: {}", stream.peer_addr().unwrap());
    let mut buffer = [0; 1024];

    // Read data from the client and send it back
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client {} disconnected", stream.peer_addr().unwrap());
                break;
            }
            Ok(n) => {
                let message = str::from_utf8(&buffer[..n]).unwrap();
                let regex = Regex::new(r"^SEND\x20\d{1,5}@[a-zA-Z\d.]{5,200}\x20[a-zA-Z\d]{5,20}@[a-zA-Z\d.]{5,200}\x20#?[a-zA-Z\d]{5,20}@(?<domain>[a-zA-Z\d.]{5,200})\x20[\x20-\xFF]{1,500}$").unwrap();
                println!("Received message from {}: {}", stream.peer_addr().unwrap(), message);
                loop {
                    match regex.captures(message) {
                        Some(caps) => {
                            println!("Domain: {}", &caps["domain"]);
                            send_message_to_client(message.to_string(), &caps["domain"]);
                            break;
                        }
                        None => {
                            println!("No match");
                            break;
                        }
                    }
                }
                // Echo the message back to the client
                stream.write_all(message.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("Error reading from client: {}", e);
                break;
            }
        }
    }
}

fn main() -> io::Result<()> {

    /*let path = Path::new("./data/relay.json");
    let mut file  = File::open(path).expect("File not found");
    let mut contents = String::new();
    File::read_to_string(&mut file, &mut contents).expect("Something went wrong reading the file");
    let mut relay : Relay = serde_json::from_str(&contents).expect("JSON parsing error");
    println!("{} {} {} {}", &relay.get_multicast_address(), &relay.get_multicast_port(), &relay.get_network_interface(), &relay.get_network_interface());
    */
    let connected_domains = new:Vec<>();
    let relay = get_relay();
    println!("Relay {} {} {} {}", relay.get_multicast_address().clone(), relay.get_multicast_port().clone(), relay.get_network_interface().clone(), relay.get_network_interface().clone());


    // Start UDP multicast listener on address 224.1.1.255:23845
    let multicast_socket = UdpSocket::bind("0.0.0.0:23502")?;
    let multicast_addr:std::net::SocketAddrV4 = "224.1.1.255:23502".parse().unwrap();
    multicast_socket.join_multicast_v4(&multicast_addr.ip(), &"0.0.0.0".parse().unwrap())?;

    // Start TCP server on 127.0.0.1:23847
    //let tcp_listener = TcpListener::bind("127.0.0.1:23502")?;

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
                            if is_domain_allowed(relay.get_configured_domains().clone(), domain.to_string()) {
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
                            } else {
                                println!("Domain {} is not allowed", domain);
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

    /* Wait for incoming TCP connections
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
    }*/

    Ok(())
}

// this method is used to verify if a client is connected to the server
fn is_domain_allowed(domains: Vec<Domains>, domainRequest: String) -> bool {
    let mut is_allowed = false;
    for domain in domains {
        if domain.get_domain_name() == domainRequest {
            is_allowed = true;
        }
    }
    is_allowed;
}

// this method is used to send the received message to the right client
fn send_message_to_client(message: String, domain: &str) {
    let mut stream = TcpStream::connect(format!("{}:23502", domain)).unwrap();
    stream.write_all(message.as_bytes()).unwrap();
}