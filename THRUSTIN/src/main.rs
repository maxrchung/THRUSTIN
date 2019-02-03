#![feature(proc_macro_hygiene, decl_macro)] // Macro stuff to make rocket work
#[macro_use] extern crate rocket; // Macro stuff to make rocket work
mod lobby;
mod networking; // Get networking module
mod player;
use std::io::Read;
use std::io::Write;

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

    let commands: std::vec::Vec<&str> = vec!{&*"make",
                                             &*"delete",
                                             &*"start",
                                             &*"join",
                                             &*"leave",
                                             &*"list"
                                             &*"thrust"

    };
    
    let mut ind:i32 = -1;

    for (i, c) in commands.iter().enumerate() {
        if com == c.to_string() {
            ind = i as i32;
            break;
        }
    }

    match ind {
        0 => lobby::make_lobby(split, token, lobbies, players, communication),
        1 => lobby::delete_lobby(split, token, lobbies, communication),
        2 => lobby::start_game(split, token, lob_ids, players, communication),
        3 => lobby::join_lobby(split, token, lobbies, players, communication),
        4 => lobby::leave_lobby(split, token, lobbies, communication),
        5 => lobby::list_lobby(token, lobbies, communication),
        _ => communication.send_message(&token, &format!("Invalid argument! Valid commands: {:#?}", commands))
    }

}


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
