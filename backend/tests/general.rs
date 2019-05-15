mod common;
use common::Client;
use thrustin;

#[test]
fn new_client() {
    thrustin::run();
    Client::new();
}