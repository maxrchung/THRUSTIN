mod common;

use std::u8;
use std::usize;

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
fn endless_configurations() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".p");
    client.send(1, ".i");
    client.read_all();
    assert_eq!(
        client.last(1),
        format!(
            "\\\\Lobby info//<br/>Name: 0<br/>Players: 1 / {}<br/>Max points: {}",
            usize::MAX,
            u8::MAX
        )
    );
}
