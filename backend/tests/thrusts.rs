mod common;

#[test]
fn thruster() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".t \"It's yo boy epic swagger!!\"");
    client.read_all();
    assert_eq!(client.last(1), "Added \"It's yo boy epic swagger!!\" to THRUSTERS!");
    client.send(1, ".t");
    client.read_all();
    assert_eq!(client.last(1), "You're THRUSTEES:<br/><br/>You're THRUSTERS:<br/>1. It's yo boy epic swagger!!");
}

#[test]
fn thrustee() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".t \"It's yo boy, _ swagger!!\"");
    client.read_all();
    assert_eq!(client.last(1), "Added \"It's yo boy, _ swagger!!\" to THRUSTEES!");
    client.send(1, ".t");
    client.read_all();
    assert_eq!(client.last(1), "You're THRUSTEES:<br/>1. It's yo boy, _ swagger!!<br/><br/>You're THRUSTERS:");
}

#[test]
fn unthrust() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".t \"It's yo boy swaggy swagger!!\"");
    client.send(1, ".t \"It's yo boy super swagger!!\"");
    client.send(1, ".t");
    client.read_all();
    assert_eq!(client.last(1), "You're THRUSTEES:<br/><br/>You're THRUSTERS:<br/>1. It's yo boy super swagger!!<br/>2. It's yo boy swaggy swagger!!");
    client.send(1, ".u");
    client.read_all();
    assert_eq!(client.last(1), "Personal THRUSTS have been cleared! If this was an accident, Good Luck!");
}