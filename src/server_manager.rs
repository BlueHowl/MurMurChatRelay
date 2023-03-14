use std::{io, thread, str};
use std::collections::HashMap;
use std::net::{TcpStream, UdpSocket};
use std::sync::{Arc, Mutex};
use regex::Regex;
use std::string::String;
use crate::models::{Domains, Relay};
use crate::server_thread::ServerThread;

pub struct ServerManager {
    connected_servers: Arc<Mutex<HashMap<String, TcpStream>>>,
}

impl ServerManager {

    pub fn new() -> ServerManager {
        ServerManager {
            connected_servers: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn start_listening(self, relay: Relay, multicast_adress: String) -> io::Result<()> {
        println!("Multicast interface Address: {}", multicast_adress);
        // Start UDP multicast listener on address 224.1.1.255:23502
        let multicast_socket = UdpSocket::bind(format!("{}:{}", multicast_adress, relay.get_multicast_port()))?;
        let multicast_addr: std::net::SocketAddrV4 = format!("{}:{}", relay.get_multicast_address(), relay.get_multicast_port()).parse().unwrap();
        multicast_socket.join_multicast_v4(&multicast_addr.ip(), &multicast_adress.as_str().parse().unwrap())?;


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
                                    let domain_obj = get_domain(relay.get_configured_domains().clone(),domain.to_string());

                                    // Connect to TCP server using unicast IP address and port
                                    match TcpStream::connect(format!("{}:{}", domain, port)) {

                                        Ok(stream) => {

                                            println!("New client: {}", stream.try_clone().unwrap().peer_addr().unwrap());

                                            let server_thread = ServerThread::new(domain_obj.clone(), Arc::clone(&self.connected_servers), relay.get_configured_domains().clone());
                                            let thread_stream = stream.try_clone().unwrap();
                                            thread::spawn(move || server_thread.run(thread_stream));

                                            let mut servers = self.connected_servers.lock().unwrap();
                                            servers.insert(domain_obj.clone().get_domain(),stream.try_clone().unwrap());

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

pub fn get_domain(domains: Vec<Domains>, domain_request: String) -> Domains {

    for domain in domains {
        let d = domain.clone();
        if domain.get_domain().clone() == domain_request {
            return d;
        }
    }

    return Domains::new("".to_string(), "".to_string());
}

