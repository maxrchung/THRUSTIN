mod common;
use thrustin::communication::Communication;

#[test]
fn setup_channels() {
    common::setup();
}

#[test]
fn send_message() {
    let client = common::setup();
    client.send_message(&1, "now this is an epic omegalul");
}

#[test]
fn read_message() {
    let mut client = common::setup();
    client.send_message(&1, "this is truly an epic achievement");
    let (token, msg) = client.read_message();
    assert_eq!(token, 1);
    assert!(msg.len() > 0);
}

#[test]
fn read_all() {
    let mut client = common::setup();
    client.send_message(&1, "this is truly an epic achievement");
    client.send_message(&1, "i love to eat hamburger");
    client.send_message(&1, "i love to eat swag");
    let (token, msg) = client.read_all();
    assert_eq!(token, 1);
    assert!(msg.len() > 0);
}

#[test]
fn read_multiple() {
    let mut client = common::setup();
    client.send_message(&1, "hey i'm the FIRST guy");
    client.send_message(&2, "YO I'm the SECOND dude");
    let (token, msg) = client.read_message();
    assert_eq!(token, 1);
    assert!(msg.len() > 0);
    let (token, msg) = client.read_message();
    assert_eq!(token, 2);
    assert!(msg.len() > 0);
}

#[test]
fn read_all_multiple() {
    let mut client = common::setup();
    client.send_message(&1, "hey i'm the FIRST guy");
    client.send_message(&2, "YO I'm the SECOND dude");
    let (token, msg) = client.read_all();
    assert_eq!(token, 2);
    assert!(msg.len() > 0);
}