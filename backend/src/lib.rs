#![feature(proc_macro_hygiene, decl_macro)] // Macro stuff to make rocket work
#![feature(vec_remove_item)] // for remove item in vector
#[macro_use]
extern crate rocket; // Macro stuff to make rocket work
#[macro_use]
extern crate lazy_static; //alexgarbage
extern crate chrono;
extern crate regex; //alexgarbage

mod commands;
pub mod communication;
mod database;
mod lobby;
mod lobby_game;
mod player;
mod player_game;
mod thrust;

use communication::{ChannelCommunication, Communication, WebSocketCommunication};
use database::MongoDB;
use lobby::Lobby;
use player::{Player, PlayerState};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn run_test_server(comm: ChannelCommunication) {
    let chan_comm = Rc::new(RefCell::new(comm));
    let db = Rc::new(RefCell::new(MongoDB::new("thrustin_test")));
    run(chan_comm, db);
}

pub fn run_test_db_server(comm: ChannelCommunication, db_name: &str) {
    let chan_comm = Rc::new(RefCell::new(comm));
    let db = Rc::new(RefCell::new(MongoDB::new(db_name)));
    // When testing db, drop the users store on load
    db.borrow().users.drop().expect(&format!("Unable to drop test db: {}", db_name));
    run(chan_comm, db);
}

pub fn run_ws_server() {
    let ws_comm = Rc::new(RefCell::new(WebSocketCommunication::new()));
    let db = Rc::new(RefCell::new(MongoDB::new("thrustin")));
    run(ws_comm, db);
}

const MAX_INPUT: usize = 6669;
fn run(communication: Rc<RefCell<dyn Communication>>, db: Rc<RefCell<MongoDB>>) {
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
            db.clone()
        ))),
    );

    // Create Endless Lobby
    Lobby::make_endless_lobby(
        &players.get(&endless_uuid).unwrap().clone(),
        &mut 0,
        &mut lobbies,
    );

    loop {
        let (token, message) = read.borrow_mut().read_message();
        // Add to players list if not already
        if let None = players.get(&token) {
            players.insert(
                token.clone(),
                Rc::new(RefCell::new(Player::new(token, communication.clone(), db.clone()))),
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

        handle_input(token, &message, &mut lobby_id, &mut lobbies, &mut players);
    }
}

fn handle_input(
    token: u32,
    input: &str,
    lobby_id: &mut i32,
    lobbies: &mut HashMap<i32, Lobby>,
    players: &mut HashMap<u32, Rc<RefCell<Player>>>,
) {
    let split: Vec<&str> = input.split(' ').collect();
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
            commands::out_of_lobby_commands(input, split, pl, players, lobby_id, lobbies);
        }

        PlayerState::InLobby => {
            commands::in_lobby_commands(input, split, pl, players, lobbies);
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
