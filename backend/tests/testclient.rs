mod common;
use common::TestClient;

// This function is always necessary to create test server to interact with
// Be careful not to mix state between tests
#[test]
fn setup() {
    common::run_test_server();
}

#[test]
fn make_new_client() {
}

#[test]
fn connect_client() {
    
}