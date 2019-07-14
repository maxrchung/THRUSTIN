// database features

mod common;

#[test]
fn invalid_login() {
    let mut client = common::setup_with_db("invalid_login");
    client.send(1, ".l yowhat'sgood");
    client.read_all();
    assert_eq!(client.last(1), "You must provide USER and PASSWORD for your account.");
    client.send(1, ".l yowhat'sgood swagginout");
    client.read_all();
    assert_eq!(client.last(1), "Failed to login lol are you sure you know what you're doing?");
}

#[test]
fn invalid_register() {
    let mut client = common::setup_with_db("invalid_register");
    client.send(1, ".r yowhat'sgood doggy");
    client.read_all();
    assert_eq!(client.last(1), "Ok you've got an invalid number of parameters for registration.");
    client.send(1, ".r yowhat'sgood swagginout swagginour");
    client.read_all();
    assert_eq!(client.last(1), "Registration failed. The given password confirmation does not match the given password.");
}

#[test]
fn register() {
    let mut client = common::setup_with_db("register");
    client.send(1, ".r yo what what");
    client.read_all();
    assert_eq!(client.last(1), "Lol ok nice you registered and good to go.");
}

#[test]
fn existing_register() {
    let mut client = common::setup_with_db("existing_register");
    client.send(1, ".r yo what what");
    client.send(2, ".r yo what what");
    client.read_all();
    assert_eq!(client.last(2), "Registration has failed. Unable to add user to database. Maybe username isn't unique?");
}