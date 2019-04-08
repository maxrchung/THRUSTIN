use crate::lobby;
use crate::networking::Networking;
use crate::player;
use ws::util::Token;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

///////////
//helpers//
///////////
// Retrieves command from split input
// Lowers input so case is insensitive
fn get_command(input: &Vec<&str>) -> String {
    let com = input[0].to_string().to_lowercase();
    return com;
}

// 
fn generate_table(commands: Vec<(&str, &str, &str)>) -> String {
    let mut table_html = String::from("<table class=\"table table-sm table-responsive w-auto\">");
    table_html.push_str("<tr>");
    table_html.push_str("<td>Command</td>");
    table_html.push_str("<td>aLiAs</td>");
    table_html.push_str("<td>Help Massage</td>");
    table_html.push_str("</tr>");
    for (command, alias, help) in commands {
        table_html.push_str("<tr>");

        table_html.push_str("<td>");
        table_html.push_str(&command.to_string());
        table_html.push_str("</td>");

        table_html.push_str("<td>");
        table_html.push_str(&alias.to_string());
        table_html.push_str("</td>");

        table_html.push_str("<td>");
        table_html.push_str(&help.to_string());
        table_html.push_str("</td>");

        table_html.push_str("</tr>");
    }
    table_html.push_str("</table>");
    return table_html;
}

///////////////
//choose name//
///////////////
pub fn choose_name_commands(
    input: Vec<&str>,
    pl: Rc<RefCell<player::Player>>,
    players: &mut HashMap<Token, Rc<RefCell<player::Player>>>,
) {
    let com = get_command(&input);
    match &*com {
        ".name" | ".n" => player::set_name(input, pl, players),

        ".help" | ".h" => list_choose_name_commands(&pl.borrow()),

        _ => {
            pl.borrow()
                .send("u gotta pick a name bro, try '.name URNAMeHERE'");
        }
    }
}

fn list_choose_name_commands(pl: &player::Player) {
    pl.send_multiple(vec![
        String::from("Alright so the first phase we've got here is this Choose Name phase. What you're gonna do here is set yourself up with a name that you'll go by. i think this is a great idea because now you have a name and people can call you by your name later when we implement chat. Names give people a sense of identity and belonging. Could you imagine having not a name? What if we reduced you just to some unique number ID, now I think that would be rude, do you not agree? I dont' really remember but I think you can change your name later too so don't worry its just like real life, how we change who we are, the way we speak and walk our gait when we're around other people."),
        generate_table(vec![
            (".help", ".h", "this is it chief"),
            (".name Y0LoSWAG4206669", ".n Y0LoSWAG4206669", "great this will change your name to Y0LoSWAG4206669")
        ])
    ]);
}

////////////////
//out of lobby//
////////////////
pub fn out_of_lobby_commands(
    input: Vec<&str>,
    pl: Rc<RefCell<player::Player>>,
    players: &mut HashMap<Token, Rc<RefCell<player::Player>>>,
    lobby_id: &mut i32,
    lobbies: &mut HashMap<i32, lobby::Lobby>,
) {
    let com = get_command(&input);
    match &*com {
        ".help" | ".h" => list_out_commands(&pl.borrow()),

        ".join" | ".j" => lobby::Lobby::join_lobby(input, pl, lobbies),

        ".list" | ".l" => lobby::list_lobby(pl, lobbies),

        ".make" | ".m" => lobby::Lobby::make_lobby(input, pl, lobby_id, lobbies),

        ".name" | ".n" => player::set_name(input, pl, players),

        ".thrust" | ".t" => lobby::handle_thrusteer_commands(&input, pl, lobbies),

        ".unthrust" => lobby::clear_pers_deck(pl, lobbies),

        ".who" | ".w" => lobby::list_all_players(pl, players),

        _ => {
            pl.borrow()
                .send("Bruh that's an invalid command...!.    try .help");
        }
    }
}

fn list_out_commands(pl: &player::Player) {
    pl.send_multiple(vec![
        String::from("Alright so now you're in like a waiting zone outside of all the lobbies. Here you can browse lobbies, organize your THRUSTS, and (eventually by milestone 5.3) chat with other people in like general chat. Have fun playing THRUSTIN, brought to you by WAXCHUG&daGWADS."),
        generate_table(vec![
            (".help", ".h", "this is it chief"),
            (".join 1", ".j 1", "Join the lobby with ID 1."),
            (".list", ".l", "Lists info for lobbies that are available"),
            (".make", ".m", "Make a new lobby"),
            (".name xx69SWAGGER911xx", ".n", "If you must, do this to change your name to xx69SWAGGER911xx"),
            (".THRUSTEE' \"Some _____ THRUSTEE\" \"Some _____ other _____ THRUSTEE\"", ".tee \"Some _____ THRUSTEE\" \"Some _____ other _____ THRUSTEE\"", "This will add new THRUSTEES to your THRUSTEE list. Remember to encapsulate each THRUSTEE with a quotation."),
            (".THRUSTER' \"Some THRUSTER\" \"Some other THRUSTER\"", ".ter \"Some THRUSTER\" \"Some other THRUSTER\"", "This is for adding a THRUSTER to your THRUSTS."),
            (".who", ".w", "See who else is swaggin' up in this whack with you"),
        ])
    ]);
}

////////////
//in lobby//
////////////
pub fn in_lobby_commands(
    input: Vec<&str>,
    pl: Rc<RefCell<player::Player>>,
    players: &mut HashMap<Token, Rc<RefCell<player::Player>>>,
    lobbies: &mut HashMap<i32, lobby::Lobby>,
) {
    let com = get_command(&input);
    let lobby = { lobbies.get_mut(&pl.borrow().lobby).unwrap() };
    match &*com {
        ".help" | ".h" => list_in_commands(&pl.borrow()),

        ".info" | ".i" => lobby.info(pl),

        ".leave" | ".l" => {
            if lobby.leave_lobby(pl) {
                let id = lobby.id;
                lobbies.remove(&id);
            }
        }

        ".name" | ".n" => player::set_name(input, pl, players),

        ".thrust" | ".t" => lobby::handle_thrusteer_commands(&input, pl.clone(), lobbies),

        ".unthrust" => lobby::clear_pers_deck(pl, lobbies),

        ".who" | ".w" => lobby.list_lobby_players(pl),

        ".chief" | ".c" => lobby.switch_host(input, pl),

        ".house" | ".ho" => lobby.toggle_house(pl),

        ".kick" | ".k" => lobby.kick(input, pl),

        ".pass" | ".pa" => lobby.set_password(input, pl),

        ".players" | ".pl" => lobby.player_max(input, pl),

        ".points" | ".po" => lobby.point_max(input, pl),

        ".start" | ".s" => lobby.start_game(pl),

        _ => pl
            .borrow()
            .send("Bruh that's an invalid command. enter .help"),
    }
}

fn list_in_commands(pl: &player::Player) {
    pl.send_multiple(vec![
        String::from("Hey cool so now you're in the lobby and now you've got some more commands. If you're the chief, you've got access to some special options to configure the lobby's game experience. Otherwise, normal non-chiefs, yall can chill out and wait for the game to start."),
        generate_table(vec![
            (".help", ".h", "this is it chief"),
            (".info", ".i", "I'm pretty sure this will give you some info about the lobby you're in."),
            (".leave", ".l", "We're sorry to see you go..."),
            (".name xxXAzn1994", ".n", "Should we really let you change your name at this point? Seems a little bit excessive but oh well yeah you can change your name to xxXAzn1994."),
            (".THRUSTEE' \"Some _____ THRUSTEE\" \"Some _____ other _____ THRUSTEE\"", ".tee \"Some _____ THRUSTEE\" \"Some _____ other _____ THRUSTEE\"", "Copy pasted. This will add new THRUSTEES to your THRUSTEE list. Remember to encapsulate each THRUSTEE with a quotation."),
            (".THRUSTER' \"Some THRUSTER\" \"Some other THRUSTER\"", ".ter \"Some THRUSTER\" \"Some other THRUSTER\"", "Copy pasted. This is for adding a THRUSTER to your THRUSTS."),
            (".who", ".w", "See who's whacking up this swag lobby with you"),
            (".chief xxXAzn1994", ".c", "(chief-only) Make xxXAzn1994 the chief of the lobby"),
            (".house", ".ho", "(chief-only) This toggles whether to additionally use our default provided cards - I mean THRUSTS --- Anyways don't worry, your own THRUSTS are always added."),
            (".kick YOLOSWAGGER69", ".k YOLOSWAGGER69", "(chief-only) Someone causing you trouble? Toxicity got you down? Well if you are a chief you can kick YOLOSWAGGER69 out of your lobby using this command."),
            (".pass passwordspelledbackwards123420", ".pa passwordspelledbackwards123420", "(chief-only) Sometimes you want to protect your lobby's privacy by setting your lobby's password to passwordspelledbackwards123420"),
            (".players 420", ".pl 420", "(chief-only) Okay, how many players do you want to allow in your lobby? 420?"),
            (".points 1", ".po 1", "(chief-only) Okay, how many points do you want to go to? 1? Don't do 1... cause then the game will end really fast."),
            (".start", ".s", "(chief-only) Yup, naturally as the chief you can start up the game."),
        ])
    ]);
}

////////////////////
//playing commands//
////////////////////
pub fn playing_commands(
    input: Vec<&str>,
    pl: Rc<RefCell<player::Player>>,
    lobbies: &mut HashMap<i32, lobby::Lobby>,
) {
    let com = get_command(&input);
    let lobby = { lobbies.get_mut(&pl.borrow().lobby).unwrap() };
    match &*com {
        ".help" | ".h" => list_playing_commands(&pl.borrow()),

        ".points" | ".p" => lobby.display_points(pl),

        ".thrust" | ".t" => lobby.handle_thrust(input, pl),

        _ => pl.borrow().send("Bruh that's an invalid command."),
    }
}

fn list_playing_commands(pl: &player::Player) {
    pl.send_multiple(vec![
        String::from("Great. Now you're in the phase where you are a THRUSTER. In this state, you can THRUST one of your THRUSTER options into the THRUSTEE. Make sure it's a good one!"),
        generate_table(vec![
            (".help", ".h", "this is it chief"),
            (".points", ".p", "See who's got the points in the lobby."),
            (".THRUST 0", ".t 0", "Thrust your first THRUSTER in baby."),
        ])
    ]);
}

////////////
//choosing//
////////////
pub fn choosing_commands(
    input: Vec<&str>,
    pl: Rc<RefCell<player::Player>>,
    lobbies: &mut HashMap<i32, lobby::Lobby>,
) {
    let com = get_command(&input);
    let lobby = { lobbies.get_mut(&pl.borrow().lobby).unwrap() };
    match &*com {
        ".help" | ".h" => list_choosing_commands(&pl.borrow()),

        ".points" | ".p" => lobby.display_points(pl),

        ".thrust" | ".t" => lobby.choose(input, pl),

        _ => pl.borrow().send("Bruh that's an invalid command."),
    }
}

fn list_choosing_commands(pl: &player::Player) {
    pl.send_multiple(vec![
        String::from("Okay you're a THRUSTEE now. First thing you've gotta do is choose a great THRUSTEE that other THRUSTERS can THRUST into. Make sure it's a juicy one!"),
        generate_table(vec![
            (".help", ".h", "this is it chief"),
            (".points", ".p", "See who's got the points in the lobby."),
            (".THRUST 2", ".t 2", "Choose THRUSTEE at index 2 to use."),
        ])
    ]);
}

////////////
//deciding//
////////////
pub fn deciding_commands(
    input: Vec<&str>,
    pl: Rc<RefCell<player::Player>>,
    lobbies: &mut HashMap<i32, lobby::Lobby>,
) {
    let com = get_command(&input);
    let lobby = { lobbies.get_mut(&pl.borrow().lobby).unwrap() };
    match &*com {
        ".help" | ".h" => list_deciding_commands(&pl.borrow()),

        ".points" | ".p" => lobby.display_points(pl),

        ".thrust" | ".t" => lobby.decide(input, pl),

        _ => pl.borrow().send("Bruh that's an invalid command."),
    }
}

fn list_deciding_commands(pl: &player::Player) {
    pl.send_multiple(vec![
        String::from("Yeah guy it's time for you to decide on the best THRUSTER. Pick the one that you like the best. Trust your head and your gut. You can do it. I believe in you."),
        generate_table(vec![
            (".help", ".h", "this is it chief"),
            (".points", ".p", "See who's got the points in the lobby."),
            (".THRUST 1", ".t 1", "You've made your decision. THRUSTER at index 1 is the best one."),
        ])
    ]);
}

///////////
//waiting//
///////////
pub fn waiting_commands(
    input: Vec<&str>,
    pl: Rc<RefCell<player::Player>>,
    lobbies: &mut HashMap<i32, lobby::Lobby>,
) {
    let com = get_command(&input);
    let lobby = { lobbies.get(&pl.borrow().lobby).unwrap() };
    match &*com {
        ".help" | ".h" => list_waiting_commands(&pl.borrow()),

        ".points" | ".p" => lobby.display_points(pl),

        ".thrust" | ".t" => pl
            .borrow()
            .send("Chill out homeboy... you needa w8 for THRUSTEE to choose..."),

        _ => pl.borrow().send("Bruh that's an invalid command."),
    }
}

fn list_waiting_commands(pl: &player::Player) {
    pl.send_multiple(vec![
        String::from("Aite my dude you needa chill and wait for the THRUSTEE to choose a good THRUSTEE to be THRUSTED with."),
        generate_table(vec![
            (".help", ".h", "this is it chief"),
            (".points", ".p", "See who's got the points in the lobby."),
            (".THRUST", ".t", "This doesn't actually do anything. We're just here to let you know you can't THRUST."),
        ])
    ]);
}
