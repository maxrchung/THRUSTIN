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
    let mut table_html = String::from("<table class=\"table w-auto\">");
    table_html.push_str("<tr>");
    table_html.push_str("<th>Command</th>");
    table_html.push_str("<th>aLiAs</th>");
    table_html.push_str("<th>Help Massage</th>");
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
    let is_thruster = true;
    let com = get_command(&input);
    println!("{}", com);
    match &*com {
        ".help" | ".h" => list_out_commands(&pl.borrow()),

        ".join" | ".j" => lobby::Lobby::join_lobby(input, pl, lobbies),

        ".list" | ".l" => lobby::list_lobby(pl, lobbies),

        ".make" | ".m" => lobby::Lobby::make_lobby(input, pl, lobby_id, lobbies),

        ".name" | ".n" => player::set_name(input, pl, players),

        ".thrustee" | ".tee" => {
            let valid = lobby::add_item(&input, pl.clone(), lobbies, !is_thruster);
            if !valid {
                pl.borrow().send("Not valid thrustee. Please add blank space to allow THRUSTERS to THRUST into them.");
            }
        }

        ".thruster" | ".ter" => {
            lobby::add_item(&input, pl, lobbies, is_thruster);
        }

        ".who" | ".w" => lobby::list_all_players(pl, players),

        _ => {
            pl.borrow()
                .send("Bruh that's an invalid command...!.    try .help");
        }
    }
}

fn list_out_commands(pl: &player::Player) {
    pl.send_multiple(vec![
        "Valid commands:".to_string(),
        "'.help' this is it chief".to_string(),
        "'.join [#]' join lobby [#]".to_string(),
        "'.list' list lobbies".to_string(),
        "'.make' make a lobby".to_string(),
        "'.name [name]' change your name to [name]".to_string(),
        "'.THRUSTEE' \"Some THRUSTEE\" to add THRUSTEE".to_string(),
        "'.THRUSTER' \"Some THRUSTER\" to add THRUSTER".to_string(),
        "'.who' list everyone playing".to_string(),
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
    let is_thruster = true;
    let com = get_command(&input);
    let lobby = { lobbies.get_mut(&pl.borrow().lobby).unwrap() };
    match &*com {
        ".help" | ".he" => list_in_commands(&pl.borrow()),

        ".host" | ".hos" => lobby.switch_host(input, pl),

        ".house" | ".hou" => lobby.toggle_house(pl),

        ".info" | ".i" => lobby.info(pl),

        ".kick" | ".k" => lobby.kick(input, pl),

        ".leave" | ".l" => {
            if lobby.leave_lobby(pl) {
                let id = lobby.id;
                lobbies.remove(&id);
            }
        }

        ".name" | ".n" => player::set_name(input, pl, players),

        ".pass" | ".pa" => lobby.set_password(input, pl),

        ".players" | ".pl" => lobby.player_max(input, pl),

        ".points" | ".po" => lobby.point_max(input, pl),

        ".start" | ".s" => lobby.start_game(pl),

        ".thrustee" | ".tee" => {
            let valid = lobby::add_item(&input, pl.clone(), lobbies, !is_thruster);
            if !valid {
                pl.borrow().send("Not valid THRUSTEE. Please add blank space to allow THRUSTERS to THRUST into them.");
            }
        }

        ".thruster" | ".ter" => {
            lobby::add_item(&input, pl, lobbies, is_thruster);
        }

        ".who" | ".w" => lobby.list_lobby_players(pl),

        _ => pl
            .borrow()
            .send("Bruh that's an invalid command. enter .help"),
    }
}

fn list_in_commands(pl: &player::Player) {
    pl.send_multiple(vec![
        "Valid commands:".to_string(),
        "'.help' this is it chief".to_string(),
        "'.leave' leave lobby".to_string(),
        "'.name [name]' change your name to [name]".to_string(),
        "'.start' start game".to_string(),
        "'.THRUSTEE' \"Some THRUSTEE\" to add THRUSTEE".to_string(),
        "'.THRUSTER' \"Some THRUSTER\" to add THRUSTER".to_string(),
        "'.who' list everyone in lobby".to_string(),
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
        "Valid commands:".to_string(),
        "'.help' this is it chief".to_string(),
        "'.THRUST [#]' THRUST your [#] card".to_string(),
        "'.THRUSTEE' show the current THRUSTEE".to_string(),
        "'.THRUSTERS' show your THRUSTERS".to_string(),
        "'.points' to see current points".to_string(),
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
            "Valid commands:".to_string(),
            "'.THRUST [#]' THRUST [#] card as THE NEXT THRUSTEE".to_string(),
            "'.help' this is it chief".to_string(),
            "'.THRUSTEE' show the current THRUSTEE".to_string(),
            "'.THRUSTERS' show your THRUSTERS".to_string(),
            "'.points' to see current points".to_string(),
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
            "Valid commands:".to_string(),
            "'.decide [#]' pick [#] card as THE THRUSTEE".to_string(),
            "'.help' this is it chief".to_string(),
            "'.THRUSTEE' show the current THRUSTEE".to_string(),
            "'.THRUSTERS' show your THRUSTERS".to_string(),
            "'.points' to see current points".to_string(),
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
            "Valid commands:".to_string(),
            "'.help' this is it chief".to_string(),
            "'.points' to see current points".to_string(),
    ]);
}
