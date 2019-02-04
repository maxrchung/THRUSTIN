use crate::player;
use crate::networking;
use crate::thrust;
use rand::{thread_rng, Rng};

#[derive(Clone, PartialEq, Debug)]
pub enum lobby_state {
    playing,
    waiting,
}

#[derive(Clone, Debug)]
pub struct Lobby {
    //optional password for lobby
    pw: std::string::String,

    //list of players
    pub list: std::vec::Vec<ws::util::Token>,

    //max number of players
    pub max: u32,

    //lobby id
    pub id: i32,

    //lobby state
    pub state: lobby_state,

    pub deck: thrust::Deck,

    pub current_thrustee: String,

    pub current_thrusters: Vec<String>,

    pub thrusted_players: Vec<ws::util::Token>
}


pub fn new(pw: std::string::String,
           max: u32,
           id: i32) -> Lobby {

    let mut lobby = Lobby {
        pw: pw,
        list: std::vec::Vec::with_capacity(max as usize),
        max: max,
        id: id,
        state: lobby_state::waiting,
        deck: thrust::Deck::default(),
        current_thrustee: String::new(),
        current_thrusters: Vec::new(),
        thrusted_players: Vec::new()
    };

    lobby.deck.sort();
    thread_rng().shuffle(&mut lobby.deck.thrusters);
    thread_rng().shuffle(&mut lobby.deck.thrustees);
    lobby
}


pub fn make_lobby(input: std::vec::Vec<&str>,
                  id: ws::util::Token,
                  lobbies: &mut std::collections::HashMap<i32, Lobby>,
                  players: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                  communication: &mut networking::Networking) {

    let max = 64;
    let lobby_id: i32 = lobbies.len() as i32;

    let player: &mut player::Player = players.get_mut(&id).unwrap();

    player.lobby = lobby_id.clone() as i32;
    player.host = true;
    player.state = player::PlayerState::InLobby;

    let mut new_lobby = new("".to_string(), max, lobby_id);
    new_lobby.list.push(id.clone());

    lobbies.insert(lobby_id, new_lobby.clone());
    communication.send_message(&id, &format!("Created lobby: {}", lobby_id));
}

pub fn decide(split: std::vec::Vec<&str>,
                 token: ws::util::Token,
                 lobbies: &mut std::collections::HashMap<i32, Lobby>,
                 players: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                 communication: &mut networking::Networking) {

    let player: &mut player::Player = players.get_mut(&token).unwrap();

    if !player.is_thrustee {
        communication.send_message(&token, &"You are not allowed to decide because you are a THRUSTER");
    }
    else {
        match split[1].parse::<i32>() {
            Ok(index) => {
                let lob: &mut Lobby = lobbies.get_mut(&player.lobby ).unwrap();
                if index < lob.current_thrusters.len() as i32 {
                    let chosen_thrust = lob.current_thrusters.remove(index as usize).clone();
                    lob.current_thrusters.clear();

                    lob.thrusted_players.clear();

                    for (index, player_token) in lob.list.iter().enumerate() {
                        communication.send_message(&player_token, & format!("THRUSTER has chosen this THRUST as the chosen THRUST, bois: {}.", &chosen_thrust));
                    }

                    lob.current_thrustee = lob.deck.thrustees.pop().unwrap();
                    player.is_thrustee = false;
                    for (index, player_token) in lob.list.iter().enumerate() {
                        if token == *player_token {
                            let mut next_thrustee = players.get_mut(&lob.list[(index + 1) % lob.list.len()]).unwrap();
                            next_thrustee.is_thrustee = true;
                            break;
                        }
                    }

                    for (index, player_token) in lob.list.iter().enumerate() {
                        match players.get(&player_token).unwrap().is_thrustee {
                            true => {
                                communication.send_message(&player_token, &"You are the neXt THRUSTEE! GetT ready to decide!");

                            },
                            false => {
                                communication.send_message(&player_token, &"ur a fkin thruster..now.");
                            }
                        };
                        communication.send_message(&player_token, &format!("HERE Is the next THRUSTEE: {}", &lob.current_thrustee));

                        // why are we here on this earth?
                        match players.get(&player_token).unwrap().is_thrustee {
                            true => (),
                            false => {
                                display_thrusters(&player_token, communication, &players.get(&player_token).unwrap().deck.thrusters);
                            }
                        };
                    }
                }
                else {
                    communication.send_message(&token, &"That shit's out of bound bro");
                }
            },
            _ => {
                communication.send_message(&token, &"That is an invalid parameter, use an index instead");
            }
        };

    }
}

pub fn handle_thrust(split: std::vec::Vec<&str>,
                 token: ws::util::Token,
                 lobbies: &mut std::collections::HashMap<i32, Lobby>,
                 players: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                 communication: &mut networking::Networking) {

    let player: &mut player::Player = players.get_mut(&token).unwrap();

    if player.is_thrustee {
        communication.send_message(&token, &"You are not allowed to THRUST because you are a THRUSTEE");
    }
    else {
        match split[1].parse::<i32>() {
            Ok(index) => {
                if index < player.deck.thrusters.len() as i32 {
                    let lob: &mut Lobby = lobbies.get_mut(&player.lobby ).unwrap();
                    for (index, player_token) in lob.thrusted_players.iter().enumerate() {
                        if token == *player_token {
                            communication.send_message(&player_token, &format!("You have already THRUSTED, you cannot THRUST again."));
                            return;
                        }
                    }

                    let picked_thruster = player.deck.thrusters.remove(index as usize);
                    let resulting_thrust = thrust::Deck::thrust(index, &picked_thruster, &lob.current_thrustee);
                    lob.current_thrusters.push(resulting_thrust.clone());

                    for (index, player_token) in lob.list.iter().enumerate() {
                        communication.send_message(&player_token, &format!("{}. {}", lob.current_thrusters.len() - 1, &resulting_thrust));
                    }
                    let replenished_thruster = lob.deck.thrusters.pop().unwrap();
                    player.deck.thrusters.push(replenished_thruster.clone());

                    lob.thrusted_players.push(player.token.clone());
                }
                else {
                    communication.send_message(&token, &"That shit's out of bound bro");
                }
            },
            _ => {
                communication.send_message(&token, &"That is an invalid parameter, use an index instead");
            }
        };

    }
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

pub fn display_thrusters(token: & ws::util::Token, communication: &mut networking::Networking, thrusters: & Vec<String>) {
    communication.send_message(&token, &"Here are your THRUSTERS:");
    for (index, thruster) in thrusters.iter().enumerate() {
        communication.send_message(&token, &format!("{}. {}", index, &thruster));
    }
}


pub fn start_game(input: std::vec::Vec<&str>,
                  id: ws::util::Token,
                  lobbies: &mut std::collections::HashMap<i32, Lobby>,
                  players: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                  communication: &mut networking::Networking) {
    let mut p: &mut player::Player = players.get_mut(&id).unwrap();
    p.is_thrustee = true;
    let lob: &mut Lobby = lobbies.get_mut(&p.lobby ).unwrap();
    lob.current_thrustee = lob.deck.thrustees.pop().unwrap();
    lob.state = lobby_state::playing;

    for token in &mut lob.list {
        let mut p = players.get_mut(&token).unwrap();
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

        let mut instructions = if p.is_thrustee {
            "You are the THRUSTEE."
        }
        else {
            "You are a THRUSTER."
        };

        communication.send_message(&p.token, &format!("This is your THRUSTEE: {}", &lob.current_thrustee));
        if !p.is_thrustee {
            display_thrusters(&p.token, communication, &p.deck.thrusters);
        };
        communication.send_message(&p.token, &format!("{}", instructions));
    }
}


pub fn join_lobby(input: std::vec::Vec<&str>,
                  id: ws::util::Token,
                  lobby: &mut std::collections::HashMap<i32, Lobby>,
                  players: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                  communication: &mut networking::Networking) {

    if input.len() < 1 {
        communication.send_message(&id, &"Lobby name required!");
        return;
    }

    match input[1].to_string().parse::<i32>() {
        Ok(lobby_id) => {
            let mut lob =  lobby.get_mut(&lobby_id);

            if let None = lob {
                communication.send_message(&id, &format!("Lobby does not exist."));
            } else {
                let l = lob.unwrap();
                let mut p: &mut player::Player = players.get_mut(&id).unwrap();
                p.state = if l.state == lobby_state::playing {
                    let thruster1 = l.deck.thrusters.pop().unwrap();
                    p.deck.thrusters.push(thruster1.clone());

                    let thruster2 = l.deck.thrusters.pop().unwrap();
                    p.deck.thrusters.push(thruster2.clone());

                    let thruster3 = l.deck.thrusters.pop().unwrap();
                    p.deck.thrusters.push(thruster3.clone());

                    let thruster4 = l.deck.thrusters.pop().unwrap();
                    p.deck.thrusters.push(thruster4.clone());

                    let thruster5 = l.deck.thrusters.pop().unwrap();
                    p.deck.thrusters.push(thruster5.clone());

                    let mut instructions = "You are a THRUSTER.";

                    communication.send_message(&p.token, &format!("This is your THRUSTEE: {}", &l.current_thrustee));
                    display_thrusters(&p.token, communication, &p.deck.thrusters);
                    communication.send_message(&p.token, &format!("{}", instructions));
                    player::PlayerState::Playing
                } else {
                    player::PlayerState::InLobby
                };
                p.lobby = l.id;
                l.list.push(p.token);
                communication.send_message(&id, &format!("Joined: {:#?}", &lobby_id));
            }
        },
        _ => communication.send_message(&id, &"Lmao make a lobby first dumbass"),

    }

}

pub fn show_thrusters(id: ws::util::Token,
                      players: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                      communication: &mut networking::Networking) {
    let mut p = players.get_mut(&id).unwrap();
    display_thrusters(&p.token, communication, &p.deck.thrusters);
}



pub fn leave_lobby(id: ws::util::Token,
                   lobbies: &mut std::collections::HashMap<i32, Lobby>,
                   players: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                   communication: &mut networking::Networking) {

    let pl = players.get_mut(&id).unwrap();
    let lob_id = pl.lobby;
    let lob: &mut Lobby = lobbies.get_mut(&lob_id).unwrap();

    pl.state = player::PlayerState::OutOfLobby;

    lob.list.remove_item(&id);

    if(lob.list.len() == 0) {
        lobbies.remove(&lob_id);
    }
    communication.send_message(&id, &format!("Left lobby: {}.", lob_id));
}


pub fn list_lobby(id: ws::util::Token,
                  lobbies: &mut std::collections::HashMap<i32, Lobby>,
                  communication: &mut networking::Networking) {
    for lob in lobbies.values() {
        let state = match &lob.state {
            lobby_state::playing => "Playing",
            lobby_state::waiting => "Waiting",
            _ => "wth is this lobby doing?"
        };
        communication.send_message(&id, &format!("id: {} | {}/{} players | {}", lob.id, lob.list.len(), lob.max, state));
    }
}


pub fn set_name(input: std::vec::Vec<&str>,
                id: ws::util::Token,
                players: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                communication: &mut networking::Networking) {

    if input.len() < 1 {
        communication.send_message(&id, &format!("You need a name!"));
        return;
    }
    let p_name = input[1].to_string();

    let player: &mut player::Player = players.get_mut(&id).unwrap();

    player.name = p_name.clone();

    communication.send_message(&id, &format!("Name set to: {}", &player.name));
}



pub fn list_all_players(id: ws::util::Token,
                        players: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                        communication: &mut networking::Networking) {

    for pl in players.values() {
        if pl.lobby >= 0 {
            communication.send_message(&id, &format!("{} in {}", pl.name, pl.lobby));
        } else {
            communication.send_message(&id, &format!("{}", pl.name));
        }
    }
}


pub fn list_lobby_players(id: ws::util::Token,
                   lobbies: &mut std::collections::HashMap<i32, Lobby>,
                   players: &mut std::collections::HashMap<ws::util::Token, player::Player>,
                   communication: &mut networking::Networking) {

    let pl = players.get_mut(&id).unwrap();
    let lob_id = pl.lobby;
    let lob: &mut Lobby = lobbies.get_mut(&lob_id).unwrap();

    for pl_tok in &lob.list {
        let play = players.get(&pl_tok).unwrap();
        let name = &play.name;
        if play.host == true {
            communication.send_message(&id, &format!("{}: host", name));
        } else {
            communication.send_message(&id, &format!("{}", name));
        }
    }
}
