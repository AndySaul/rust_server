use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    pub address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("listening on {}", self.address);
        match TcpListener::bind(&self.address) {
            Err(e) => println!("{}", e),
            Ok(mut listener) => loop {
                wait_for_connection(&mut handler, &mut listener);
            },
        }
    }
}

pub fn wait_for_connection(handler: &mut impl Handler, listener: &mut TcpListener) {
    match listener.accept() {
        Err(e) => println!("Failed to establish connection: {}", e),

        Ok((mut stream, _)) => handle_request(handler, &mut stream),
    }
}

pub fn handle_request(handler: &mut impl Handler, stream: &mut TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Err(e) => println!("Failed to read from connection: {}", e),

        Ok(_) => {
            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

            let response = match Request::try_from(&buffer[..]) {
                Ok(request) => handler.handle_request(&request),
                Err(e) => handler.handle_bad_request(&e),
            };

            if let Err(e) = response.send(stream) {
                println!("Failed to send response: {}", e);
            }
        }
    }
}
