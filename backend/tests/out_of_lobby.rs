mod common;

use common::FileSystemClient;

#[test]
fn make_lobby() {
    let id = "make_lobby";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.name();
    let msg = a.send_and_read(".m");
    assert_eq!(msg, String::from("Created lobby: 1"));

    a.stop();
}

#[test]
fn join_endless() {
    let id = "join_endless";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.name();
    let msg = a.send_and_read(".m");
    assert_eq!(msg, String::from("Created lobby: 1"));

    a.stop();
}

#[test]
fn name_help() {

}