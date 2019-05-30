mod common;

use common::FileSystemClient;
use std::path::{Path};

#[test]
fn valid_name() {
    let id = "valid_name";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    let msg = a.send_and_read(".name a");
    assert_eq!(msg, String::from("Name set to: a<br/>ok a, now ur redy 2 THRUST, try \'.help\' for sum updated information"));

    a.stop();
}

#[test]
fn valid_name_abbreviation() {
    let id = "valid_name_abbreviation";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    let msg = a.send_and_read(".n a");
    assert_eq!(msg, String::from("Name set to: a<br/>ok a, now ur redy 2 THRUST, try \'.help\' for sum updated information"));

    a.stop();
}

#[test]
fn invalid_command() {
    let id = "invalid_command";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    let msg = a.send_and_read("what's up ya lil swagger");
    assert_eq!(msg, String::from("u gotta pick a name bro, try '.name URNAMeHERE'"));

    a.stop();
}

#[test]
fn help() {
    let id = "help";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    let msg = a.send_and_read(".help");
    assert_eq!(msg, String::from("Alright so the first phase we\'ve got here is this Choose Name phase. What you\'re gonna do here is set yourself up with a name that you\'ll go by. i think this is a great idea because now you have a name and people can call you by your name later when we implement chat. Names give people a sense of identity and belonging. Could you imagine having not a name? What if we reduced you just to some unique number ID, now I think that would be rude, do you not agree? I dont\' really remember but I think you can change your name later too so don\'t worry its just like real life, how we change who we are, the way we speak and walk our gait when we\'re around other people.<br/><table class=\"table table-sm table-responsive w-auto\"><tr><td>Command</td><td>aLiAs</td><td>Help Massage</td></tr><tr><td>.help</td><td>.h</td><td>this is it chief</td></tr><tr><td>.name Y0LoSWAG4206669</td><td>.n Y0LoSWAG4206669</td><td>great this will change your name to Y0LoSWAG4206669</td></tr></table>"));

    a.stop();
}

#[test]
fn help_abbreviation() {
    let id = "help_abbreviation";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    let msg = a.send_and_read(".h");
    assert_eq!(msg, String::from("Alright so the first phase we\'ve got here is this Choose Name phase. What you\'re gonna do here is set yourself up with a name that you\'ll go by. i think this is a great idea because now you have a name and people can call you by your name later when we implement chat. Names give people a sense of identity and belonging. Could you imagine having not a name? What if we reduced you just to some unique number ID, now I think that would be rude, do you not agree? I dont\' really remember but I think you can change your name later too so don\'t worry its just like real life, how we change who we are, the way we speak and walk our gait when we\'re around other people.<br/><table class=\"table table-sm table-responsive w-auto\"><tr><td>Command</td><td>aLiAs</td><td>Help Massage</td></tr><tr><td>.help</td><td>.h</td><td>this is it chief</td></tr><tr><td>.name Y0LoSWAG4206669</td><td>.n Y0LoSWAG4206669</td><td>great this will change your name to Y0LoSWAG4206669</td></tr></table>"));

    a.stop();
}