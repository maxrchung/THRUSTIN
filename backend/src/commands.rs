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
        "Valid commands:".to_string(),
        "'.help' this is it chief".to_string(),
        "'.name [name]' change your name to [name]".to_string(),
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
    match &*com {
        ".help" => list_out_commands(&pl.borrow()),

        ".join" => lobby::Lobby::join_lobby(input, pl, lobbies),

        ".list" => lobby::list_lobby(pl, lobbies),

        ".make" => lobby::Lobby::make_lobby(input, pl, lobby_id, lobbies),

        ".name" => player::set_name(input, pl, players),

        ".thrustee" => {
            let valid = lobby::add_item(&input, pl.clone(), lobbies, !is_thruster);
            if !valid {
                pl.borrow().send("Not valid thrustee. Please add blank space to allow THRUSTERS to THRUST into them.");
            }
        }

        ".thruster" => {
            lobby::add_item(&input, pl, lobbies, is_thruster);
        }

        ".who" => lobby::list_all_players(pl, players),

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
        ".help" => list_in_commands(&pl.borrow()),

        ".name" => player::set_name(input, pl, players),

        ".leave" => {
            if lobby.leave_lobby(pl) {
                let id = lobby.id;
                lobbies.remove(&id);
            }
        }

        ".info" => lobby.info(pl),

        ".who" => lobby.list_lobby_players(pl),

        ".host" => lobby.switch_host(input, pl),

        ".kick" => lobby.kick(input, pl),

        ".pass" => lobby.set_password(input, pl),

        ".points" => lobby.point_max(input, pl),

        ".players" => lobby.player_max(input, pl),

        ".start" => lobby.start_game(pl),

        ".house" => lobby.toggle_house(pl),

        ".thrustee" => {
            let valid = lobby::add_item(&input, pl.clone(), lobbies, !is_thruster);
            if !valid {
                pl.borrow().send("Not valid THRUSTEE. Please add blank space to allow THRUSTERS to THRUST into them.");
            }
        }

        ".thruster" => {
            lobby::add_item(&input, pl, lobbies, is_thruster);
        }

        _ => pl
            .borrow()
            .send("Bruh that's an invalid command. enter .help"),
    }
}

fn list_in_commands(pl: &player::Player) {
    //fn list_in_commands(token: Token, communication: &Networking) {
    //communication.send_messages(
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
        ".help" => list_playing_commands(&pl.borrow()),

        ".thrust" => lobby.handle_thrust(input, pl),

        ".points" => lobby.display_points(pl),

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
        ".help" => list_playing_commands(&pl.borrow()),

        ".thrust" => lobby.choose(input, pl),

        ".points" => lobby.display_points(pl),

        _ => pl.borrow().send("Bruh that's an invalid command."),
    }
}

fn list_choosing_commands(token: Token, communication: &Networking) {
    communication.send_messages(
        &token,
        vec![
            "Valid commands:".to_string(),
            "'.THRUST [#]' THRUST [#] card as THE NEXT THRUSTEE".to_string(),
            "'.help' this is it chief".to_string(),
            "'.THRUSTEE' show the current THRUSTEE".to_string(),
            "'.THRUSTERS' show your THRUSTERS".to_string(),
            "'.points' to see current points".to_string(),
        ],
    );
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
        ".help" => list_playing_commands(&pl.borrow()),

        ".thrust" => lobby.decide(input, pl),

        ".points" => lobby.display_points(pl),

        _ => pl.borrow().send("Bruh that's an invalid command."),
    }
}

fn list_deciding_commands(token: Token, communication: &Networking) {
    communication.send_messages(
        &token,
        vec![
            "Valid commands:".to_string(),
            "'.decide [#]' pick [#] card as THE THRUSTEE".to_string(),
            "'.help' this is it chief".to_string(),
            "'.THRUSTEE' show the current THRUSTEE".to_string(),
            "'.THRUSTERS' show your THRUSTERS".to_string(),
            "'.points' to see current points".to_string(),
        ],
    );
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
        ".help" => list_playing_commands(&pl.borrow()),

        ".thrust" => pl
            .borrow()
            .send("Chill out homeboy... you needa w8 for THRUSTEE to choose..."),

        ".points" => lobby.display_points(pl),

        _ => pl.borrow().send("Bruh that's an invalid command."),
    }
}

fn list_waiting_commands(token: Token, communication: &Networking) {
    communication.send_messages(
        &token,
        vec![
            "Valid commands:".to_string(),
            "'.help' this is it chief".to_string(),
            "'.points' to see current points".to_string(),
        ],
    );
}