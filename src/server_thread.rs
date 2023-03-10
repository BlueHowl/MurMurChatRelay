use std::io::{Read, Write};
use std::net::TcpStream;
use std::{str, thread};
use std::sync::{Arc, Mutex};
use regex::Regex;
use crate::aesgcm::AesGcmEncryptor;
use crate::models::Domains;

#[derive(Clone)]
pub(crate) struct ServerThread {
    server: Domains,
    stream: Option<Arc<Mutex<TcpStream>>>
}

impl ServerThread {

    pub fn new(server: Domains) -> ServerThread {
        ServerThread {
            server,
            stream: None
        }

    }

    pub fn run(&mut self, mut stream: TcpStream) {
        self.stream = Option::from(Arc::new(Mutex::new(stream)));
        let aes_gcm = AesGcmEncryptor::new(self.server.get_aeskey().clone());

        let handle = thread::spawn(move || {
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
                        let message = str::from_utf8(&buffer[..n]).unwrap().to_string();

                        let uncrypted_msg = aes_gcm.decrypt_string(&message).unwrap();

                        println!("Received message from {}: {}", stream.peer_addr().unwrap(), uncrypted_msg.clone());

                        let regex = Regex::new(r"^SEND\x20\d{1,5}@[a-zA-Z\d.]{5,200}\x20[a-zA-Z\d]{5,20}@[a-zA-Z\d.]{5,200}\x20#?[a-zA-Z\d]{5,20}@(?P<domain>[a-zA-Z\d.]{5,200})\x20[\x20-\xFF]{1,500}$").unwrap();


                        match regex.captures(&*uncrypted_msg.trim()) {
                            Some(caps) => {
                                println!("Domain: {}", &caps["domain"]);
                                //TODO
                                break;
                            }

                            None => {
                                println!("No send match");
                                break;
                            }
                        }

                    }

                    Err(e) => {
                        println!("Error reading from client: {}", e);
                        break;
                    }
                }
            }
        });

        handle.join().unwrap();
    }

    pub fn get_server(&self) -> Domains {
        return self.server.clone()
    }

    pub fn send(&self, string: String) {
        let aes = AesGcmEncryptor::new(self.server.get_aeskey().clone());
        if let Some(stream) = &self.stream {
            let tcp_stream = stream.lock().unwrap();
            println!("Sending message to {}", tcp_stream.peer_addr().unwrap());
        } else {
            println!("Erreur stream");
        }
    }
}