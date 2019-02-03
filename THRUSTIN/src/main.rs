mod player;
mod lobby;
use std::io::Read;
use std::io::Write;

fn read_socket() -> (std::string::String, u32) {
    let mut out = std::string::String::new();

    std::io::stdin().read_line(&mut out).unwrap();
    (out, 1)
}


fn handle_input(mut input: (std::string::String, u32),
                lobbies: &mut std::collections::HashMap<std::string::String, lobby::Lobby>,
                lob_ids: &mut std::collections::HashMap<u32, lobby::Lobby>,
                players: &mut std::collections::HashMap<u32, player::Player>) {

    input.0.pop();
    let split: std::vec::Vec<&str> = input.0.split(' ').collect();
    let mut com = split[0].to_string();

    com = com[..com.len()].to_string();

    let commands: std::vec::Vec<&str> = vec!{&*"make",
                                             &*"delete",
                                             &*"start",
                                             &*"join",
                                             &*"leave",
                                             &*"list"
    };
    
    let mut ind:i32 = -1;

    for (i, c) in commands.iter().enumerate() {
        if com == c.to_string() {
            ind = i as i32;
            break;
        }
    }

    match ind {
        0 => lobby::make_lobby(split, input.1, lobbies, players),
        1 => lobby::delete_lobby(split, input.1, lobbies),
        2 => lobby::start_game(split, input.1, lob_ids, players),
        3 => lobby::join_lobby(split, input.1, lobbies, players),
        4 => lobby::leave_lobby(split, input.1, lobbies),
        5 => lobby::list_lobby(lobbies),
        _ => println!("Invalid argument!")
    }

}


fn main() {
    let mut lobbies: std::collections::HashMap<std::string::String, lobby::Lobby> = std::collections::HashMap::new();
    let mut lob_ids: std::collections::HashMap<u32, lobby::Lobby> = std::collections::HashMap::new();
    let mut players: std::collections::HashMap<u32, player::Player> = std::collections::HashMap::new();
    
    let p1 = player::new("fker".to_string());
    players.insert(1, p1);

    loop {
        print!(">> ");
        std::io::stdout().flush();
        let command = read_socket();
        handle_input(command, &mut lobbies, &mut lob_ids, &mut players);
    }
}
