use crate::player;
use crate::networking;

pub enum lobby_state {
    waiting,
    playing
}


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
    pub state: lobby_state,

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
        state: lobby_state::waiting,
        id: id
    }
}


pub fn make_lobby(input: std::vec::Vec<&str>, 
                  id: ws::util::Token,
                  lobby: &mut std::collections::HashMap<std::string::String, Lobby>,
                  player: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                  communication: &mut networking::Networking) {

    let name = input[1].to_string();
    let max = 64;
    let lobby_id: u32 = lobby.len() as u32;

    player.get_mut(&id).unwrap().lobby = lobby_id.clone() as i32;

    let mut new_lobby = new(name.clone(), "".to_string(), max, lobby_id);
    new_lobby.list.push((*player.get(&id).unwrap()).clone());
    new_lobby.count += 1;

    lobby.insert(name.clone(), new_lobby);
    communication.send_message(&id, &format!("Created lobby: {}", name));
}


pub fn delete_lobby(input: std::vec::Vec<&str>, 
                    id: ws::util::Token, 
                    lobby: &mut std::collections::HashMap<std::string::String, Lobby>,
                    communication: &mut networking::Networking) {
    let name = input[1];
    lobby.remove(name);
}


pub fn start_game(input: std::vec::Vec<&str>, 
                  id: ws::util::Token,
                  lobby: &mut std::collections::HashMap<u32, Lobby>,
                  player: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                  communication: &mut networking::Networking) {
    lobby.get_mut(&(player.get(&id).unwrap().lobby as u32)).unwrap().state = lobby_state::playing;
    //do call back or something here?
}


pub fn join_lobby(input: std::vec::Vec<&str>, 
                  id: ws::util::Token,
                  lobby: &mut std::collections::HashMap<std::string::String, Lobby>,
                  player: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                  communication: &mut networking::Networking) {
    let lobby_name = input[1].to_string();

    let lob: &mut Lobby = lobby.get_mut(&lobby_name).unwrap();
    lob.list.push((*player.get(&id).unwrap()).clone());
}


pub fn leave_lobby(input: std::vec::Vec<&str>, 
                   id: ws::util::Token, 
                   lobby: &mut std::collections::HashMap<std::string::String, Lobby>,
                   communication: &mut networking::Networking) {
}


pub fn list_lobby(id: ws::util::Token,
                  lobby: &mut std::collections::HashMap<std::string::String, Lobby>,
                  communication: &mut networking::Networking) {
    for l in lobby.values() {
        println!("{}: {}", l.id, l.name);

        for p in &l.list {
            println!("    {}", p.name);
        }
    }
}
