// Joining, leaving, starting lobbies

mod common;
use common::FileSystemClient;

#[test]
fn make_lobby() {
    let id = "make_lobby";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.name();
    let msg = a.send_and_read(".m");
    a.stop();
    assert_eq!(msg, String::from("Created lobby: 1"));
}

#[test]
fn join_lobby() {
    let id = "join_lobby";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    let b = FileSystemClient::new(id, "b");
    a.name();
    b.name();
    a.send_and_read(".m");
    let msg = b.send_and_read(".j 1");
    assert_eq!(msg, String::from("Joined: 1"));
    let msg = a.read();
    assert_eq!(msg, "b has joined the lobby.");
    a.stop();
}

#[test]
fn leave_lobby() {
    let id = "leave_lobby";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.name();
    a.send_and_read(".m");
    let b = FileSystemClient::new(id, "b");
    b.name();
    b.send_and_read(".j 1");
    a.read();
    let msg = b.send_and_read(".l");
    assert_eq!(msg, "You left the lobby okay!");
    let msg = a.read();
    assert_eq!(msg, "b left the lobby..");
    a.stop();
}

#[test]
fn start_lobby() {
    let id = "start_lobby";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.name();
    a.send_and_read(".m");
    let b = FileSystemClient::new(id, "b");
    b.name();
    b.send_and_read(".j 1");
    a.read();
    let msg = a.send_and_read(".s");
    assert!(msg.contains("You are the THRUSTEE. Choose NOW.........."));
    let msg = b.read();
    assert!(msg.contains("You are a THRUSTER. waiting for a good THRUSTEE; mmm baby!"));
    a.stop();
}

#[test]
fn leave_lobby_as_thruster() {
    let id = "leave_lobby_as_thruster";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.name();
    a.send_and_read(".m");
    let b = FileSystemClient::new(id, "b");
    b.name();
    b.send_and_read(".j 1");
    a.read();
    a.send_and_read(".s");
    b.read();
    let msg = b.send_and_read(".l");
    assert_eq!(msg, "You left the lobby okay!");
    let msg = a.read();
    assert_eq!(msg, "b left the lobby..");
    a.stop();
}

#[test]
fn leave_lobby_as_thrustee() {
    let id = "leave_lobby_as_thrustee";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.name();
    a.send_and_read(".m");
    let b = FileSystemClient::new(id, "b");
    b.name();
    b.send_and_read(".j 1");
    a.read();
    a.send_and_read(".s");
    b.read();
    let msg = a.send_and_read(".l");
    assert_eq!(msg, "You left the lobby okay!");
    let msg = b.read();
    assert!(msg.contains("a left the lobby..<br/>Chief left so now we got a new one --> b<br/>Lol yo bro 'cause the THRUSTEE left b is choosin' the next THRUSTEE now!<br/><br/>your THRUSTEE Choices:<br/>"));
    a.stop();
}

#[test]
fn kick_in_lobby() {
    let id = "kick_in_lobby";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.name();
    a.send_and_read(".m");
    let b = FileSystemClient::new(id, "b");
    b.name();
    b.send_and_read(".j 1");
    a.read();
    let msg = a.send_and_read(".k b");
    assert_eq!(msg, "b left the lobby..");
    let msg = b.read();
    assert_eq!(msg, "You left the lobby okay!");
    a.stop();
}

#[test]
fn kick_in_game() {
    let id = "kick_in_game";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.name();
    a.send_and_read(".m");
    let b = FileSystemClient::new(id, "b");
    b.name();
    b.send_and_read(".j 1");
    a.read();
    a.send_and_read(".s");
    b.read();
    let msg = a.send_and_read(".k b");
    assert_eq!(msg, "b left the lobby..");
    let msg = b.read();
    assert_eq!(msg, "You left the lobby okay!");
    a.stop();
}

#[test]
fn kick_thrustee() {
    let id = "kick_thrustee";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.name();
    a.send_and_read(".m");
    let b = FileSystemClient::new(id, "b");
    b.name();
    b.send_and_read(".j 1");
    a.read();
    a.send_and_read(".s");
    b.read();
    a.send_and_read(".t 0");
    b.read();
    b.send_and_read(".t 1");
    a.read();
    a.send_and_read(".t 0");
    b.read();
    let msg = a.send_and_read(".k b");
    assert!(msg.contains("b left the lobby.."));
    let msg = b.read();
    assert_eq!(msg, "You left the lobby okay!");
    a.stop();
}
