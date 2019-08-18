// Tests for configuring lobby settings

mod common;

#[test]
fn only_chief_commands() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(2, ".n 2");
    client.send(2, ".j 1");

    let only_chief = vec![
        ".c 2",
        ".ho 69",
        ".k 1",
        ".pw yoloswag",
        ".po 420",
        ".s",
        ".e 10",
        ".r 10",
    ];
    for command in only_chief {
        client.send(2, command);
        client.read_all();
        assert!(client.last(2).to_lowercase().contains("only chief"));
    }
}

#[test]
fn only_chief_shows_help_commands() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(2, ".n 2");
    client.send(2, ".j 1");
    client.send(1, ".h");
    client.send(2, ".h");
    client.read_all();

    assert!(!client.last(2).to_lowercase().contains("chief-only"));
    assert!(client.last(1).to_lowercase().contains("chief-only"));
}

#[test]
fn fail_thrustee_validation() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(1, ".ho 0");
    client.send(1, ".s");
    client.read_all();
    assert_eq!(client.last(1), "Dude, I can't start the game for you because yall don't got enough THRUSTEES. Here's a lil bit of mathematics:<br/>Total THRUSTEES HAS to BE GREATER THAN 0");
}

#[test]
fn fail_thruster_validation() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(1, ".ho 0");
    client.send(1, ".t \"Now this is _____\"");
    client.send(
        1,
        ".t \"Now this is _____\" \"Now this ain't _____\" \"What's swagging my _____\"",
    );
    client.send(1, ".s");
    client.read_all();
    assert_eq!(client.last(1), "Yo... got an issue boss, we don't go enough THRUSTERS. Let me calculate to tell you why:<br/>Total THRUSTERS HAS to BE GREATER THAN 0");
}

#[test]
fn appoint_another_chief() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(2, ".n swag");
    client.send(2, ".j 1");
    client.send(1, ".c swag");
    client.read_all();
    assert_eq!(client.last(1), "swag is now chief!");
    assert_eq!(client.last(2), "You are now chief!");
    client.send(1, ".s");
    client.read_all();
    assert_eq!(client.last(1), "Only chief can start game!");
    client.send(2, ".s");
    client.read_all();
    assert_eq!(
        client.last(2),
        "You are a THRUSTER. waiting for a good THRUSTEE from 1; mmm baby!"
    );
    assert!(client
        .last(1)
        .contains("You are the THRUSTEE. Choose NOW.........."));
}

#[test]
fn set_player_max() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(1, ".pl 2");
    client.send(2, ".n swag");
    client.send(2, ".j 1");
    client.send(3, ".n swagger");
    client.send(3, ".j 1");
    client.read_all();
    assert_eq!(client.last(1), "swag has joined the lobby.");
    assert_eq!(client.last(2), "Joined: 1");
    assert_eq!(client.last(3), "bro this lobbBY is FULLLLL!!");
}

#[test]
fn set_points_max() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(1, ".po 1");
    client.send(2, ".n 2");
    client.send(2, ".j 1");
    client.send(1, ".s");
    client.send(1, ".t 1");
    client.thrust(2);
    client.send(1, ".t 1");
    client.read_all();
    assert!(client
        .last(1)
        .contains("1 has chosen this THRUSTER as the chosen THRUST, bois:"));
    assert!(client.last(1).contains("Congratulations, 2! You're Winner! Everyone else, You're Loser! Game has been put into waiting state, THRUSTIN'ers!"));
    assert!(client
        .last(2)
        .contains("1 has chosen this THRUSTER as the chosen THRUST, bois:"));
    assert!(client.last(2).contains("Congratulations, 2! You're Winner! Everyone else, You're Loser! Game has been put into waiting state, THRUSTIN'ers!"));
}

#[test]
fn set_password() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(1, ".pw lololol");
    client.read_all();
    assert_eq!(
        client.last(1),
        "Now, the password has now been locked and loaded, my dude, now it's: lololol"
    );
    client.send(2, ".n 2");
    client.send(2, ".j 1");
    client.read_all();
    assert_eq!(client.last(2), "Ya need a password BR)");
    client.send(2, ".j 1 lololol");
    client.read_all();
    assert_eq!(client.last(1), "2 has joined the lobby.");
    assert_eq!(client.last(2), "Joined: 1");
}

#[test]
fn default_lobby_configuration() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(1, ".i");
    client.read_all();
    assert_eq!(client.last(1), "\\\\Lobby info//<br/>ID: 1<br/>***(Only chief [that\'s you!] may see this!) Password: <br/>Chief: 1<br/>Players: 1/10<br/>Max points? 7<br/>House THRUSTS? 420<br/>THRUSTEES? 3<br/>THRUSTERS? 5");
}

#[test]
fn make_new_lobby_with_password() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m 1 1 1 1 1");
    client.read_all();
    assert_eq!(
        client.last(1),
        "Yo you gotta give the right parameters into .make bro!"
    );

    client.send(1, ".m TesT1N6!!!!!");
    client.send(2, ".n 2");
    client.send(2, ".j 1 TesT1N6!!!!!");
    client.read_all();
    assert_eq!(client.last(2), "Joined: 1");
}
