use ws::util::Token;
use crate::lobby;
use crate::player;
use crate::networking::Networking;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub fn ChooseNameCommands(input: std::vec::Vec<&str>,
                       token: Token,
                       players: &mut HashMap<Token, Rc<RefCell<player::Player>>>,
                       communication: &Networking) {

    let mut com = input[0].to_string();
    com = com[..com.len()].to_string();
    
    match &*com {
        ".name" => player::set_name(input, token, players, communication),

        ".help" => lobby::list_choose_name_commands(token, communication),

        _ => {
            communication.send_message(&token, "u gotta pick a name bro, try '.name URNAMeHERE'");
        }
    }
}


pub fn OutOfLobbyCommands(input: std::vec::Vec<&str>,
                      token: Token,
                      players: &mut HashMap<Token, Rc<RefCell<player::Player>>>,
                      lobby_id: &mut i32,
                      lobbies: &mut HashMap<i32, lobby::Lobby>,
                      communication: &Networking) {
let is_thruster = true;

    let mut com = input[0].to_string();
    com = com[..com.len()].to_string();

    match &*com {
        ".help" => lobby::list_out_commands(token, communication),

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
    

pub fn InLobbyCommands(input: std::vec::Vec<&str>,
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
        ".help" => lobby::list_in_commands(token, communication),

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


pub fn PlayingCommands(input: std::vec::Vec<&str>,
                   token: Token,
                   lobby: &mut lobby::Lobby,
                   communication: &Networking) {
    let mut com = input[0].to_string();
    com = com[..com.len()].to_string();

    match &*com {
        ".help" => lobby::list_playing_commands(token, communication),

        ".thrust" => lobby.handle_thrust(input, token, communication),

        ".points" => lobby.display_points(token, communication),

        _ => communication.send_message(&token, "Bruh that's an invalid command."),
    }
}


pub fn ChoosingCommands(input: std::vec::Vec<&str>,
                   token: Token,
                   lobby: &mut lobby::Lobby,
                   communication: &Networking) {
    let mut com = input[0].to_string();
    com = com[..com.len()].to_string();

    match &*com {
        ".help" => lobby::list_playing_commands(token, communication),

        ".thrust" => lobby.choose(input, token, communication),

        ".points" => lobby.display_points(token, communication),

        _ => communication.send_message(&token, "Bruh that's an invalid command."),
    }
}

pub fn DecidingCommands(input: std::vec::Vec<&str>,
                   token: Token,
                   lobby: &mut lobby::Lobby,
                   communication: &Networking) {
    let mut com = input[0].to_string();
    com = com[..com.len()].to_string();

    match &*com {
        ".help" => lobby::list_playing_commands(token, communication),

        ".thrust" => lobby.decide(input, token, communication),

        ".points" => lobby.display_points(token, communication),

        _ => communication.send_message(&token, "Bruh that's an invalid command."),
    }
}

pub fn WaitingCommands(input: std::vec::Vec<&str>,
                   token: Token,
                   lobby: &mut lobby::Lobby,
                   communication: &Networking) {
    let mut com = input[0].to_string();
    com = com[..com.len()].to_string();

    match &*com {
        ".help" => lobby::list_playing_commands(token, communication),

        ".thrust" => communication.send_message(&token, "Chill out homeboy... you needa w8 for THRUSTEE to CHOOSE..."),

        ".points" => lobby.display_points(token, communication),

        _ => communication.send_message(&token, "Bruh that's an invalid command."),
    }
}
