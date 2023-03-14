use std::io::{Read, Write};
use std::net::TcpStream;
use std::{str};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use regex::Regex;
use crate::aesgcm::AesGcmEncryptor;
use crate::models::{Domains};

pub(crate) struct ServerThread {
    domain: Domains,
    connected_servers: Arc<Mutex<HashMap<String, TcpStream>>>,
    domain_list: Vec<Domains>
}

impl ServerThread {

    pub fn new(domain: Domains, connected_servers: Arc<Mutex<HashMap<String, TcpStream>>>, domain_list: Vec<Domains>) -> ServerThread {
        ServerThread {
            domain,
            connected_servers,
            domain_list
        }

    }

    pub fn run(self, mut stream: TcpStream) {
        let aes_gcm = AesGcmEncryptor::new(self.domain.get_aeskey().clone());

        let mut buffer = [0; 1024];

        // Read data from the client and send it back
        loop {

            match stream.read(&mut buffer) {

                Ok(0) => {
                    println!("Client {} disconnected", stream.peer_addr().unwrap());
                    break;
                }

                Ok(n) => {
                    let message = str::from_utf8(&buffer[..n]).unwrap().to_string();

                    let uncrypted_msg = aes_gcm.decrypt_string(&message).unwrap();

                    println!("Received message from {}: {}", stream.peer_addr().unwrap(), uncrypted_msg.clone());

                    let regex = Regex::new(r"^SEND\x20\d{1,5}@[a-zA-Z\d.]{5,200}\x20[a-zA-Z\d]{5,20}@[a-zA-Z\d.]{5,200}\x20#?[a-zA-Z\d]{5,20}@(?P<domain>[a-zA-Z\d.]{5,200})\x20[\x20-\xFF]{1,500}$").unwrap();


                    match regex.captures(&*uncrypted_msg.trim()) {
                        Some(caps) => {
                            println!("Domain: {}", &caps["domain"]);
                            self.send(uncrypted_msg.clone(), caps["domain"].to_string());//, binding);
                        }

                        None => {
                            println!("No send match");
                        }
                    }

                }

                Err(e) => {
                    println!("Error reading from client: {}", e);
                    break;
                }
            }

        }

    }


    pub fn send(&self, message: String, domain: String) {//, binding: MutexGuard<HashMap<String, TcpStream>>) {
        let iter = self.domain_list.iter();
        for server in iter {
            if server.clone().get_domain().eq(domain.clone().as_str()) {

                let binding = self.connected_servers.lock().unwrap();
                let mut stream = binding.get(domain.clone().as_str()).unwrap();

                let aes = AesGcmEncryptor::new(server.clone().get_aeskey().clone());

                let encrypted_msg = format!("{}\n", aes.encrypt_string(message).unwrap());
                println!("Encrypted message : {}", encrypted_msg.clone());

                let bytes_written = stream.write(encrypted_msg.as_bytes()).unwrap();
                if bytes_written == encrypted_msg.len() {
                    println!("Message sent successfully to {}", stream.peer_addr().unwrap());
                } else {
                    println!("Error: only {} out of {} bytes written", bytes_written, encrypted_msg.len());
                }

                return
            }
        }

    }
}