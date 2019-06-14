mod common;

use common::FileSystemClient;

#[test]
fn join_endless() {
    let id = "join_endless";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.name();
    let msg = a.send_and_read(".j 0");
    a.stop();
    assert!(msg.contains("Welcome to the 『Endless Lobby』, big doggo. You lucky, family, you are THRUSTEE!!!!.. . Choose now...    .<br/>your THRUSTEE Choices:"));
}

#[test]
fn play_endless() {
    let id = "play_endless";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.name();
    let msg = a.send_and_read(".p");
    a.stop();
    assert!(msg.contains("Welcome to the 『Endless Lobby』, big doggo. You lucky, family, you are THRUSTEE!!!!.. . Choose now...    .<br/>your THRUSTEE Choices:"));
}

#[test]
// Can play after default THRUST cap is reached
fn todo_no_endless_point_limit() {
    panic!();
}

#[test]
// Can join after default player cap is reached
fn no_endless_player_limit() {
    let id = "no_endless_player_limit";
    common::run_test_server(id);
    let mut msg = String::new();
    for client_name in 10..100 {
        let client = FileSystemClient::new(id, &client_name.to_string());
        msg = client.name();
        assert_eq!(msg, client_name.to_string());

        msg = client.send_and_read(".p");

    }
    let last = FileSystemClient::new(id, "last");
    last.name();
    let msg = last.send_and_read(".w");
    last.stop();
    assert_eq!(msg, "lool");
    assert!(msg.contains("Joined: 0"));

}