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
        ".ho",
        ".k 1",
        ".pa yoloswag",
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
fn fail_start_validation() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(1, ".ho");
    client.send(1, ".s");
    client.read_all();
    assert_eq!(client.last(1), "Dude, I can't start the game for you because yall don't got enough THRUSTEES. Here's a lil bit of mathematics:<br/>\
            0 (Total THRUSTEES) < 3 (THRUSTEE Choices)");
}

#[test]
fn fail_thrustee_validation() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(1, ".ho");
    client.send(1, ".t \"Now this is _____\"");
    client.send(1, ".s");
    client.read_all();
    assert_eq!(client.last(1), "Dude, I can't start the game for you because yall don't got enough THRUSTEES. Here's a lil bit of mathematics:<br/>\
            1 (Total THRUSTEES) < 3 (THRUSTEE Choices)");
}

#[test]
fn fail_thruster_validation() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(1, ".ho");
    client.send(
        1,
        ".t \"Now this is _____\" \"Now this ain't _____\" \"What's swagging my _____\"",
    );
    client.send(1, ".s");
    client.read_all();
    assert_eq!(client.last(1), "Yo... got an issue boss, we don't go enough THRUSTERS. Let me calculate to tell you why:<br/>\
            0 (Total THRUSTERS) < 5 (THRUSTER Choices) * 1 (Number Of People In Lobby)");
}

#[test]
fn fail_underscore_validation() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(1, ".t \"Now _\" \"Now _ _ _ _ _ _ _ _\" \"Now _ _\"");
    client.send(1, ".s");
    client.read_all();
    assert_eq!(client.last(1), "Hello, I am unable to start the game. This is because there is a THRUSTEE that requires too many THRUSTERS. Allow me to explain through geometry:<br/>\
            5 (THRUSTER Choices) < 8 (THRUSTERS For A THRUSTEE)");
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
        "You are a THRUSTER. waiting for a good THRUSTEE; mmm baby!"
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
        .contains("2 has chosen this THRUSTER as the chosen THRUST, bois:"));
    assert!(client.last(1).contains("Congratulations, 2! You're Winner! Everyone else, You're Loser! Game has been put into waiting state, THRUSTIN'ers!"));
    assert!(client
        .last(2)
        .contains("2 has chosen this THRUSTER as the chosen THRUST, bois:"));
    assert!(client.last(2).contains("Congratulations, 2! You're Winner! Everyone else, You're Loser! Game has been put into waiting state, THRUSTIN'ers!"));
}

#[test]
fn set_password() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(1, ".pa lololol");
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
