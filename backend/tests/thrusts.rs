
mod common;
use common::FileSystemClient;

#[test]
fn thruster() {
    let id = "thruster";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.name();
    let msg = a.send_and_read(".t \"It's yo boy epic swagger!!\"");
    assert_eq!(msg, "Added \"It's yo boy epic swagger!!\" to THRUSTERS!");
    let msg = a.send_and_read(".t");
    assert_eq!(msg, "You're THRUSTEES:<br/><br/>You're THRUSTERS:<br/>1. It's yo boy epic swagger!!");
    a.stop();
}

#[test]
fn thrustee() {
    let id = "thrustee";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.name();
    let msg = a.send_and_read(".t \"It's yo boy, _ swagger!!\"");
    assert_eq!(msg, "Added \"It's yo boy, _ swagger!!\" to THRUSTEES!");
    let msg = a.send_and_read(".t");
    assert_eq!(msg, "You're THRUSTEES:<br/>1. It's yo boy, _ swagger!!<br/><br/>You're THRUSTERS:");
    a.stop();
}

#[test]
fn unthrust() {
    let id = "unthrust";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.name();
    a.send_and_read(".t \"It's yo boy swaggy swagger!!\"");
    a.send_and_read(".t \"It's yo boy super swagger!!\"");
    let msg = a.send_and_read(".t");
    // THRUSTS are sorted
    assert_eq!(msg, "You're THRUSTEES:<br/><br/>You're THRUSTERS:<br/>1. It's yo boy super swagger!!<br/>2. It's yo boy swaggy swagger!!");
    let msg = a.send_and_read(".u");
    assert_eq!(msg, "Personal THRUSTS have been cleared! If this was an accident, Good Luck!");
    a.stop();
}