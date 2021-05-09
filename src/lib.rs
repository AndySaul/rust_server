pub mod http;
pub mod website_handler;

use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::{Read, Result, Write};
use std::net::{SocketAddr, TcpListener};

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

    pub fn run(self, mut handler: impl Handler) -> Result<()> {
        println!("listening on {}", self.address);

        let listener = TcpListener::bind(&self.address)?;

        loop {
            wait_for_connection(&mut handler, listener.accept());
        }
    }
}

pub fn wait_for_connection(
    handler: &mut impl Handler,
    result: Result<(impl Read + Write, SocketAddr)>,
) {
    match result {
        Err(e) => println!("Failed to establish connection: {}", e),

        Ok((mut stream, _)) => handle_request(handler, &mut stream),
    }
}

pub fn handle_request(handler: &mut impl Handler, mut stream: impl Read + Write) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Err(e) => println!("Failed to read from connection: {}", e),

        Ok(_) => {
            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

            let response = match Request::try_from(&buffer[..]) {
                Ok(request) => handler.handle_request(&request),
                Err(e) => handler.handle_bad_request(&e),
            };

            if let Err(e) = response.send(&mut stream) {
                println!("Failed to send response: {}", e);
            }
        }
    }
}
