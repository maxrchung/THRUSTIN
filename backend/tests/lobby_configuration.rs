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
        ".tee 10",
        ".ter 10"
    ];
    for command in only_chief {
        client.send(2, command);
        client.read_all();
        assert!(client.last(2).to_lowercase().contains("only chief"));
    }
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
            0 (Total THRUSTEES) < 3 (Maximum THRUSTEE Choices)");
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
            1 (Total THRUSTEES) < 3 (Maximum THRUSTEE Choices)");
}

#[test]
fn fail_thruster_validation() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(1, ".ho");
    client.send(1, ".t \"Now this is _____\" \"Now this ain't _____\" \"What's swagging my _____\"");
    client.send(1, ".s");
    client.read_all();
    assert_eq!(client.last(1), "Yo... got an issue boss, we don't go enough THRUSTERS. Let me calculate to tell you why:<br/>\
            0 (Total THRUSTERS) < 5 (Maximum THRUSTER Choices) * 1 (Number Of People In Lobby)");
}

#[test]
fn fail_underscore_validation() {

}

#[test]
fn todo_no_thrusts() {}

#[test]
fn todo_appoint_chief() {}

#[test]
fn todo_players() {}

#[test]
fn todo_points() {}

#[test]
fn todo_password() {}

#[test]
fn only_chief_help_commands() {
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
