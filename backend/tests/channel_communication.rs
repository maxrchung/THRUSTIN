// Testing channels client communication

mod common;

#[test]
fn setup_channels() {
    common::setup();
}

#[test]
fn send() {
    let mut client = common::setup();
    client.send(1, "now this is an epic omegalul");
}

#[test]
fn read_all() {
    let mut client = common::setup();
    client.send(1, "this is truly an epic achievement");
    client.read_all();
}

#[test]
fn last() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, "omegalul");
    client.read_all();
    assert_eq!(client.last(1), "omegalul");
    assert_eq!(client.last_from(1), "1");
    assert_eq!(client.last_bg(1), "b7410e");
    assert_eq!(client.last_fg(1), "000");
}

#[test]
fn read_all_multiple() {
    let mut client = common::setup();
    client.send(1, "hey i'm the FIRST guy");
    client.send(2, "YO I'm the SECOND dude");
    client.read_all();
    assert!(client.last(1).len() > 0);
    assert!(client.last(2).len() > 0);
}

#[test]
fn last_state() {
    let mut client = common::setup();
    client.send(1, "yo");
    client.read_all();
    assert_eq!(client.last_state(1), "ChooseName");

    client.send(1, ".n 1");
    client.read_all();
    assert_ne!(client.last_state(1), "ChooseName");
    assert_eq!(client.last_state(1), "OutOfLobby");

    client.send(1, ".m 1");
    client.read_all();
    assert_ne!(client.last_state(1), "OutOfLobby");
    assert_eq!(client.last_state(1), "InLobby");

    client.send(1, ".s");
    client.read_all();
    assert_ne!(client.last_state(1), "InLobby");
    assert_eq!(client.last_state(1), "Choosing");

    client.send(1, ".t 1");
    client.read_all();
    assert_ne!(client.last_state(1), "Choosing");
    assert_eq!(client.last_state(1), "Deciding");

}