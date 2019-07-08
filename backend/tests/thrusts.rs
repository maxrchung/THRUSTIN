// Adding, removing, updating THRUSTS

mod common;

#[test]
fn add_thruster() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".t \"It's yo boy epic swagger!!\"");
    client.read_all();
    assert_eq!(
        client.last(1),
        "Added \"It's yo boy epic swagger!!\" to THRUSTERS!"
    );
    client.send(1, ".t");
    client.read_all();
    assert_eq!(
        client.last(1),
        "You're THRUSTEES:<br/><br/>You're THRUSTERS:<br/>1. It's yo boy epic swagger!!"
    );
}

#[test]
fn add_thrustee() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".t \"It's yo boy, _ swagger!!\"");
    client.read_all();
    assert_eq!(
        client.last(1),
        "Added \"It's yo boy, _ swagger!!\" to THRUSTEES!"
    );
    client.send(1, ".t");
    client.read_all();
    assert_eq!(
        client.last(1),
        "You're THRUSTEES:<br/>1. It's yo boy, _ swagger!!<br/><br/>You're THRUSTERS:"
    );
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
    assert_eq!(
        client.last(1),
        "Personal THRUSTS have been cleared! If this was an accident, Good Luck!"
    );
}

#[test]
fn multiple_thrust() {
    let mut client = common::setup();
    client.send(1, ".n 1");
    client.send(1, ".t \"It's yo boy swaggy swagger!!\" \"Now what is up swagger\" \"It's yo boy ___\" \"Now what is up ____\"");
    client.send(1, ".t");
    client.read_all();
    assert_eq!(client.last(1), "You're THRUSTEES:<br/>1. It's yo boy ___<br/>2. Now what is up ____<br/><br/>You're THRUSTERS:<br/>1. It's yo boy swaggy swagger!!<br/>2. Now what is up swagger");
}
