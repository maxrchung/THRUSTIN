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
    client.send(1, ".m 1");
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
    client.send(1, ".m 1");
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
        .contains("You are a THRUSTER. waiting for a good THRUSTEE; mmm baby!"));
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
    client.send(1, ".t 0");
    client.thrust(2);
    client.send(1, ".t 0");
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
fn next_thrustee_no_chief_display() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".p");
    client.send(2, ".n 2");
    client.send(2, ".p");
    client.send(1, ".l");
    client.read_all();
    let msg = client.last(2);
    assert!(msg.contains("your THRUSTEE Choices:"));
    assert!(msg.contains("<br/>0. "));
    assert!(msg.contains("<br/>1. "));
    assert!(msg.contains("<br/>2. "));
}

#[test]
fn next_thrustee_chief_display() {
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
    assert!(msg.contains("<br/>0. "));
    assert!(msg.contains("<br/>1. "));
    assert!(msg.contains("<br/>2. "));
}

#[test]
fn thrust_back_and_forth() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(2, ".n 2");
    client.send(2, ".j 1");
    client.send(1, ".s");
    client.read_all();
    let msg = client.last(1);
    assert!(msg.contains("You are the THRUSTEE. Choose NOW.........."));
    assert!(msg.contains("your THRUSTEE Choices:"));
    assert!(msg.contains("0. "));
    assert!(msg.contains("1. "));
    assert!(msg.contains("2. "));
    // If THRUSTER was in lobby when game was started
    assert_eq!(
        client.last(2),
        "You are a THRUSTER. waiting for a good THRUSTEE; mmm baby!"
    );
    client.send(2, ".l");
    client.send(2, ".j 1");
    client.read_all();
    // If THRUSTER joined midgame
    assert_eq!(
        client.last(2),
        "Joined: 1<br/>THRUSTEE is currently CHOOSING next THRUSTEE. Hold on tight!"
    );

    for n in 0..2 {
        let (thrustee, thruster) = if n == 0 { (1, 2) } else { (2, 1) };

        client.send(thrustee, ".t 0");
        client.read_all();
        let msg = client.last(thrustee);
        assert!(msg.contains(&format!("{} has chosen this new THRUSTEE:", thrustee)));
        assert!(msg.contains("get Ready to decide best THRUSTER for THRUSTING!"));
        let msg = client.last(thruster);
        assert!(msg.contains(&format!("{} has chosen this new THRUSTEE:", thrustee)));
        assert!(msg.contains("Here are your THRUSTERS:"));
        // Should not have more than 5 options
        assert!(!msg.contains("5. "));

        client.thrust(thruster);
        client.read_all();
        assert!(client.last(thrustee).contains("0. "));

        client.send(thrustee, ".t 0");
        client.read_all();
        let msg = client.last(thrustee);
        assert!(msg.contains(&format!(
            "{} has chosen this THRUSTER as the chosen THRUST, bois:",
            thrustee
        )));
        assert!(msg.contains(&format!(
            "The winning THRUSTER, {} now has 1/7 point(s)! Watch out!",
            thruster
        )));
        assert!(msg.contains("get rdy to THRUST....."));
        let msg = client.last(thruster);
        assert!(msg.contains(&format!(
            "{} has chosen this THRUSTER as the chosen THRUST, bois:",
            thrustee
        )));
        assert!(msg.contains(&format!(
            "The winning THRUSTER, {} now has 1/7 point(s)! Watch out!",
            thruster
        )));
        assert!(msg.contains("You are the neXt THRUSTEE! GetT ready to CHOOSE a good THRUSTEE!"));
        assert!(msg.contains("your THRUSTEE Choices:"));
        assert!(msg.contains("0. "));
        assert!(msg.contains("1. "));
        assert!(msg.contains("2. "));
    }
}
