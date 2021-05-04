use super::*;
use crate::http::{Request, Response, StatusCode};
use std::io::ErrorKind;

struct FakeHandler {}

impl FakeHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl server::Handler for FakeHandler {
    fn handle_request(&mut self, _request: &Request) -> Response {
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
