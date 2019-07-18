// Tests for game flow and interaction

mod common;

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
    assert!(msg.contains("1. "));
    assert!(msg.contains("2. "));
    assert!(msg.contains("3. "));
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

        client.send(thrustee, ".t 1");
        client.read_all();
        let msg = client.last(thrustee);
        assert!(msg.contains(&format!("{} has chosen this new THRUSTEE:", thrustee)));
        assert!(msg.contains("get Ready to decide best THRUSTER for THRUSTING!"));
        let msg = client.last(thruster);
        assert!(msg.contains(&format!("{} has chosen this new THRUSTEE:", thrustee)));
        assert!(msg.contains("Here are your THRUSTERS:"));
        // Should not have more than 5 options
        assert!(!msg.contains("6. "));

        client.thrust(thruster);
        client.read_all();
        assert!(client.last(thrustee).contains("1. "));

        client.send(thrustee, ".t 1");
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
        assert!(msg.contains("1. "));
        assert!(msg.contains("2. "));
        assert!(msg.contains("3. "));
    }
}

// Bug: Zero index panics in debug
// Reason: Index input is parsed into unsigned type and subtraction goes into negative
#[test]
fn out_of_range_thrust() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".p");
    client.send(1, ".t 0");
    client.read_all();
    assert_eq!(client.last(1), "That shit's out of bound bro");
    client.send(1, ".t 6");
    client.read_all();
    assert_eq!(client.last(1), "That shit's out of bound bro");
    client.send(1, ".t 1");

    client.send(2, ".n 2");
    client.send(2, ".p");
    // Not easy to test thruster index because of random shuffling
    client.thrust(2);

    client.send(1, ".t 0");
    assert_eq!(client.last(1), "That shit's out of bound bro");
    client.read_all();
    assert_eq!(client.last(1), "That shit's out of bound bro");
    client.send(1, ".t 6");
    client.read_all();
    assert_eq!(client.last(1), "That shit's out of bound bro");
}

#[test]
fn cannot_thrust_again() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".p");
    client.send(2, ".n 2");
    client.send(2, ".p");
    client.send(1, ".t 1");
    client.thrust(2);
    client.thrust(2);
    client.read_all();
    assert_eq!(
        client.last(2),
        "You have already THRUSTED, you cannot THRUST again."
    );
}

#[test]
fn shows_correct_index_after_thrusting() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".p");
    client.send(2, ".n 2");
    client.send(2, ".p");
    client.send(3, ".n 3");
    client.send(3, ".p");

    client.send(1, ".t 1");
    client.thrust(2);
    client.read_all();
    assert!(client.last(1).contains("1. "));
    assert!(client.last(3).contains("1. "));

    client.thrust(3);
    client.read_all();
    assert!(client.last(1).contains("2. "));
    assert!(client.last(2).contains("2. "));
}
