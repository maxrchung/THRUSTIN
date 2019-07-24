// database features

mod common;

#[test]
fn invalid_login() {
    let mut client = common::setup_with_db("invalid_login");
    client.send(1, ".l yowhat'sgood");
    client.long_read_all();
    assert_eq!(
        client.last(1),
        "You must provide USER and PASSWORD for your account."
    );
    client.send(1, ".l yowhat'sgood swagginout");
    client.long_read_all();
    assert_eq!(
        client.last(1),
        "Failed to login lol are you sure you know what you're doing?"
    );
}

#[test]
fn invalid_register() {
    let mut client = common::setup_with_db("invalid_register");
    client.send(1, ".r yowhat'sgood doggy");
    client.long_read_all();
    assert_eq!(
        client.last(1),
        "Ok you've got an invalid number of parameters for registration."
    );
    client.send(1, ".r yowhat'sgood swagginout swagginour");
    client.long_read_all();
    assert_eq!(
        client.last(1),
        "Registration failed. The given password confirmation does not match the given password."
    );
}

#[test]
fn register() {
    let mut client = common::setup_with_db("register");
    client.send(1, ".r yo what what");
    client.long_read_all();
    assert_eq!(client.last(1), "Lol ok nice you registered and good to go.<br/><br/>A current exploration of lobbies that are available to be joined into is as follows below. Simply `.join [ID]` to enter. Lobby 0 is an endless lobby. It's always gonna be there.<br/>ID: 0 | Password: ❌ | Players: 0/18446744073709551615 | Currently: Playing");
}

#[test]
fn existing_register() {
    let mut client = common::setup_with_db("existing_register");
    client.send(1, ".r yo what what");
    client.send(2, ".r yo what what");
    client.long_read_all();
    assert_eq!(
        client.last(2),
        "Registration has failed. Unable to add user to database. Maybe username isn't unique?"
    );
}

#[test]
fn register_and_login() {
    let mut client = common::setup_with_db("register_and_login");
    client.send(1, ".r yo what what");
    client.send(1, ".disconnect");
    client.send(2, ".l yo what");
    client.long_read_all();
    assert_eq!(
        client.last(2),
        "Welcome back ([USER] >>>\"yo\"<<< [USER]) to THRUSTIN.<br/><br/>A current exploration of lobbies that are available to be joined into is as follows below. Simply `.join [ID]` to enter. Lobby 0 is an endless lobby. It's always gonna be there.<br/>ID: 0 | Password: ❌ | Players: 0/18446744073709551615 | Currently: Playing"
    );
}

#[test]
fn name_checks_database() {
    let mut client = common::setup_with_db("name_checks_database");
    client.send(1, ".r SWAGGINGi'mSWAGGINGOUT yo yo");
    client.send(2, ".n SWAGGINGi'mSWAGGINGOUT");
    client.long_read_all();
    assert_eq!(
        client.last(2),
        "yo that name exists ya gotta pick something else aight?"
    );
}

#[test]
fn thrust_database() {
    let mut client = common::setup_with_db("thrust_database");
    client.send(1, ".r 1 1 1");
    client.send(1, ".t \"Yo what's up\" \"Hey, it's _____.\"");
    client.send(2, ".l 1 1");
    client.send(2, ".t");
    client.long_read_all();
    assert_eq!(
        client.last(2),
        "You're THRUSTEES:<br/>1. Hey, it's _____.<br/><br/>You're THRUSTERS:<br/>1. Yo what's up",
    );
    client.send(2, ".u");
    client.send(3, ".l 1 1");
    client.send(3, ".t");
    client.long_read_all();
    assert_eq!(
        client.last(3),
        "You're THRUSTEES:<br/><br/>You're THRUSTERS:",
    );
}

#[test]
fn change_user_and_pass() {
    let mut client = common::setup_with_db("change_user_and_pass");
    client.send(1, ".r 1 1 1");
    client.send(2, ".l 1 1");
    client.send(2, ".us 1.5 1.5");
    client.send(2, ".pa 1.5 1.5");
    client.send(3, ".l 1.5 1");
    client.long_read_all();
    assert_eq!(
        client.last(3),
        "Failed to login lol are you sure you know what you're doing?"
    );
    client.send(3, ".l 1 1.5");
    client.long_read_all();
    assert_eq!(
        client.last(3),
        "Failed to login lol are you sure you know what you're doing?"
    );
    client.send(3, ".l 1.5 1.5");
    client.long_read_all();
    assert_eq!(
        client.last(3),
        "Welcome back ([USER] >>>\"1\"<<< [USER]) to THRUSTIN.<br/><br/>A current exploration of lobbies that are available to be joined into is as follows below. Simply `.join [ID]` to enter. Lobby 0 is an endless lobby. It's always gonna be there.<br/>ID: 0 | Password: ❌ | Players: 0/18446744073709551615 | Currently: Playing"
    );
}
