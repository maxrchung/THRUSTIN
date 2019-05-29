mod common;

use common::FileSystemClient;
use std::path::{Path};

#[test]
fn enter_valid_name() {
    let id = "enter_valid_name";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    let msg = a.name();
    assert_eq!(msg, String::from("Name set to: a<br/>ok a, now ur redy 2 THRUST, try \'.help\' for sum updated information"));

    a.stop();
}