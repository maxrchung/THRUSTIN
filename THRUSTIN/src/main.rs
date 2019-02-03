#![feature(proc_macro_hygiene, decl_macro)] // Macro stuff to make rocket work
#[macro_use] extern crate rocket; // Macro stuff to make rocket work
mod lobby;
mod networking; // Get networking module
mod player;

fn main() {
    let mut communication = networking::Networking::init();
    let mut lobbies: std::collections::HashMap<std::string::String, lobby::Lobby> = std::collections::HashMap::new();
    let mut lob_ids: std::collections::HashMap<u32, lobby::Lobby> = std::collections::HashMap::new();
    let mut players: std::collections::HashMap<ws::util::Token, player::Player> = std::collections::HashMap::new();
    
    loop {
        let (token, message) = communication.read_message();

        // Add to players list if not already
        if let None = players.get(&token)  {
            players.insert(token.clone(), player::new("some_shit".to_string()));
        }
        handle_input(token, message, &mut lobbies, &mut lob_ids, &mut players, &mut communication);
    }
}


fn handle_input(token: ws::util::Token,
                input: std::string::String,
                lobbies: &mut std::collections::HashMap<std::string::String, lobby::Lobby>,
                lob_ids: &mut std::collections::HashMap<u32, lobby::Lobby>,
                players: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                communication: &mut networking::Networking) {

    // input.0.pop();
    let split: std::vec::Vec<&str> = input.split(' ').collect();
    let mut com = split[0].to_string();
    com = com[..com.len()].to_string();

    let player = players.get(&token).unwrap();
    match &player.state {
        player::PlayerState::GettingName => {
            match &*com {
                "name" => {

                }
            }
        },
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
                _ => {
                    communication.send_message(&token, &format!("Invalid argument!"));
                }
            }
        },
        player::PlayerState::InLobby => {
            match &*com {
                "start" => {
                    lobby::start_game(split, token, lob_ids, players, communication)
                },
                "leave" => {
                    lobby::leave_lobby(split, token, lobbies, communication)
                },
                _ => {
                    communication.send_message(&token, &format!("Invalid argument!"));
                }
            }
        },
        player::PlayerState::Playing => {
            match &*com {
                "thrust" => {
                },
                "decide" => {
                },
                _ => {
                    communication.send_message(&token, &format!("Invalid argument!"));
                }
            }
        }
    }
}