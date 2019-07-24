// Joining, leaving, starting lobbies

mod common;

#[test]
fn make_lobby() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.read_all();
    assert_eq!(client.last(1), String::from("Created lobby: 1"));
}

#[test]
fn join_lobby() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(2, ".n 2");
    client.send(2, ".j 1");
    client.read_all();
    assert_eq!(client.last(2), String::from("Joined: 1"));
    assert_eq!(client.last(1), "2 has joined the lobby.");
}

#[test]
fn leave_lobby() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(2, ".n 2");
    client.send(2, ".j 1");
    client.send(2, ".l");
    client.read_all();
    assert_eq!(
        client.last(2),
        String::from("You have been leaved from the lobby okay!")
    );
    assert_eq!(client.last(1), "2 left the lobby..");
}

#[test]
fn start_lobby() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(2, ".n 2");
    client.send(2, ".j 1");
    client.send(1, ".s");
    client.read_all();
    assert!(client
        .last(1)
        .contains("You are the THRUSTEE. Choose NOW.........."));
    assert!(client
        .last(2)
        .contains("You are a THRUSTER. waiting for a good THRUSTEE from 1; mmm baby!"));
}

#[test]
fn leave_lobby_as_thruster() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(2, ".n 2");
    client.send(2, ".j 1");
    client.send(1, ".s");
    client.send(2, ".l");
    client.read_all();
    assert_eq!(client.last(2), "You have been leaved from the lobby okay!");
    assert_eq!(client.last(1), "2 left the lobby..");
}

#[test]
fn leave_lobby_as_thrustee() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(2, ".n 2");
    client.send(2, ".j 1");
    client.send(1, ".s");
    client.send(1, ".l");
    client.read_all();
    assert_eq!(client.last(1), "You have been leaved from the lobby okay!");
    assert!(client.last(2).contains("1 left the lobby..<br/>Chief left so now we got a new one --> 2<br/>Lol yo bro 'cause the THRUSTEE left 2 is choosin' the next THRUSTEE now!<br/><br/>your THRUSTEE Choices:<br/>"));
}

#[test]
fn kick_in_lobby() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(2, ".n 2");
    client.send(2, ".j 1");
    client.send(1, ".k 2");
    client.read_all();
    assert_eq!(client.last(1), "2 left the lobby..");
    assert_eq!(client.last(2), "You have been leaved from the lobby okay!");
}

#[test]
fn kick_in_game() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(2, ".n 2");
    client.send(2, ".j 1");
    client.send(1, ".s");
    client.send(1, ".k 2");
    client.read_all();
    assert_eq!(client.last(1), "2 left the lobby..");
    assert_eq!(client.last(2), "You have been leaved from the lobby okay!");
}

#[test]
fn kick_thrustee() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(2, ".n 2");
    client.send(2, ".j 1");
    client.send(1, ".s");
    client.send(1, ".t 1");
    client.thrust(2);
    client.send(1, ".t 1");
    client.send(1, ".k 2");
    client.read_all();
    assert!(client.last(1).contains("2 left the lobby.."));
    assert_eq!(client.last(2), "You have been leaved from the lobby okay!");
}

#[test]
fn who_out_of_lobby() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".w");
    client.read_all();
    assert_eq!(client.last(1), "1 (You)");
    client.send(2, ".n 2");
    client.send(2, ".w");
    client.read_all();
    assert_eq!(client.last(2), "1<br/>2 (You)");
}

#[test]
fn who_in_lobby() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(1, ".w");
    client.read_all();
    assert_eq!(client.last(1), "1: chief (You)");
    client.send(2, ".n 2");
    client.send(2, ".j 1");
    client.send(2, ".w");
    client.read_all();
    assert_eq!(client.last(2), "1: chief<br/>2 (You)");
}

// Bug: If THRUSTEE chooser is not a host and leaves, next user is unable to see THRUSTEE choices to pick
// Reason: THRUSTEE was improperly being chosen based on who was chief
#[test]
fn new_thrustee_is_not_chief() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".p");
    client.send(2, ".n 2");
    client.send(2, ".p");
    client.send(1, ".l");
    client.read_all();
    let msg = client.last(2);
    assert!(msg.contains("your THRUSTEE Choices:"));
    assert!(msg.contains("<br/>1. "));
    assert!(msg.contains("<br/>2. "));
    assert!(msg.contains("<br/>3. "));
}

#[test]
fn new_thrustee_is_chief() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(1, ".s");
    client.send(2, ".n 2");
    client.send(2, ".j 1");
    client.send(1, ".l");
    client.read_all();
    let msg = client.last(2);
    assert!(msg.contains("your THRUSTEE Choices:"));
    assert!(msg.contains("<br/>1. "));
    assert!(msg.contains("<br/>2. "));
    assert!(msg.contains("<br/>3. "));
}

#[test]
fn end_midgame() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(2, ".n 2");
    client.send(2, ".j 1");
    client.send(1, ".s");
    client.send(2, ".e");
    client.read_all();
    assert_eq!(
        client.last(2),
        "Only chief shall have the privilege to end the game."
    );
    client.send(1, ".e");
    client.read_all();
    assert_eq!(client.last(1), "Yo guys, the game's been manually ended by the chief almighty. Yall have been returned to the lobby setup area.");
    assert_eq!(client.last(2), "Yo guys, the game's been manually ended by the chief almighty. Yall have been returned to the lobby setup area.");
}

#[test]
fn out_of_lobby_list() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".l");
    client.read_all();
    assert_eq!(
        client.last(1),
        "A current exploration of lobbies that are available to be joined into is as follows below. Simply `.join [ID]` to enter. Lobby 0 is an endless lobby. It's always gonna be there.<br/>ID: 0 | Password: ❌ | Players: 0/18446744073709551615 | Currently: Playing"
    );

    client.send(2, ".n 2");
    client.send(2, ".m");
    client.send(1, ".l");
    client.read_all();
    assert_eq!(client.last(1), "A current exploration of lobbies that are available to be joined into is as follows below. Simply `.join [ID]` to enter. Lobby 0 is an endless lobby. It's always gonna be there.<br/>ID: 0 | Password: ❌ | Players: 0/18446744073709551615 | Currently: Playing<br/>ID: 1 | Password: ❌ | Players: 1/10 | Currently: Waiting");

    client.send(2, ".pa yolo");
    client.send(1, ".l");
    client.read_all();
    assert_eq!(client.last(1), "A current exploration of lobbies that are available to be joined into is as follows below. Simply `.join [ID]` to enter. Lobby 0 is an endless lobby. It's always gonna be there.<br/>ID: 0 | Password: ❌ | Players: 0/18446744073709551615 | Currently: Playing<br/>ID: 1 | Password: ✅ | Players: 1/10 | Currently: Waiting");

    client.send(3, ".n 3");
    client.send(3, ".j 1 yolo");
    client.send(2, ".s");
    client.send(1, ".l");
    client.read_all();
    assert_eq!(client.last(1), "A current exploration of lobbies that are available to be joined into is as follows below. Simply `.join [ID]` to enter. Lobby 0 is an endless lobby. It's always gonna be there.<br/>ID: 0 | Password: ❌ | Players: 0/18446744073709551615 | Currently: Playing<br/>ID: 1 | Password: ✅ | Players: 2/10 | Currently: Playing");
}

#[test]
fn shows_updated_lobby_list_when_login() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.read_all();
    assert_eq!(
        client.last(1),
        "Name set to: 1<br/>ok 1, now ur redy 2 THRUST, try \'.help\' for sum updated information<br/><br/>A current exploration of lobbies that are available to be joined into is as follows below. Simply `.join [ID]` to enter. Lobby 0 is an endless lobby. It's always gonna be there.<br/>ID: 0 | Password: ❌ | Players: 0/18446744073709551615 | Currently: Playing"
    );

    client.send(1, ".m");
    client.send(2, ".n 2");
    client.read_all();
    assert_eq!(
        client.last(2),
        "Name set to: 2<br/>ok 2, now ur redy 2 THRUST, try \'.help\' for sum updated information<br/><br/>A current exploration of lobbies that are available to be joined into is as follows below. Simply `.join [ID]` to enter. Lobby 0 is an endless lobby. It's always gonna be there.<br/>ID: 0 | Password: ❌ | Players: 0/18446744073709551615 | Currently: Playing<br/>ID: 1 | Password: ❌ | Players: 1/10 | Currently: Waiting"
    );
}