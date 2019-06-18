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
fn todo_house() {}

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
