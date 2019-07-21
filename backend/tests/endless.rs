// Tests specific for endless lobby or endless lobby interaction

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
    assert_eq!(client.last(1), "\\\\Lobby info//<br/>Name: 0<br/>Chief: EndlessLobbyHostDoggo<br/>Players: 1/18446744073709551615<br/>Max points? 255<br/>Use house THRUSTS? true<br/>THRUSTEES? 3<br/>THRUSTERS? 5");
}

// Bug: Panic occurrs when trying to join endless after leaving
// Reason: THRUSTEE was not being reset properly when new lobby starts
#[test]
fn join_after_a_round_is_played() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".p");
    client.send(2, ".n 2");
    client.send(2, ".p");
    client.send(1, ".t 1");
    client.thrust(2);
    client.send(1, ".t 1");
    client.send(1, ".l");
    client.send(2, ".l");
    client.send(1, ".p");
    client.send(2, ".p");
    client.read_all();
    assert_eq!(
        client.last(2),
        "Joined: 0<br/>THRUSTEE 1 is currently CHOOSING next THRUSTEE. Hold on tight!"
    );
}

// Bug: Panic occurs when trying to rejoin left endless lobby and choosing a THRUSTEE
#[test]
fn rejoin_after_choosing_thrustee() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".p");
    client.send(1, ".t 1");
    client.send(1, ".l");
    client.send(1, ".p");
    client.read_all();
    let msg = client.last(1);
    assert!(msg.contains("You lucky, family, you are THRUSTEE!!!!"));
    assert!(msg.contains("your THRUSTEE Choices:"));
    assert!(msg.contains("1. "));
    assert!(msg.contains("2. "));
    assert!(msg.contains("3. "));
}
