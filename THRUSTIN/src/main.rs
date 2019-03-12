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

use std::collections::HashMap;
use ws::util::Token;

fn main() {
    let mut communication = networking::Networking::init();
    let mut lobbies: HashMap<i32, lobby::Lobby> = HashMap::new();
    let mut lobby_id = 0;
    let mut players: HashMap<Token, player::Player> = HashMap::new();

    loop {
        let (token, message) = communication.read_message();

        // Add to players list if not already
        if let None = players.get(&token) {
            players.insert(token.clone(), player::new(&token));
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
    players: &mut HashMap<Token, player::Player>,
    communication: &mut networking::Networking,
) {
    let split: std::vec::Vec<&str> = input.split(' ').collect();
    let mut com = split[0].to_string();
    com = com[..com.len()].to_string();
    let is_thruster: bool = true;

    let player = players.get(&token).unwrap();
    match &player.state {
        player::PlayerState::OutOfLobby => match &*com {
            ".help" => {
                lobby::list_out_commands(token, communication);
            }

            ".join" => lobby::join_lobby(split, token, lobbies, players, communication),

            ".list" => lobby::list_lobby(token, lobbies, communication),

            ".make" => {
                lobby::Lobby::make_lobby(split, token, lobby_id, lobbies, players, communication);
            }

            ".name" => lobby::set_name(split, token, players, communication),

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

            ".who" => {
                lobby::list_all_players(token, players, communication);
            }

            _ => {
                communication.send_message(&token, "Bruh that's an invalid command.");
            }
        },
        player::PlayerState::InLobby => {
            let lobby = lobbies.get_mut(&player.lobby).unwrap();

            match &*com {
                ".help" => {
                    lobby::list_in_commands(token, communication);
                }

                ".leave" => {
                    if lobby.leave_lobby(token, players, communication) {
                        let id = lobby.id;
                        lobbies.remove(&id);
                    }
                }

                ".start" => {
                    lobby.start_game(split, token, players, communication);
                }

                ".who" => {
                    lobby.list_lobby_players(token, players, communication);
                }

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

                _ => {
                    communication.send_message(&token, "Bruh that's an invalid command.");
                }
            }
        }

        player::PlayerState::Playing => match &*com {
            ".decide" => {
                lobby::decide(split, token, lobbies, players, communication);
            }

            ".help" => {
                lobby::list_playing_commands(token, communication);
            }

            ".thrust" => {
                lobby::handle_thrust(split, token, lobbies, players, communication);
            }

            ".thrustee" => {
                lobby::show_thrustee(token, lobbies, players, communication);
            }

            ".thrusters" => {
                lobby::show_thrusters(token, players, communication);
            }

            ".thrusts" => {
                lobby::show_thrusts(token, lobbies, players, communication);
            }

            _ => {
                communication.send_message(&token, "Bruh that's an invalid command.");
            }
        },
    }
}
