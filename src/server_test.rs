use super::*;

#[test]
fn ip_address_is_stored_in_server() {
    let server = Server::new("127.0.0.1".to_string());
    assert_eq!(server.address, "127.0.0.1");
}
