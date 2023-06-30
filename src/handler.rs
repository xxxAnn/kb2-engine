use std::{net::TcpStream, io::{Read, Write}};

pub trait Handler {
    fn handle(&mut self, stream: &mut TcpStream) {
        let mut buf = [0; 255];

        stream.read(&mut buf).unwrap();

        stream.write("ACK".as_bytes()).unwrap();

        println!("Received: '{}'", std::str::from_utf8(&buf).unwrap());
    }
}

pub struct BaseHandler;

impl Handler for BaseHandler {}

impl BaseHandler {
    pub fn new() -> Self {
        Self {}
    }
}