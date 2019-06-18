mod common;

#[test]
fn setup_channels() {
    common::setup();
}

#[test]
fn send() {
    let client = common::setup();
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
    client.send(1, "this is truly an epic achievement");
    client.read_all();
    assert!(client.last(1).len() > 0);
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
