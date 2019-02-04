use crate::player;
use crate::networking;
use crate::thrust;

#[derive(Clone, Debug)]
pub struct Lobby {
    //name of lobby
    pub name: std::string::String,

    //optional password for lobby
    pw: std::string::String,

    //list of players
    pub list: std::vec::Vec<player::Player>,

    //number of players in room
    pub count: u32,

    //max number of players
    pub max: u32,

    //lobby id
    pub id: i32,

    //lobby

    pub deck: thrust::Deck,
}


pub fn new(name: std::string::String, 
           pw: std::string::String, 
           max: u32, 
           id: i32) -> Lobby {
    let mut lobby = Lobby {
        name: name,
        pw: pw,
        list: std::vec::Vec::with_capacity(max as usize),
        count: 0,
        max: max,
        id: id,
        deck: thrust::Deck::default(),
    };
    lobby.deck.sort();
    println!("{:#?}", lobby.deck);
    lobby
}


pub fn make_lobby(input: std::vec::Vec<&str>, 
                  id: ws::util::Token,
                  lobbies: &mut std::collections::HashMap<std::string::String, Lobby>,
                  players: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                  communication: &mut networking::Networking) {

    let name = input[1].to_string();
    let max = 64;
    let lobby_id: i32 = lobbies.len() as i32;

    let player: &mut player::Player = players.get_mut(&id).unwrap();

    player.lobby = lobby_id.clone() as i32;
    player.host = true;
    player.state = player::PlayerState::InLobby;

    let mut new_lobby = new(name.clone(), "".to_string(), max, lobby_id);
    new_lobby.list.push((*player).clone());
    new_lobby.count += 1;

    lobbies.insert(name.clone(), new_lobby.clone());
    communication.send_message(&id, &format!("Created lobby: {}", name));
}

// Users should not delete lobbies manually so this should be private
fn delete_lobby(input: std::vec::Vec<&str>, 
                    id: ws::util::Token, 
                    lobbies: &mut std::collections::HashMap<std::string::String, Lobby>,
                    communication: &mut networking::Networking) {
    let name = input[1];
    lobbies.remove(name);
}


pub fn start_game(input: std::vec::Vec<&str>, 
                  id: ws::util::Token,
                  lobbies: &mut std::collections::HashMap<u32, Lobby>,
                  players: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                  communication: &mut networking::Networking) {
    println!("1");

    let mut p: &mut player::Player = players.get_mut(&id).unwrap();
    p.host = true;
    p.is_thrustee = true;
    println!("222");

    println!("lobby: {}", p.lobby);
    println!("{:#?}", lobbies);
    let lob: &mut Lobby = lobbies.get_mut(&(p.lobby as u32) ).unwrap();
    println!("13");

    let current_thrustee = lob.deck.thrustees.pop().unwrap();

    println!("4444");

    for p in &mut lob.list {
        p.state = player::PlayerState::Playing;
        p.deck.thrusters.push(lob.deck.thrusters.pop().unwrap());
        p.deck.thrusters.push(lob.deck.thrusters.pop().unwrap());
        p.deck.thrusters.push(lob.deck.thrusters.pop().unwrap());
        p.deck.thrusters.push(lob.deck.thrusters.pop().unwrap());
        p.deck.thrusters.push(lob.deck.thrusters.pop().unwrap());
        communication.send_message(&p.token, &format!("{:#?}", &current_thrustee));
    }
}


pub fn join_lobby(input: std::vec::Vec<&str>, 
                  id: ws::util::Token,
                  lobby: &mut std::collections::HashMap<std::string::String, Lobby>,
                  players: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                  communication: &mut networking::Networking) {
    let lobby_name = input[1].to_string();
    let mut lob =  lobby.get_mut(&lobby_name);

    if let None = lob {
        communication.send_message(&id, &format!("Lobby does not exist."));
    } else {
        let l = lob.unwrap();
        let mut p: &mut player::Player = players.get_mut(&id).unwrap();
        p.state = player::PlayerState::InLobby;
        p.lobby = l.id;
        p.host = true;
        l.list.push(p.clone());
        communication.send_message(&id, &format!("Joined: {:#?}", &lobby_name));
    }
}


pub fn leave_lobby(input: std::vec::Vec<&str>, 
                   id: ws::util::Token, 
                   lobbies: &mut std::collections::HashMap<std::string::String, Lobby>,
                   communication: &mut networking::Networking) {
}


pub fn list_lobby(id: ws::util::Token,
                  lobbies: &mut std::collections::HashMap<std::string::String, Lobby>,
                  communication: &mut networking::Networking) {
    communication.send_message(&id, &format!("{:#?}", lobbies));
}


pub fn set_name(input: std::vec::Vec<&str>,
                id: ws::util::Token,
                players: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                communication: &mut networking::Networking) {

    let p_name = input[1].to_string();

    let player: &mut player::Player = players.get_mut(&id).unwrap();

    player.name = p_name.clone();
}
