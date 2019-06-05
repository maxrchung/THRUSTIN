mod common;

use common::FileSystemClient;

#[test]
fn name() {
    let id = "name";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    let msg = a.send_and_read(".name a");
    a.stop();
    assert_eq!(msg, String::from("Name set to: a<br/>ok a, now ur redy 2 THRUST, try \'.help\' for sum updated information"));
}

#[test]
fn name_abbr() {
    let id = "name_abbr";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    let msg = a.send_and_read(".n a");
    a.stop();
    assert_eq!(msg, String::from("Name set to: a<br/>ok a, now ur redy 2 THRUST, try \'.help\' for sum updated information"));
}

#[test]
fn invalid_command() {
    let id = "invalid_command";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    let msg = a.send_and_read("what's up ya lil swagger");
    a.stop();
    assert_eq!(msg, String::from("u gotta pick a name bro, try '.name URNAMeHERE'"));
}

#[test]
fn name_help() {
    let id = "name_help";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    let msg = a.send_and_read(".help");
    a.stop();
    assert_eq!(msg, String::from("Alright so the first phase we\'ve got here is this Choose Name phase. What you\'re gonna do here is set yourself up with a name that you\'ll go by. i think this is a great idea because now you have a name and people can call you by your name later when we implement chat. Names give people a sense of identity and belonging. Could you imagine having not a name? What if we reduced you just to some unique number ID, now I think that would be rude, do you not agree? I dont\' really remember but I think you can change your name later too so don\'t worry its just like real life, how we change who we are, the way we speak and walk our gait when we\'re around other people.<br/><table class=\"table table-sm table-responsive w-auto\"><tr><td>Command</td><td>aLiAs</td><td>Help Massage</td></tr><tr><td>.help</td><td>.h</td><td>this is it chief</td></tr><tr><td>.name Y0LoSWAG4206669</td><td>.n Y0LoSWAG4206669</td><td>great this will change your name to Y0LoSWAG4206669</td></tr></table>"));
}

#[test]
fn rename() {
    let id = "rename";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    a.name();
    let msg = a.send_and_read(".n b");
    a.stop();
    assert_eq!(msg, String::from("Name set to: b"));
}

#[test]
fn duplicate_name_error() {
    let id = "duplicate_name_error";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    let b = FileSystemClient::new(id, "a");
    a.name();
    let msg = b.name();
    a.stop();
    assert_eq!(msg, String::from("yo that name exists ya gotta pick something else aight?"));
}

#[test]
fn duplicate_rename_error() {
    let id = "duplicate_rename_error";
    common::run_test_server(id);
    let a = FileSystemClient::new(id, "a");
    let b = FileSystemClient::new(id, "b");
    a.name();
    b.name();
    let msg = b.send_and_read(".n a");
    a.stop();
    assert_eq!(msg, String::from("yo that name exists ya gotta pick something else aight?"));
}