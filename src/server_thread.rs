use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
use regex::Regex;

struct ServerThread {}

impl ServerThread {
    pub fn new(mut stream: TcpStream) {
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
                                send_message_to_client(message.to_string(), &caps["domain"], 23502);
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
}

// this method is used to send the received message to the right client
fn send_message_to_client(message: String, domain: &str, port: u16) {
    let mut stream = TcpStream::connect(format!("{}:{}", domain, port)).unwrap();
    stream.write_all(message.as_bytes()).unwrap();
}