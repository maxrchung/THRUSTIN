mod common;

#[test]
fn join_endless() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".j 0");
    client.read_all();
    assert!(client.last(1).contains("Welcome to the 『Endless Lobby』, big doggo. You lucky, family, you are THRUSTEE!!!!.. . Choose now...    .<br/>your THRUSTEE Choices:"));
}

#[test]
fn play_endless() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".p");
    client.read_all();
    assert!(client.last(1).contains("Welcome to the 『Endless Lobby』, big doggo. You lucky, family, you are THRUSTEE!!!!.. . Choose now...    .<br/>your THRUSTEE Choices:"));
}

#[test]
//Can join after default player cap is reached
fn no_endless_player_limit() {
    let mut client = common::setup();
    for client_name in 1..21 {
        client.send(client_name, &format!(".n {}", client_name));
        client.send(client_name, &format!(".p {}", client_name));
    }
    client.read_all();
    assert_eq!(client.last(20), "Joined: 0<br/>THRUSTEE is currently CHOOSING next THRUSTEE. Hold on tight!");
}

#[test]
// Can play after default THRUST cap is reached
fn todo_no_endless_point_limit() {
}