use super::*;

#[test]
fn this_test_will_pass() {
    let server = Server::new("127.0.0.1".to_string());
    assert_eq!(server.address, "127.0.0.1");
}
