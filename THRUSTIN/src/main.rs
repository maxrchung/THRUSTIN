#![feature(proc_macro_hygiene, decl_macro)] // Macro stuff to make rocket work
#![feature(vec_remove_item)] // for remove item in vector
#[macro_use] extern crate rocket; // Macro stuff to make rocket work
#[macro_use] extern crate lazy_static; //alexgarbage
extern crate regex; //alexgarbage
mod lobby;
mod networking; // Get networking module
mod player;
mod thrust;

fn main() {
    let mut communication = networking::Networking::init();
    let mut lobbies: std::collections::HashMap<i32, lobby::Lobby> = std::collections::HashMap::new();
    let mut players: std::collections::HashMap<ws::util::Token, player::Player> = std::collections::HashMap::new();

    loop {
        let (token, message) = communication.read_message();

        // Add to players list if not already
        if let None = players.get(&token)  {
            players.insert(token.clone(), player::new(&token));
        }

        handle_input(token, message, &mut lobbies, &mut players, &mut communication);
    }
}


fn handle_input(token: ws::util::Token,
                input: std::string::String,
                lobbies: &mut std::collections::HashMap<i32, lobby::Lobby>,
                players: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                communication: &mut networking::Networking) {

    // input.0.pop();
    let split: std::vec::Vec<&str> = input.split(' ').collect();
    let mut com = split[0].to_string();
    com = com[..com.len()].to_string();

    let player = players.get(&token).unwrap();
    match &player.state {
        player::PlayerState::OutOfLobby => {
            match &*com {
                "make" => {
                    lobby::make_lobby(split, token, lobbies, players, communication)
                },

                "join" => {
                    lobby::join_lobby(split, token, lobbies, players, communication)
                },

                "list" => {
                    lobby::list_lobby(token, lobbies, communication)
                },

                "name" => {
                    lobby::set_name(split, token, players, communication)
                },

                "who" => {
                    lobby::list_all_players(token, players, communication);
                },
                
                "help" => {
                    lobby::list_commands(token, communication);
                },

                _ => {
                    lobby::list_commands(token, communication);
                }
            }
        },
        player::PlayerState::InLobby => {
            match &*com {
                "start" => {
                    lobby::start_game(split, token, lobbies, players, communication)
                },

                "leave" => {
                    lobby::leave_lobby(token, lobbies, players, communication)
                },

                "who" => {
                    lobby::list_lobby_players(token, lobbies, players, communication);
                },
                
                "help" => {
                    lobby::list_commands(token, communication);
                },

                _ => {
                    lobby::list_commands(token, communication);
                }
            }
        },

        player::PlayerState::Playing => {
            match &*com {
                "thrust" => {
                    lobby::handle_thrust(split, token, lobbies, players, communication);
                },

                "decide" => {
                    lobby::decide(split, token, lobbies, players, communication);
                },

                "thrusters" => {
                    lobby::show_thrusters(token, players, communication);
                },

                "thrustee" => {

                },
                
                "help" => {
                    lobby::list_commands(token, communication);
                },

                _ => {
                    lobby::list_commands(token, communication);
                }
            }
        }
    }
}
