use crate::player;
use crate::networking;

#[derive(Debug)]
pub enum LobbyState {
    Waiting,
    Playing
}

#[derive(Debug)]
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

    //state of the lobby
    pub state: LobbyState,

    //lobby id
    pub id: u32,

    //lobby
}


pub fn new(name: std::string::String, 
           pw: std::string::String, 
           max: u32, 
           id: u32) -> Lobby {
    Lobby {
        name: name,
        pw: pw,
        list: std::vec::Vec::with_capacity(max as usize),
        count: 0,
        max: max,
        state: LobbyState::Waiting,
        id: id
    }
}


pub fn make_lobby(input: std::vec::Vec<&str>, 
                  id: ws::util::Token,
                  lobbies: &mut std::collections::HashMap<std::string::String, Lobby>,
                  players: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                  communication: &mut networking::Networking) {

    let name = input[1].to_string();
    let max = 64;
    let lobby_id: u32 = lobbies.len() as u32;

    players.get_mut(&id).unwrap().lobby = lobby_id.clone() as i32;

    let mut new_lobby = new(name.clone(), "".to_string(), max, lobby_id);
    new_lobby.list.push((*players.get(&id).unwrap()).clone());
    new_lobby.count += 1;

    lobbies.insert(name.clone(), new_lobby);
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

    let mut pl: &mut player::Player = players.get_mut(&id).unwrap();
    pl.host = true;
    
    let mut lob: &mut Lobby = lobbies.get_mut(&(pl.lobby as u32) ).unwrap();

    //lobbies.get_mut(&(players.get(&id).unwrap().lobby as u32)).unwrap().state = LobbyState::Playing;
    lob.state = LobbyState::Playing;
    //lobbies.get_mut(pl.lobby).unwrap().state = LobbyState::Playing;



    for p in &mut lob.list {
        p.state = player::PlayerState::Playing;
    }
}


pub fn join_lobby(input: std::vec::Vec<&str>, 
                  id: ws::util::Token,
                  lobby: &mut std::collections::HashMap<std::string::String, Lobby>,
                  players: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                  communication: &mut networking::Networking) {
    let lobby_name = input[1].to_string();

    let lob: &mut Lobby = lobby.get_mut(&lobby_name).unwrap();
    lob.list.push((*players.get(&id).unwrap()).clone());
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
