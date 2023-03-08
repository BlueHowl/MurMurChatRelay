use std::{io, thread, str};
use std::net::{TcpStream, UdpSocket};
use std::sync::{Arc, Mutex};
use regex::Regex;
use crate::models::{Domains, Relay};
use crate::server_thread::ServerThread;

pub struct ServerManager {
    connected_domains: Arc<Mutex<Vec<ServerThread>>>,
}

impl ServerManager {

    pub fn new() -> ServerManager {
        ServerManager {
            connected_domains: Arc::new(Mutex::new(Vec::new()))
        }
    }
    pub fn start_listening(self, relay: Relay) -> io::Result<()> {
        // Start UDP multicast listener on address 224.1.1.255:23502
        let multicast_socket = UdpSocket::bind(format!("0.0.0.0:{}", relay.get_multicast_port()))?;
        let multicast_addr: std::net::SocketAddrV4 = format!("{}:{}", relay.get_multicast_address(), relay.get_multicast_port()).parse().unwrap();
        multicast_socket.join_multicast_v4(&multicast_addr.ip(), &"0.0.0.0".parse().unwrap())?;


        // Spawn thread to handle multicast messages
        let handle = thread::spawn(move || {

            let mut buffer = [0; 1024];
            let regex = Regex::new(r"^ECHO[\x20](\d{1,5})[\x20]([a-zA-Z\d.]{5,200})").unwrap();

            loop {

                match multicast_socket.recv_from(&mut buffer) {

                    Ok((size, src)) => {

                        let msg = str::from_utf8(&buffer[..size]).unwrap();
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

                                            let server_thread = ServerThread::new();
                                            let mut connected_domains = self.connected_domains.lock().unwrap();
                                            connected_domains.push(server_thread);
                                            server_thread.run(stream);
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

        handle.join().unwrap();

        Ok(())
    }
}

// this method is used to verify if a client is connected to the server
pub fn is_domain_allowed(domains: Vec<Domains>, domain_request: String) -> bool {
    let mut is_allowed = false;
    for domain in domains {
        if domain.get_domain().clone() == domain_request {
            is_allowed = true;
        }
    }
    is_allowed
}