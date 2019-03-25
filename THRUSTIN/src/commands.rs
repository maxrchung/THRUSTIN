use ws::util::Token;
use crate::lobby;
use crate::player;
use crate::networking::Networking;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

///////////////
//choose name//
///////////////
pub fn choose_name_commands(input: std::vec::Vec<&str>,
                       token: Token,
                       players: &mut HashMap<Token, Rc<RefCell<player::Player>>>,
                       communication: &Networking) {

    let mut com = input[0].to_string();
    com = com[..com.len()].to_string();
    
    match &*com {
        ".name" => player::set_name(input, token, players, communication),

        ".help" => list_choose_name_commands(token, communication),

        _ => {
            communication.send_message(&token, "u gotta pick a name bro, try '.name URNAMeHERE'");
        }
    }
}

fn list_choose_name_commands(token: Token, communication: &Networking) {
    communication.send_messages(
        &token,
        vec![
            "Valid commands:".to_string(),
            "'.help' this is it chief".to_string(),
            "'.name [name]' change your name to [name]".to_string(),
        ],
    );
}



////////////////
//out of lobby//
////////////////
pub fn out_of_lobby_commands(input: std::vec::Vec<&str>,
                      token: Token,
                      players: &mut HashMap<Token, Rc<RefCell<player::Player>>>,
                      lobby_id: &mut i32,
                      lobbies: &mut HashMap<i32, lobby::Lobby>,
                      communication: &Networking) {
let is_thruster = true;

    let mut com = input[0].to_string();
    com = com[..com.len()].to_string();

    match &*com {
        ".help" => list_out_commands(token, communication),

        ".join" => lobby::Lobby::join_lobby(input, token, lobbies, players, communication),

        ".list" => lobby::list_lobby(token, lobbies, communication),

        ".make" => lobby::Lobby::make_lobby(input, token, lobby_id, lobbies, players, communication),

        ".name" => player::set_name(input, token, players, communication),

        ".thrustee" => {
            let valid =
                lobby::add_item(&input, token, lobbies, players, communication, !is_thruster);
            if !valid {
                communication.send_message(&token, &"Not valid thrustee. Please add blank space to allow thrusters to thrust into them.");
            }
        }

        ".thruster" => {
            lobby::add_item(&input, token, lobbies, players, communication, is_thruster);
        },

        ".who" => lobby::list_all_players(token, players, communication),

        _ => {
            communication.send_message(&token, "Bruh that's an invalid command...!.    try .help");
        }
    }
}
    

fn list_out_commands(token: Token, communication: &Networking) {
    communication.send_messages(
        &token,
        vec![
            "Valid commands:".to_string(),
            "'.help' this is it chief".to_string(),
            "'.join [#]' join lobby [#]".to_string(),
            "'.list' list lobbies".to_string(),
            "'.make' make a lobby".to_string(),
            "'.name [name]' change your name to [name]".to_string(),
            "'.thrustee' \"Some thrustee\" to add thrustee".to_string(),
            "'.thruster' \"Some thruster\" to add thruster".to_string(),
            "'.who' list everyone playing".to_string(),
        ],
    );
}





////////////
//in lobby//
////////////
pub fn in_lobby_commands(input: std::vec::Vec<&str>,
                   token: Token,
                   players: &mut HashMap<Token, Rc<RefCell<player::Player>>>,
                   lobbies: &mut HashMap<i32, lobby::Lobby>,
                   communication: &Networking,) {
    let is_thruster = true;

    let mut com = input[0].to_string();
    com = com[..com.len()].to_string();

    let lobby = {
        let player = players.get_mut(&token).unwrap().borrow();
        lobbies.get_mut(&player.lobby).unwrap()
    };


    match &*com {
        ".help" => list_in_commands(token, communication),

        ".name" => player::set_name(input, token, players, communication),

        ".leave" => {
            if lobby.leave_lobby(token, communication) {
                let id = lobby.id;
                lobbies.remove(&id);
            }
        }

        ".info" => lobby.info(token, communication),

        ".who" => lobby.list_lobby_players(token, communication),

        ".host" => lobby.switch_host(input, token, communication),

        ".kick" => lobby.kick(input, token, communication),

        ".pass" => lobby.set_password(input, token, communication),

        ".points" => lobby.point_max(input, token, communication),

        ".players" => lobby.player_max(input, token, communication),

        ".start" => lobby.start_game(token, communication),

        ".thrustee" => {
            let valid = lobby::add_item(
                &input,
                token,
                lobbies,
                players,
                communication,
                !is_thruster,
            );
            if !valid {
                communication.send_message(&token, &"Not valid thrustee. Please add blank space to allow thrusters to thrust into them.");
            }
        },

        ".thruster" => {
            lobby::add_item(&input, token, lobbies, players, communication, is_thruster);
        }

        _ => communication.send_message(&token, "Bruh that's an invalid command. enter .help"),
    }
}

fn list_in_commands(token: Token, communication: &Networking) {
    communication.send_messages(
        &token,
        vec![
            "Valid commands:".to_string(),
            "'.help' this is it chief".to_string(),
            "'.leave' leave lobby".to_string(),
            "'.name [name]' change your name to [name]".to_string(),
            "'.start' start game".to_string(),
            "'.thrustee' \"Some thrustee\" to add thrustee".to_string(),
            "'.thruster' \"Some thruster\" to add thruster".to_string(),
            "'.who' list everyone in lobby".to_string(),
        ],
    );
}


////////////////////
//playing commands//
////////////////////
pub fn playing_commands(input: std::vec::Vec<&str>,
                   token: Token,
                   lobby: &mut lobby::Lobby,
                   communication: &Networking) {
    let mut com = input[0].to_string();
    com = com[..com.len()].to_string();

    match &*com {
        ".help" => list_playing_commands(token, communication),

        ".thrust" => lobby.handle_thrust(input, token, communication),

        ".points" => lobby.display_points(token, communication),

        _ => communication.send_message(&token, "Bruh that's an invalid command."),
    }
}

fn list_playing_commands(token: Token, communication: &Networking) {
    communication.send_messages(
        &token,
        vec![
            "Valid commands:".to_string(),
            "'.help' this is it chief".to_string(),
            "'.thrust [#]' THRUST your [#] card".to_string(),
            "'.thrustee' show the current THRUSTEE".to_string(),
            "'.thrusters' show your THRUSTERS".to_string(),
            "'.points' to see current points".to_string(),
        ],
    );
}


////////////
//choosing//
////////////
pub fn choosing_commands(input: std::vec::Vec<&str>,
                   token: Token,
                   lobby: &mut lobby::Lobby,
                   communication: &Networking) {
    let mut com = input[0].to_string();
    com = com[..com.len()].to_string();

    match &*com {
        ".help" => list_playing_commands(token, communication),

        ".thrust" => lobby.choose(input, token, communication),

        ".points" => lobby.display_points(token, communication),

        _ => communication.send_message(&token, "Bruh that's an invalid command."),
    }
}

fn list_choosing_commands(token: Token, communication: &Networking) {
    communication.send_messages(
        &token,
        vec![
            "Valid commands:".to_string(),
            "'.thrust [#]' thrust [#] card as THE NEXT THRUSTEE".to_string(),
            "'.help' this is it chief".to_string(),
            "'.thrustee' show the current THRUSTEE".to_string(),
            "'.thrusters' show your THRUSTERS".to_string(),
            "'.points' to see current points".to_string(),
        ],
    );
}



////////////
//deciding//
////////////
pub fn deciding_commands(input: std::vec::Vec<&str>,
                   token: Token,
                   lobby: &mut lobby::Lobby,
                   communication: &Networking) {
    let mut com = input[0].to_string();
    com = com[..com.len()].to_string();

    match &*com {
        ".help" => list_playing_commands(token, communication),

        ".thrust" => lobby.decide(input, token, communication),

        ".points" => lobby.display_points(token, communication),

        _ => communication.send_message(&token, "Bruh that's an invalid command."),
    }
}

fn list_deciding_commands(token: Token, communication: &Networking) {
    communication.send_messages(
        &token,
        vec![
            "Valid commands:".to_string(),
            "'.decide [#]' pick [#] card as THE THRUSTEE".to_string(),
            "'.help' this is it chief".to_string(),
            "'.thrustee' show the current THRUSTEE".to_string(),
            "'.thrusters' show your THRUSTERS".to_string(),
            "'.points' to see current points".to_string(),
        ],
    );
}

///////////
//waiting//
///////////
pub fn waiting_commands(input: std::vec::Vec<&str>,
                   token: Token,
                   lobby: &mut lobby::Lobby,
                   communication: &Networking) {
    let mut com = input[0].to_string();
    com = com[..com.len()].to_string();

    match &*com {
        ".help" => list_playing_commands(token, communication),

        ".thrust" => communication.send_message(&token, "Chill out homeboy... you needa w8 for THRUSTEE to CHOOSE..."),

        ".points" => lobby.display_points(token, communication),

        _ => communication.send_message(&token, "Bruh that's an invalid command."),
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


