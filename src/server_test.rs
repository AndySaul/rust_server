use super::*;
use crate::http::{Request, Response, StatusCode};
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::net::SocketAddr;

struct FakeHandler {
    was_called: bool,
}

impl FakeHandler {
    pub fn new() -> Self {
        Self { was_called: false }
    }
}

impl server::Handler for FakeHandler {
    fn handle_request(&mut self, _request: &Request) -> Response {
        self.was_called = true;
        Response::new(StatusCode::Ok, Some("dummy".to_string()))
    }
}

struct FakeStream {
    request: String,
    read_was_called: bool,
    write_was_called: bool,
}

impl FakeStream {
    pub fn new(request: String) -> Self {
        Self {
            request: request,
            read_was_called: false,
            write_was_called: false,
        }
    }
}

impl Read for FakeStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.read_was_called = true;
        let mut b = buf;
        b.write(self.request.as_bytes())
    }
}
impl Write for FakeStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.write_was_called = true;
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

struct FakeListener {}

impl FakeListener {
    pub fn request(request: String) -> Result<(FakeStream, SocketAddr)> {
        Ok((
            FakeStream::new(request),
            SocketAddr::from(([127, 0, 0, 1], 8080)),
        ))
    }
    pub fn connection_error() -> Result<(FakeStream, SocketAddr)> {
        Err(Error::new(ErrorKind::NotConnected, "oh no!"))
    }
}

#[test]
fn ip_address_is_stored_in_server() {
    let server = Server::new("127.0.0.1".to_string());

    assert_eq!(server.address, "127.0.0.1");
}

#[test]
fn does_not_run_with_invalid_ip_address() {
    let handler = FakeHandler::new();
    let server = Server::new("not an ip address".to_string());

    let error = server.run(handler).unwrap_err();

    assert_eq!(error.kind(), ErrorKind::InvalidInput);
}

#[test]
fn handler_is_not_called_when_there_is_a_connection_error() {
    let mut handler = FakeHandler::new();

    server::wait_for_connection(&mut handler, FakeListener::connection_error());

    assert_eq!(handler.was_called, false);
}

#[test]
fn handler_is_called_when_there_is_a_valid_request() {
    let mut handler = FakeHandler::new();

    server::wait_for_connection(
        &mut handler,
        FakeListener::request("GET / HTTP/1.1\r\n".to_string()),
    );

    assert_eq!(handler.was_called, true);
}

#[test]
fn handler_is_not_called_when_there_is_an_invalid_request() {
    let mut handler = FakeHandler::new();

    server::wait_for_connection(
        &mut handler,
        FakeListener::request("GET / ".to_string()),
    );

    assert_eq!(handler.was_called, false);
}
