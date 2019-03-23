#![feature(proc_macro_hygiene, decl_macro)] // Macro stuff to make rocket work
#![feature(vec_remove_item)] // for remove item in vector
#[macro_use]
extern crate rocket; // Macro stuff to make rocket work
#[macro_use]
extern crate lazy_static; //alexgarbage
extern crate regex; //alexgarbage
mod commands;
mod lobby;
mod networking; // Get networking module
mod player;
mod thrust;

use crate::lobby::Lobby;
use crate::player::{Player, PlayerState};
use crate::networking::Networking;
use std::collections::HashMap;
use ws::util::Token;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let mut communication = Networking::init();
    let mut lobby_id = 0;
    let mut lobbies: HashMap<i32, Lobby> = HashMap::new();
    let mut players: HashMap<Token, Rc<RefCell<Player>>> = HashMap::new();

    loop {
        let (token, message) = communication.read_message();

        // Add to players list if not already
        if let None = players.get(&token) {
            players.insert(token.clone(), Rc::new(RefCell::new(player::new(&token))));
        }

        handle_input(
            token,
            message,
            &mut lobby_id,
            &mut lobbies,
            &mut players,
            &mut communication,
        );
    }
}

fn handle_input(
    token: Token,
    input: String,
    lobby_id: &mut i32,
    lobbies: &mut HashMap<i32, lobby::Lobby>,
    players: &mut HashMap<Token, Rc<RefCell<player::Player>>>,
    communication: &mut Networking,
) {
    let split: std::vec::Vec<&str> = input.split(' ').collect();
/*
    let mut com = split[0].to_string();
    com = com[..com.len()].to_string();
*/

    let state = {
        let player = players.get_mut(&token).unwrap().borrow(); 
        player.state.clone()
    };

    match state {
        PlayerState::ChooseName => commands::ChooseNameCommands(split, token, players, communication),

        PlayerState::OutOfLobby => commands::OutOfLobbyCommands(split, token, players, lobby_id, lobbies, communication),
        PlayerState::InLobby => {
            commands::InLobbyCommands(split, token, players, lobbies, communication);
        }

        PlayerState::Playing => {
            let lobby = {
                let player = players.get_mut(&token).unwrap().borrow();
                lobbies.get_mut(&player.lobby).unwrap()
            };

            commands::PlayingCommands(split, token, lobby, communication);
        },

        PlayerState::Choosing => {
            let lobby = {
                let player = players.get_mut(&token).unwrap().borrow();
                lobbies.get_mut(&player.lobby).unwrap()
            };

            commands::ChoosingCommands(split, token, lobby, communication);
        },

        PlayerState::Deciding => {
            let lobby = {
                let player = players.get_mut(&token).unwrap().borrow();
                lobbies.get_mut(&player.lobby).unwrap()
            };


            commands::DecidingCommands(split, token, lobby, communication);
        },

        PlayerState::Waiting => {
            let lobby = {
                let player = players.get_mut(&token).unwrap().borrow();
                lobbies.get_mut(&player.lobby).unwrap()
            };

            commands::WaitingCommands(split, token, lobby, communication);
        }
    }
}
