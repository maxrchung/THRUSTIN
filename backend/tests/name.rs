mod common;
use common::FileSystemClient;

#[test]
fn name() {
    let id = "name";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    let msg = a.send_and_read(".name a");
    a.stop();
    assert_eq!(msg, String::from("Name set to: a<br/>ok a, now ur redy 2 THRUST, try \'.help\' for sum updated information"));
}

#[test]
fn rename() {
    let id = "rename";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.name();
    let msg = a.send_and_read(".n b");
    a.stop();
    assert_eq!(msg, String::from("Name set to: b"));
}

#[test]
fn invalid_rename_in_lobby() {
    let id = "rename_in_lobby";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.name();
    a.send_and_read(".m");
    let msg = a.send_and_read(".name b");
    assert_eq!(msg, "Broski that shall be an invalid command. enter .help");
    a.stop();
}

#[test]
fn invalid_rename_in_game() {
    let id = "leave_lobby";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.name();
    a.send_and_read(".m");
    a.send_and_read(".s");
    let msg = a.send_and_read(".name b");
    assert_eq!(msg, "Brother that is an invalid command.");
    a.stop();
}

#[test]
fn duplicate_name_error() {
    let id = "duplicate_name_error";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    let b = FileSystemClient::new(id, "a");
    a.name();
    let msg = b.name();
    a.stop();
    assert_eq!(msg, String::from("yo that name exists ya gotta pick something else aight?"));
}

#[test]
fn duplicate_rename_error() {
    let id = "duplicate_rename_error";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    let b = FileSystemClient::new(id, "b");
    a.name();
    b.name();
    let msg = b.send_and_read(".n a");
    a.stop();
    assert_eq!(msg, String::from("yo that name exists ya gotta pick something else aight?"));
}