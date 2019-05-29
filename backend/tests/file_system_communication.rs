mod common;

use common::FileSystemClient;
use std::path::{Path};

#[test]
fn start_and_stop_server() {
    let id = "start_and_stop_server";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");

    a.stop();
    assert!(!Path::new(&id).exists());
}

#[test]
fn send_client_message() {
    let id = "send_client_message";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.send_message("Now this is epic.");
    a.stop();
}

#[test]
fn read_client_message() {
    let id = "read_client_message";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.send_message("Now this is epic.");
    let msg = a.read_message();
    assert!(msg.len() > 0);

    a.stop();
}

#[test]
fn send_and_read_client_message() {
    let id = "send_and_read_client_message";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    let msg = a.send_and_read_message("Now this is epic.");
    assert!(msg.len() > 0);

    a.stop();
}

#[test]
fn read_multiple_client_messages() {
    let id = "read_multiple_client_messages";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.send_message("Now this is epic.");
    let b = FileSystemClient::new(id, "b");
    b.send_message(".name yoloSW4G420000000000000");

    let msg = b.read_message();
    assert!(msg.len() > 0);
    let msg = a.read_message();
    assert!(msg.len() > 0);

    a.stop();
}

#[test]
fn name_client_message() {
    let id = "name_client_message";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    let msg = a.name();
    assert!(msg.len() > 0);

    a.stop();
}