// database features

mod common;

#[test]
fn invalid_login() {
    let mut client = common::setup_with_db("invalid_login");
    client.send(1, ".l yowhat'sgood");
    client.read_all();
    assert_eq!(
        client.last(1),
        "You must provide USER and PASSWORD for your account."
    );
    client.send(1, ".l yowhat'sgood swagginout");
    client.read_all();
    assert_eq!(
        client.last(1),
        "Failed to login lol are you sure you know what you're doing?"
    );
}

#[test]
fn invalid_register() {
    let mut client = common::setup_with_db("invalid_register");
    client.send(1, ".r yowhat'sgood doggy");
    client.read_all();
    assert_eq!(
        client.last(1),
        "Ok you've got an invalid number of parameters for registration."
    );
    client.send(1, ".r yowhat'sgood swagginout swagginour");
    client.read_all();
    assert_eq!(
        client.last(1),
        "Registration failed. The given password confirmation does not match the given password."
    );
}

#[test]
fn register() {
    let mut client = common::setup_with_db("register");
    client.send(1, ".r yo what what");
    client.read_all();
    assert_eq!(client.last(1), "Lol ok nice you registered and good to go.<br/><br/>A current exploration of lobbies that are available to be joined into is as follows below. Simply `.join [ID]` to enter. Lobby 0 is an endless lobby. It's always gonna be there.<br/>ID: 0 | Password: ❌ | Players: 0/18446744073709551615 | Currently: Playing");
}

#[test]
fn existing_register() {
    let mut client = common::setup_with_db("existing_register");
    client.send(1, ".r yo what what");
    client.send(2, ".r yo what what");
    client.read_all();
    assert_eq!(
        client.last(2),
        "Registration has failed. Unable to add user to database. Maybe username isn't unique?"
    );
}

#[test]
fn register_and_login() {
    let mut client = common::setup_with_db("register_and_login");
    client.send(1, ".r yo what what");
    client.send(2, ".l yo what");
    client.read_all();
    assert_eq!(
        client.last(2),
        "Welcome back ([]>>>\"yo\"<<<[]) to THRUSTIN.<br/><br/>A current exploration of lobbies that are available to be joined into is as follows below. Simply `.join [ID]` to enter. Lobby 0 is an endless lobby. It's always gonna be there.<br/>ID: 0 | Password: ❌ | Players: 0/18446744073709551615 | Currently: Playing"
    );
}

#[test]
fn name_checks_database() {
    let mut client = common::setup_with_db("name_checks_database");
    client.send(1, ".r SWAGGINGi'mSWAGGINGOUT yo yo");
    client.send(2, ".n SWAGGINGi'mSWAGGINGOUT");
    client.read_all();
    assert_eq!(
        client.last(2),
        "yo that name exists ya gotta pick something else aight?"
    );
}

#[test]
fn thrust_database() {
    let mut client = common::setup_with_db("thrust_database");
    client.send(1, ".r 1 1 1");
    client.send(1, ".t \"Yo what's up\" \"Hey, it's _____.\"");
    client.send(2, ".l 1 1");
    client.send(2, ".t");
    client.read_all();
    assert_eq!(
        client.last(2),
        "You're THRUSTEES:<br/>1. Hey, it's _____.<br/><br/>You're THRUSTERS:<br/>1. Yo what's up",
    );
    client.send(2, ".u");
    client.send(3, ".l 1 1");
    client.send(3, ".t");
    client.read_all();
    assert_eq!(
        client.last(3),
        "You're THRUSTEES:<br/><br/>You're THRUSTERS:",
    );
}

#[test]
fn change_user_and_pass() {
    let mut client = common::setup_with_db("change_user_and_pass");
    client.send(1, ".r 1 1 1");
    client.send(2, ".l 1 1");
    client.send(2, ".un 1.5 1.5");
    client.send(2, ".pw 1.5 1.5");
    client.send(3, ".l 1.5 1");
    client.read_all();
    assert_eq!(
        client.last(3),
        "Failed to login lol are you sure you know what you're doing?"
    );
    client.send(3, ".l 1 1.5");
    client.read_all();
    assert_eq!(
        client.last(3),
        "Failed to login lol are you sure you know what you're doing?"
    );
    client.send(3, ".l 1.5 1.5");
    client.read_all();
    assert_eq!(
        client.last(3),
        "Welcome back ([]>>>\"1\"<<<[]) to THRUSTIN.<br/><br/>A current exploration of lobbies that are available to be joined into is as follows below. Simply `.join [ID]` to enter. Lobby 0 is an endless lobby. It's always gonna be there.<br/>ID: 0 | Password: ❌ | Players: 0/18446744073709551615 | Currently: Playing"
    );
}

#[test]
fn view_account() {
    let mut client = common::setup_with_db("view_account");
    client.send(1, ".n 1");
    client.send(1, ".a");
    client.read_all();
    assert_eq!(client.last(1), "You cannot do this. You must be fully authenticated and logged in in order to get your account info with a registered account.");

    client.send(2, ".r user2 2 2");
    client.send(2, ".a");
    client.read_all();
    assert_eq!(client.last(2), "A display of your account information and statistical information. Please enjoy THRUSTIN!<br/>Username - user2<br/>Name - user2<br/>Password - [ENCRYPTED_CONTENT__UNVIEWABLE]<br/>Points Earned So Far - 0<br/>Games Played So Far - 0<br/>Games Won So Far - 0");
}

#[test]
fn update_account_stats() {
    let mut client = common::setup_with_db("update_account_stats");
    client.send(1, ".r 1 1 1");
    client.send(1, ".m");
    client.send(1, ".po 1");
    client.send(1, ".s");
    client.send(1, ".a");
    client.read_all();
    assert_eq!(client.last(1), "A display of your account information and statistical information. Please enjoy THRUSTIN!<br/>Username - 1<br/>Name - 1<br/>Password - [ENCRYPTED_CONTENT__UNVIEWABLE]<br/>Points Earned So Far - 0<br/>Games Played So Far - 1<br/>Games Won So Far - 0");

    client.send(2, ".r 2 2 2");
    client.send(2, ".j 1");
    client.send(2, ".a");
    client.read_all();
    assert_eq!(client.last(2), "A display of your account information and statistical information. Please enjoy THRUSTIN!<br/>Username - 2<br/>Name - 2<br/>Password - [ENCRYPTED_CONTENT__UNVIEWABLE]<br/>Points Earned So Far - 0<br/>Games Played So Far - 1<br/>Games Won So Far - 0");

    client.send(1, ".t 1");
    client.thrust(2);
    client.send(1, ".t 1");
    client.read_all();

    client.send(1, ".a");
    client.send(2, ".a");
    client.read_all();
    assert_eq!(client.last(1), "A display of your account information and statistical information. Please enjoy THRUSTIN!<br/>Username - 1<br/>Name - 1<br/>Password - [ENCRYPTED_CONTENT__UNVIEWABLE]<br/>Points Earned So Far - 0<br/>Games Played So Far - 1<br/>Games Won So Far - 0");
    assert_eq!(client.last(2), "A display of your account information and statistical information. Please enjoy THRUSTIN!<br/>Username - 2<br/>Name - 2<br/>Password - [ENCRYPTED_CONTENT__UNVIEWABLE]<br/>Points Earned So Far - 1<br/>Games Played So Far - 1<br/>Games Won So Far - 1");
}

#[test]
fn chieftain() {
    let mut client = common::setup_with_db_and_logging("chieftain");
    client.send(1, ".l chieftain chieftain");
    client.send(1, ".ct");
    client.read_all();
    assert_eq!(client.last(1), "A LIST OF CHIEFTAINS RESPONSIBLE FOR MANAGEMENT OF THIS THRUSTIN SERVER IS AS FOLLOWS.<br/>chieftain");

    client.send(2, ".n 2");
    client.send(2, ".ct");
    client.read_all();
    assert_eq!(
        client.last(2),
        "Yo dawg, this command can only be used by chieftains of THRUSTIN."
    );

    client.send(1, ".ct 2 2");
    client.read_all();
    assert_eq!(
        client.last(1),
        "Hey Chieftain, you should know what you're doing. Invalid indexes bro."
    );

    // Can't appoint someone who doesn't exist
    client.send(1, ".ct 3");
    client.read_all();
    assert_eq!(client.last(1), "FAILED TO APPOINT CHIEFTAIN: 3");

    // Can't appoint someone who isn't in database
    client.send(1, ".ct 2");
    client.read_all();
    assert_eq!(client.last(1), "FAILED TO APPOINT CHIEFTAIN: 2");

    client.send(3, ".r 3 3 3");
    client.send(1, ".ct 3");
    client.read_all();
    assert_eq!(client.last(1), "A NEW CHIEFTAIN HAS BEEN APPOINTED: 3");

    client.send(3, ".ct");
    client.read_all();
    assert_eq!(client.last(3), "A LIST OF CHIEFTAINS RESPONSIBLE FOR MANAGEMENT OF THIS THRUSTIN SERVER IS AS FOLLOWS.<br/>3<br/>chieftain");

    client.send(4, ".n 4");
    client.send(4, ".uc");
    client.read_all();
    assert_eq!(
        client.last(4),
        "Yo dawg, this command can only be used by chieftains of THRUSTIN."
    );

    client.send(3, ".uc");
    client.read_all();
    assert_eq!(
        client.last(3),
        "Hey Chieftain, you should know what you're doing. Invalid indexes bro."
    );

    client.send(3, ".uc blah");
    client.read_all();
    assert_eq!(
        client.last(3),
        "It looks like something went wrong with unchieftaining. Maybe blah isn't real?"
    );

    client.send(3, ".uc chieftain");
    client.read_all();
    assert_eq!(
        client.last(3),
        "Congratulations, you have unchieftained chieftain."
    );

    client.send(1, ".ct");
    client.read_all();
    assert_eq!(
        client.last(1),
        "Yo dawg, this command can only be used by chieftains of THRUSTIN."
    );
}

#[test]
fn ban_update() {
    let mut client = common::setup_with_db("ban_update");
    client.send(1, ".n a");
    client.send(1, ".b");
    client.read_all();
    assert_eq!(
        client.last(1),
        "Yo dawg, this command can only be used by chieftains of THRUSTIN."
    );

    client.send(2, ".l chieftain chieftain");
    client.send(2, ".b yep yep");
    client.read_all();
    assert_eq!(
        client.last(2),
        "Hey Chieftain, you should know what you're doing. Invalid indexes bro."
    );

    client.send(2, ".b");
    client.read_all();
    assert_eq!(client.last(2), "Banned fellows from this server. Kill'em.");

    client.send(2, ".b 123.123.123.123");
    client.read_all();
    assert_eq!(
        client.last(2),
        "IP address 123.123.123.123 has been banned."
    );

    client.send(2, ".b");
    client.read_all();
    assert!(client
        .last(2)
        .contains("Banned fellows from this server. Kill'em."));
    assert!(client.last(2).contains("123.123.123.123"));

    client.send(1, ".ub");
    client.read_all();
    assert_eq!(
        client.last(1),
        "Yo dawg, this command can only be used by chieftains of THRUSTIN."
    );

    client.send(2, ".ub");
    client.read_all();
    assert_eq!(
        client.last(2),
        "Hey Chieftain, you should know what you're doing. Invalid indexes bro."
    );

    client.send(2, ".ub 1.1.1.1");
    client.read_all();
    assert_eq!(
        client.last(2),
        "Failed to unban 1.1.1.1. Something went wrong. Unexpected error."
    );

    client.send(2, ".ub 123.123.123.123");
    client.read_all();
    assert_eq!(
        client.last(2),
        "The target 123.123.123.123 has been unbanned."
    );
}

// Not really too reliable since this is only with ChannelCommunication
// WebSocketCommunication probably has many more possible closing cases that I hope will be fine
#[test]
fn ban() {
    let mut client = common::setup_with_db("ban");
    client.send(1, ".n 1");
    client.send(1, ".m");
    client.send(1, ".s");
    client.send(2, ".l chieftain chieftain");
    client.send(2, ".b 1");
    client.send(1, "What's up my dudes?");
    client.read_all();
    assert!(client
        .last(1)
        .contains("You cannot exist. You are banned until "));
}

#[test]
fn color() {
    let mut client = common::setup_with_db("color");
    client.send(1, ".n 1");
    client.send(1, ".c 000");
    client.read_all();
    assert_eq!(client.last(1), "Invalid parameters to color.");

    client.send(1, ".c 000 111 222");
    client.read_all();
    assert_eq!(client.last(1), "Invalid parameters to color.");

    client.send(1, ".c 000 000");
    client.read_all();
    assert_eq!(
        client.last(1),
        "Excuse me, you can't assign your colors to the same one, that makes it too hard to see."
    );

    client.send(1, ".c 000 b7410e");
    client.read_all();
    assert_eq!(client.last(1), "Um, I'm gonna disallow you from choosing this color combination. It's mine, and I feel my identity being threatened if you choose this.");

    client.send(1, ".c 000 111");
    client.read_all();
    assert_eq!(
        client.last(1),
        "Awesome, we successfully set your chat colors to 000 (bg) and 111 (fg)."
    );

    client.send(2, ".r 2 2 2");
    client.send(2, ".c 000 111");
    client.read_all();
    assert_eq!(
        client.last(2),
        "Awesome, we successfully set your chat colors to 000 (bg) and 111 (fg)."
    );

    client.send(3, ".l 2 2");
    client.send(3, "hey what is up my studs");
    client.read_all();
    assert_eq!(
        client.last_bg(1),
        "000",
    );
    assert_eq!(
        client.last_fg(1),
        "111",
    );
}

#[test]
fn multiplayer_color() {
    let mut client = common::setup_with_db("color");
    client.send(1, ".n 1");
    client.send(1, ".c 000 111");
    client.send(2, ".n 2");
    client.send(2, ".c 111 000");
    client.send(1, "yo");
    client.read_all();
    assert_eq!(
        client.last_bg(1),
        "000"
    );
        assert_eq!(
        client.last_fg(2),
        "111"
    );

    client.send(2, "yo");
    client.read_all();
    assert_eq!(
        client.last_bg(1),
        "111"
    );
    assert_eq!(
        client.last_fg(2),
        "000"
    );
}

#[test]
fn login_after_rename() {
    let mut client = common::setup_with_db("login_after_rename");
    client.send(1, ".l chieftain chieftain");
    client.send(2, ".n 2");
    client.send(1, "hola at yo boi");
    client.read_all();
    assert_eq!(client.last_from(2), "chieftain");

    client.send(1, ".n chieftain2");
    client.send(1, "holla at yo boi");
    client.read_all();
    assert_eq!(client.last_from(2), "chieftain2");

    client.send(3, ".l chieftain chieftain");
    client.send(3, "Holler at your male.");
    client.read_all();
    assert_eq!(client.last_from(2), "chieftain2");
}