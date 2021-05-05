use super::*;
use crate::http::{Request, Response, StatusCode};
use std::io::{Error, ErrorKind};

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
    let connection = Err(Error::new(ErrorKind::NotConnected, "oh no!"));

    server::wait_for_connection(&mut handler, connection);

    assert_eq!(handler.was_called, false);
}
