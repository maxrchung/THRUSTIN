mod common;

#[test]
fn set_name() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.read_all();
    assert_eq!(client.last(1), String::from("Name set to: 1<br/>ok 1, now ur redy 2 THRUST, try \'.help\' for sum updated information"));
}

#[test]
fn rename() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".n player1");
    client.read_all();
    assert_eq!(client.last(1), String::from("Name set to: player1"));
}

#[test]
fn invalid_rename_in_lobby() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(1, ".n player1");
    client.read_all();
    assert_eq!(
        client.last(1),
        "Broski that shall be an invalid command. enter .help"
    );
}

#[test]
fn invalid_rename_in_game() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(1, ".s");
    client.send(1, ".n player1");
    client.read_all();
    assert_eq!(client.last(1), "Brother that is an invalid command.");
}

#[test]
fn duplicate_name_error() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(2, ".n 1");
    client.read_all();
    assert_eq!(
        client.last(2),
        "yo that name exists ya gotta pick something else aight?"
    );
}

#[test]
fn duplicate_rename_error() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(2, ".n 2");
    client.send(2, ".n 1");
    client.read_all();
    assert_eq!(
        client.last(2),
        "yo that name exists ya gotta pick something else aight?"
    );
}
