// Adding, removing, updating THRUSTS

mod common;

#[test]
fn add_thruster() {
    let mut client = common::setup_with_db("add_thruster");
    client.send(1, ".r brother 1 1");
    client.send(1, ".t \"It's yo boy epic swagger!!\"");
    client.read_all();
    assert_eq!(
        client.last(1),
        "Added to THRUSTERS:<br/>1. It's yo boy epic swagger!!"
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
    let mut client = common::setup_with_db("add_thrustee");
    client.send(1, ".r broletmein 1 1");
    client.send(1, ".t \"It's yo boy, _ swagger!!\"");
    client.read_all();
    assert_eq!(
        client.last(1),
        "Added to THRUSTEES:<br/>1. It's yo boy, _ swagger!!"
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
    let mut client = common::setup_with_db("unthrust");
    client.send(1, ".r superepicman 1 1");
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
    let mut client = common::setup_with_db("multiple_thrusts");
    client.send(1, ".r unepicman 1 1");
    client.send(1, ".t \"It's yo boy swaggy swagger!!\" \"Now what is up swagger\" \"It's yo boy ___\" \"Now what is up ____\"");
    client.read_all();
    assert_eq!(client.last(1), "Added to THRUSTEES:<br/>1. It's yo boy ___<br/>2. Now what is up ____<br/><br/>Added to THRUSTERS:<br/>1. It's yo boy swaggy swagger!!<br/>2. Now what is up swagger");
    client.send(1, ".t");
    client.read_all();
    assert_eq!(client.last(1), "You're THRUSTEES:<br/>1. It's yo boy ___<br/>2. Now what is up ____<br/><br/>You're THRUSTERS:<br/>1. It's yo boy swaggy swagger!!<br/>2. Now what is up swagger");
}

#[test]
fn no_quotations_thrust() {
    let mut client = common::setup_with_db("no_quotations_thrust");
    client.send(1, ".r brotherman 1 1");
    client.send(1, ".t lol");
    client.read_all();
    assert_eq!(
        client.last(1),
        "No THRUST arguments found. Did you forget quotations? Try something like .t \"Hello there!\""
    );
}
