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
use crate::networking::Networking;
use crate::player::{Player, PlayerState};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use ws::util::Token;

const MAX_INPUT: usize = 6669;
fn main() {
    let communication = Rc::new(RefCell::new(Networking::init()));
    let mut lobby_id = 0;
    let mut lobbies: HashMap<i32, Lobby> = HashMap::new();
    let mut players: HashMap<Token, Rc<RefCell<Player>>> = HashMap::new();

    let read = communication.clone();

    loop {
        let (token, message) = read.borrow_mut().read_message();

        // Add to players list if not already
        if let None = players.get(&token) {
            players.insert(
                token.clone(),
                Rc::new(RefCell::new(player::new(&token, communication.clone()))),
            );
        }

        handle_input(token, message, &mut lobby_id, &mut lobbies, &mut players);
    }
}

fn handle_input(
    token: Token,
    input: String,
    lobby_id: &mut i32,
    lobbies: &mut HashMap<i32, lobby::Lobby>,
    players: &mut HashMap<Token, Rc<RefCell<player::Player>>>,
) {
    if input.len() > MAX_INPUT {
        let player = players.get(&token).unwrap().borrow();
        player.send("ok bro you are typing way too much lmao");
        return;
    }

    let split: std::vec::Vec<&str> = input.split(' ').collect();
    let state = {
        let player = players.get(&token).unwrap().borrow();
        player.state.clone()
    };

    let pl = { players.get(&token).unwrap().clone() };
    match state {
        PlayerState::ChooseName => {
            commands::choose_name_commands(split, pl, players);
        }

        PlayerState::OutOfLobby => {
            commands::out_of_lobby_commands(split, pl, players, lobby_id, lobbies);
        }

        PlayerState::InLobby => {
            commands::in_lobby_commands(split, pl, players, lobbies);
        }

        PlayerState::Playing => {
            commands::playing_commands(split, pl, lobbies);
        }

        PlayerState::Choosing => {
            commands::choosing_commands(split, pl, lobbies);
        }

        PlayerState::Deciding => {
            commands::deciding_commands(split, pl, lobbies);
        }

        PlayerState::Waiting => {
            commands::waiting_commands(split, pl, lobbies);
        }
    }
}
