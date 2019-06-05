mod common;

use common::FileSystemClient;

#[test]
fn make_lobby() {
    let id = "make_lobby";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.name();
    let msg = a.send_and_read(".m");
    a.stop();
    assert_eq!(msg, String::from("Created lobby: 1"));
}

#[test]
fn join_lobby() {
    let id = "join_lobby";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    let b = FileSystemClient::new(id, "b");
    a.name();
    b.name();
    a.send_and_read(".m");
    let msg = b.send_and_read(".j 1");
    assert_eq!(msg, String::from("Joined: 1"));
    let msg = a.read();
    assert_eq!(msg, "b has joined the lobby.");
    a.stop();
}