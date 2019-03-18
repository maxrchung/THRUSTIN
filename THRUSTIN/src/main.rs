#![feature(proc_macro_hygiene, decl_macro)] // Macro stuff to make rocket work
#![feature(vec_remove_item)] // for remove item in vector
#[macro_use]
extern crate rocket; // Macro stuff to make rocket work
#[macro_use]
extern crate lazy_static; //alexgarbage
extern crate regex; //alexgarbage
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
    let mut com = split[0].to_string();
    com = com[..com.len()].to_string();
    let is_thruster: bool = true;

    let state = {
        let player = players.get_mut(&token).unwrap().borrow(); 
        player.state.clone()
    };
    match state {
        PlayerState::ChooseName => match &*com {
            ".name" => player::set_name(split, token, players, communication),

            ".help" => lobby::list_choose_name_commands(token, communication),

            _ => {
                communication.send_message(&token, "Bruh that's an invalid command....    enter .help");
            }
        },

        PlayerState::OutOfLobby => match &*com {
            ".help" => lobby::list_out_commands(token, communication),

            ".join" => Lobby::join_lobby(split, token, lobbies, players, communication),

            ".list" => lobby::list_lobby(token, lobbies, communication),

            ".make" => {
                Lobby::make_lobby(split, token, lobby_id, lobbies, players, communication);
            }

            ".name" => player::set_name(split, token, players, communication),

            ".thrustee" => {
                let valid =
                    lobby::add_item(&split, token, lobbies, players, communication, !is_thruster);
                if !valid {
                    communication.send_message(&token, &"Not valid thrustee. Please add blank space to allow thrusters to thrust into them.");
                }
            }

            ".thruster" => {
                lobby::add_item(&split, token, lobbies, players, communication, is_thruster);
            }

            ".who" => lobby::list_all_players(token, players, communication),

            _ => {
                communication.send_message(&token, "Bruh that's an invalid command...!.    try .help");
            }
        },

        PlayerState::InLobby => {
            let lobby = {
                let player = players.get_mut(&token).unwrap().borrow();
                lobbies.get_mut(&player.lobby).unwrap()
            };

            match &*com {
                ".help" => lobby::list_in_commands(token, communication),

                ".name" => player::set_name(split, token, players, communication),
                
                ".leave" => {
                    if lobby.leave_lobby(token, communication) {
                        let id = lobby.id;
                        lobbies.remove(&id);
                    }
                }

                ".info" => lobby.info(token, communication),

                ".who" => lobby.list_lobby_players(token, communication),

                ".host" => lobby.switch_host(split, token, communication),

                ".kick" => lobby.kick(split, token, communication),

                ".pass" => lobby.set_password(split, token, communication),

                ".points" => lobby.point_max(split, token, communication),

                ".players" => lobby.player_max(split, token, communication),

                ".start" => lobby.start_game(token, communication),
                
                ".thrustee" => {
                    let valid = lobby::add_item(
                        &split,
                        token,
                        lobbies,
                        players,
                        communication,
                        !is_thruster,
                    );
                    if !valid {
                        communication.send_message(&token, &"Not valid thrustee. Please add blank space to allow thrusters to thrust into them.");
                    }
                }

                ".thruster" => {
                    lobby::add_item(&split, token, lobbies, players, communication, is_thruster);
                }
                
                _ => communication.send_message(&token, "Bruh that's an invalid command. enter .help"),
            }
        }

        PlayerState::Playing => {
            let lobby = {
                let player = players.get_mut(&token).unwrap().borrow();
                lobbies.get_mut(&player.lobby).unwrap()
            };
            match &*com {
                ".help" => {
                    lobby::list_playing_commands(token, communication);
                }

                ".thrust" => {
                    lobby.handle_thrust(split, token, communication);
                }

                _ => {
                    communication.send_message(&token, "Bruh that's an invalid command.");
                }
            }
        },

        PlayerState::Choosing => {
            let lobby = {
                let player = players.get_mut(&token).unwrap().borrow();
                lobbies.get_mut(&player.lobby).unwrap()
            };
            match &*com {
                ".choose" => {
                    lobby.choose(split, token, communication);
                }

                ".help" => {
                    lobby::list_choosing_commands(token, communication);
                }

                _ => {
                    communication.send_message(&token, "Bruh... that's invalid... enter .help");
                }
            }
        },

        PlayerState::Deciding => {
            let lobby = {
                let player = players.get_mut(&token).unwrap().borrow();
                lobbies.get_mut(&player.lobby).unwrap()
            };
            match &*com {
                ".thrust" => {
                    lobby.decide(split, token, communication);
                }

                ".help" => {
                    lobby::list_deciding_commands(token, communication);
                }

                _ => {
                    communication.send_message(&token, "Broski... that's invalid... enter .help");
                }
            }
        }
    }
}
