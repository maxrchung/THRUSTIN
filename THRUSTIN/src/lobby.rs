use crate::player;
use crate::networking;
use crate::thrust;
use rand::{thread_rng, Rng};

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
    thread_rng().shuffle(&mut lobby.deck.thrusters);
    thread_rng().shuffle(&mut lobby.deck.thrustees);

    println!("{:#?}", lobby.deck);
    lobby
}


pub fn make_lobby(input: std::vec::Vec<&str>, 
                  id: ws::util::Token,
                  lobbies: &mut std::collections::HashMap<i32, Lobby>,
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

    lobbies.insert(lobby_id, new_lobby.clone());
    communication.send_message(&id, &format!("Created lobby: {}", name));
}

// Users should not delete lobbies manually so this should be private
fn delete_lobby(input: std::vec::Vec<&str>, 
                    id: ws::util::Token, 
                    lobbies: &mut std::collections::HashMap<i32, Lobby>,
                    communication: &mut networking::Networking) {
    match input[1].parse::<i32>() {
        Ok(name) => {
            lobbies.remove(&name);
        },
        _ => ()
    };
}


pub fn start_game(input: std::vec::Vec<&str>, 
                  id: ws::util::Token,
                  lobbies: &mut std::collections::HashMap<i32, Lobby>,
                  players: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                  communication: &mut networking::Networking) {
    println!("1");

    let mut p: &mut player::Player = players.get_mut(&id).unwrap();
    p.host = true;
    p.is_thrustee = true;
    println!("222");

    println!("lobby: {}", p.lobby);
    println!("{:#?}", lobbies);
    let lob: &mut Lobby = lobbies.get_mut(&p.lobby ).unwrap();
    println!("13");

    let current_thrustee = lob.deck.thrustees.pop().unwrap();

    println!("4444");

    for p in &mut lob.list {
        p.state = player::PlayerState::Playing;
        let thruster1 = lob.deck.thrusters.pop().unwrap();
        p.deck.thrusters.push(thruster1.clone());

        let thruster2 = lob.deck.thrusters.pop().unwrap();
        p.deck.thrusters.push(thruster2.clone());

        let thruster3 = lob.deck.thrusters.pop().unwrap();
        p.deck.thrusters.push(thruster3.clone());

        let thruster4 = lob.deck.thrusters.pop().unwrap();
        p.deck.thrusters.push(thruster4.clone());

        let thruster5 = lob.deck.thrusters.pop().unwrap();
        p.deck.thrusters.push(thruster5.clone());

        let mut instructions = if p.host {
            "You are being thrusted!"
        }
        else {
            "You are thrusting!"
        };

        communication.send_message(&p.token, &format!("This is your thrustee: {}", &current_thrustee));
        communication.send_message(&p.token, &format!("Here are your thrusters:"));
        communication.send_message(&p.token, &format!("{}", &thruster1));
        communication.send_message(&p.token, &format!("{}", &thruster2));
        communication.send_message(&p.token, &format!("{}", &thruster3));
        communication.send_message(&p.token, &format!("{}", &thruster4));
        communication.send_message(&p.token, &format!("{}", &thruster5));
        communication.send_message(&p.token, &format!("{}", instructions));
    }
}


pub fn join_lobby(input: std::vec::Vec<&str>, 
                  id: ws::util::Token,
                  lobby: &mut std::collections::HashMap<i32, Lobby>,
                  players: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                  communication: &mut networking::Networking) {
    match input[1].to_string().parse::<i32>() {
        Ok(lobby_id) => {
            let mut lob =  lobby.get_mut(&lobby_id);

            if let None = lob {
                communication.send_message(&id, &format!("Lobby does not exist."));
            } else {
                let l = lob.unwrap();
                let mut p: &mut player::Player = players.get_mut(&id).unwrap();
                p.state = player::PlayerState::InLobby;
                p.lobby = l.id;
                l.list.push(p.clone());
                communication.send_message(&id, &format!("Joined: {:#?}", &lobby_id));
            }
        },
        _ => ()
    }
}


pub fn leave_lobby(input: std::vec::Vec<&str>, 
                   id: ws::util::Token, 
                   lobbies: &mut std::collections::HashMap<i32, Lobby>,
                   communication: &mut networking::Networking) {
}


pub fn list_lobby(id: ws::util::Token,
                  lobbies: &mut std::collections::HashMap<i32, Lobby>,
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
