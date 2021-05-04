use super::*;
use std::io::ErrorKind;

#[test]
fn ip_address_is_stored_in_server() {
    let server = Server::new("127.0.0.1".to_string());

    assert_eq!(server.address, "127.0.0.1");
}

#[test]
fn does_not_run_with_invalid_ip_address() {
    let server = Server::new("not an ip address".to_string());

    let error = server.run(Website::new("".to_string())).unwrap_err();

    assert_eq!(error.kind(), ErrorKind::InvalidInput);
}
