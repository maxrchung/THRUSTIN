#![feature(proc_macro_hygiene, decl_macro)] // Macro stuff to make rocket work
#![feature(vec_remove_item)] // for remove item in vector
#[macro_use]
extern crate rocket; // Macro stuff to make rocket work
#[macro_use]
extern crate lazy_static; //alexgarbage
extern crate regex; //alexgarbage

mod commands;
pub mod communication;
mod lobby;
mod player;
mod thrust;

use communication::{ChannelCommunication, Communication, WebSocketCommunication};
use lobby::Lobby;
use player::{Player, PlayerState};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn run_channel(comm: ChannelCommunication) {
    let chan_comm = Rc::new(RefCell::new(comm));
    run(chan_comm);
}

pub fn run_ws_server() {
    let ws_comm = Rc::new(RefCell::new(WebSocketCommunication::new()));
    run(ws_comm);
}

const MAX_INPUT: usize = 6669;
fn run(communication: Rc<RefCell<dyn Communication>>) {
    communication.borrow().start();

    let mut lobby_id = 1;
    let mut lobbies: HashMap<i32, Lobby> = HashMap::new();
    let mut players: HashMap<u32, Rc<RefCell<Player>>> = HashMap::new();

    let read = communication.clone();

    let endless_uuid = 0;
    // Add endless lobby host dummy boi
    players.insert(
        endless_uuid,
        Rc::new(RefCell::new(Player::new_endless_host(
            communication.clone(),
        ))),
    );

    // Create Endless Lobby
    Lobby::make_endless_lobby(
        &players.get(&endless_uuid).unwrap().clone(),
        &mut 0,
        &mut lobbies,
    );

    while read.borrow().continue_running() {
        let (token, message) = read.borrow_mut().read_message();
        // Logging for troubleshooting and FBI-ing user commands
        println!("\n{}: {}", &token, &message);

        // Add to players list if not already
        if let None = players.get(&token) {
            players.insert(
                token.clone(),
                Rc::new(RefCell::new(Player::new(token, communication.clone()))),
            );
        }

        // Ignore empty messages
        if message.is_empty() {
            return;
        }

        // Handle messages over cap
        if message.len() > MAX_INPUT {
            let player = players
                .get(&token)
                .expect("player not found for message length check")
                .borrow();
            player.send_message("ok bro you are typing way too much lmao");
            return;
        }

        let split: Vec<&str> = message.split(' ').collect();

        handle_input(token, split, &mut lobby_id, &mut lobbies, &mut players);
    }

    communication.borrow().stop();
}

fn handle_input(
    token: u32,
    split: Vec<&str>,
    lobby_id: &mut i32,
    lobbies: &mut HashMap<i32, Lobby>,
    players: &mut HashMap<u32, Rc<RefCell<Player>>>,
) {
    let state = {
        let player = players
            .get(&token)
            .expect("player not found for state")
            .borrow();
        player.state.clone()
    };

    let pl = {
        players
            .get(&token)
            .expect("player not found for handle input")
            .clone()
    };

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
            commands::playing_commands(split, pl, players, lobbies);
        }

        PlayerState::Choosing => {
            commands::choosing_commands(split, pl, players, lobbies);
        }

        PlayerState::Deciding => {
            commands::deciding_commands(split, pl, players, lobbies);
        }

        PlayerState::Waiting => {
            commands::waiting_commands(split, pl, players, lobbies);
        }
    }
}
