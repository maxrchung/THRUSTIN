use crate::networking;
use crate::player;
use crate::thrust;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use ws::util::Token;

#[derive(Clone, PartialEq, Debug)]
pub enum LobbyState {
    Playing,
    Waiting,
}

#[derive(Clone, Debug)]
pub struct Lobby {
    //optional password for lobby
    pw: String,

    //list of players
    pub list: std::vec::Vec<Token>,

    //max number of players
    pub max: u32,

    //max hand size
    pub hand_size: u32,

    //lobby id
    pub id: i32,

    //lobby state
    pub state: LobbyState,

    pub host: usize,

    pub deck: thrust::Deck,

    pub current_thrustee: String,

    pub current_thrusts: HashMap<Token, String>,
    //maps thrust index to token (end me)
    pub index_to_token: HashMap<i32, Token>,

    pub thrusted_players: Vec<Token>,
}

impl Lobby {
    fn new(pw: String, max: u32, id: i32, pers_deck: &mut thrust::Deck) -> Lobby {
        let mut lobby = Lobby {
            pw: pw,
            list: std::vec::Vec::with_capacity(max as usize),
            max: max,
            id: id,
            state: LobbyState::Waiting,
            hand_size: 5,
            host: 0,
            deck: thrust::Deck::default(),
            current_thrustee: String::new(),
            current_thrusts: HashMap::new(),
            index_to_token: HashMap::new(),
            thrusted_players: Vec::new(),
        };

        lobby
            .deck
            .thrustees
            .append(&mut pers_deck.thrustees.clone()); //addin "personal decks" to lobby(default) deck. .
        lobby
            .deck
            .thrusters
            .append(&mut pers_deck.thrusters.clone());
        lobby.deck.sort();
        thread_rng().shuffle(&mut lobby.deck.thrusters);
        thread_rng().shuffle(&mut lobby.deck.thrustees);
        lobby
    }

    fn is_host(&self, player: &player::Player) -> bool {
        self.host == self.search_player(&player)
    }

    pub fn make_lobby(
        input: std::vec::Vec<&str>,
        id: Token,
        lobbies: &mut HashMap<i32, Lobby>,
        players: &mut HashMap<Token, player::Player>,
        communication: &mut networking::Networking,
    ) {
        let max = 64;
        let lobby_id: i32 = lobbies.len() as i32;

        let player: &mut player::Player = players.get_mut(&id).unwrap();

        player.lobby = lobby_id.clone() as i32;
        player.state = player::PlayerState::InLobby;

        let mut new_lobby = Lobby::new("".to_string(), max, lobby_id, &mut player.personal_deck);
        new_lobby.list.push(id.clone());

        lobbies.insert(lobby_id, new_lobby.clone());
        communication.send_message(&id, &format!("Created lobby: {}", lobby_id));
    }

    pub fn start_game(
        &mut self,
        input: std::vec::Vec<&str>,
        id: Token,
        players: &mut HashMap<Token, player::Player>,
        communication: &mut networking::Networking,
    ) {
        let mut p: &mut player::Player = players.get_mut(&id).unwrap();

        if !self.is_host(&p) {
            communication.send_message(&p.token, &format!("Only host can start game!"));
            return;
        }

        p.is_thrustee = true;

        self.current_thrustee = self.deck.thrustees.pop().unwrap();
        self.state = LobbyState::Playing;

        let mut next = "".to_string();
        for token in &mut self.list {
            let mut p = players.get_mut(&token).unwrap();
            p.state = player::PlayerState::Playing;

            for i in 0..self.hand_size {
                if let Some(card) = self.deck.thrusters.pop() {
                    p.deck.thrusters.push(card.clone());

                } else {
                    communication.send_message(&id, &format!("chief called and said we outta cards"));
                    return;
                }
            }

            let instructions = if p.is_thrustee {
                next = p.name.clone();
                "You are the THRUSTEE."
            } else {
                "You are a THRUSTER."
            };

            let mut messages = vec![format!("{}", instructions).to_string()];
            messages.push(
                format!(
                    "This is your THRUSTEE for {}: {}",
                    next, &self.current_thrustee
                )
                .to_string(),
            );
            if !p.is_thrustee {
                messages.extend(get_thrusters(&p.deck.thrusters));
            };
            communication.send_messages(&p.token, messages);
        }
    }

    fn search_player(&self, player: &player::Player) -> usize {
        for (i, pl) in self.list.iter().enumerate() {
            if pl == &player.token {
                return i;
            }
        }

        self.list.len()
    }

    pub fn leave_lobby(
        &mut self,
        id: Token,
        players: &mut HashMap<Token, player::Player>,
        communication: &mut networking::Networking,
    ) -> bool {
        let pl = players.get_mut(&id).unwrap();
        let lob_id = pl.lobby;

        pl.state = player::PlayerState::OutOfLobby;

        self.list.remove_item(&id);

        communication.send_message(&id, &format!("Left lobby: {}.", lob_id));
        self.send_message(&format!("{} has left the lobby.", pl.name), communication);

        if self.list.len() == 0 {
            true
        } else {
            false
        }
    }

    pub fn list_lobby_players(
        &self,
        id: Token,
        players: &mut HashMap<Token, player::Player>,
        communication: &mut networking::Networking,
    ) {
        let mut messages = Vec::new();
        for pl_tok in &self.list {
            let play = players.get(&pl_tok).unwrap();
            let name = &play.name;

            if self.is_host(&play) {
                messages.push(format!("{}: host", name).to_string());
            } else {
                messages.push(format!("{}", name).to_string());
            }
        }
        communication.send_messages(&id, messages);
    }

    pub fn send_message(&self, message: &str, communication: &mut networking::Networking) {
        for pl in &self.list {
            communication.send_message(pl, &message);
        }
    }
}

pub fn decide(
    split: std::vec::Vec<&str>,
    token: Token,
    lobbies: &mut HashMap<i32, Lobby>,
    players: &mut HashMap<Token, player::Player>,
    communication: &mut networking::Networking,
) {
    let player = players.get_mut(&token).unwrap();
    if !player.is_thrustee {
        communication.send_message(
            &token,
            &"You are not allowed to decide because you are a THRUSTER",
        );
    } else {
        match split[1].parse::<i32>() {
            Ok(index) => {
                let lob: &mut Lobby = lobbies.get_mut(&player.lobby).unwrap();
                if index < lob.current_thrusts.len() as i32 && index >= 0 {
                    let chosen_thrust = lob.current_thrusts.remove(&lob.index_to_token.get(&index).unwrap()).unwrap();
                    lob.current_thrusts.clear();
                    lob.thrusted_players.clear();

                    // This block also helps solve single player mut reference issues
                    let common = vec![format!(
                        "THRUSTEE {} has chosen this THRUSTER as the chosen THRUST, bois: {}<br/>",
                        &player.name, &chosen_thrust
                    )
                    .to_string()];

                    lob.current_thrustee = lob.deck.thrustees.pop().unwrap();
                    player.is_thrustee = false;

                    let mut next = "".to_string();
                    for (index, player_token) in lob.list.iter().enumerate() {
                        if token == *player_token {
                            let mut next_thrustee = players
                                .get_mut(&lob.list[(index + 1) % lob.list.len()])
                                .unwrap();
                            next_thrustee.is_thrustee = true;
                            next = next_thrustee.name.clone();
                            break;
                        }
                    }

                    for player_token in &lob.list {
                        let mut messages = common.clone();

                        match players.get(&player_token).unwrap().is_thrustee {
                            true => {
                                messages.push(
                                    "You are the neXt THRUSTEE! GetT ready to decide!".to_string(),
                                );
                                messages.push(
                                    format!("HERE Is your THRUSTEE: {}", &lob.current_thrustee)
                                        .to_string(),
                                );
                            }
                            false => {
                                messages.push("ur a fkin thruster..now.".to_string());
                                messages.push(
                                    format!(
                                        "HERE Is the next THRUSTEE for {}: {}",
                                        next, &lob.current_thrustee
                                    )
                                    .to_string(),
                                );
                                messages.extend(get_thrusters(
                                    &players.get(&player_token).unwrap().deck.thrusters,
                                ));
                            }
                        };
                        communication.send_messages(player_token, messages);
                    }
                } else {
                    communication.send_message(&token, &"That shit's out of bound bro");
                }
            }
            _ => {
                communication.send_message(
                    &token,
                    &"That is an invalid parameter, use an index instead",
                );
            }
        };
    }
}

pub fn handle_thrust(
    split: std::vec::Vec<&str>,
    token: Token,
    lobbies: &mut HashMap<i32, Lobby>,
    players: &mut HashMap<Token, player::Player>,
    communication: &mut networking::Networking,
) {
    let player: &mut player::Player = players.get_mut(&token).unwrap();

    if player.is_thrustee {
        communication.send_message(
            &token,
            &"You are not allowed to THRUST because you are a THRUSTEE",
        );
    } else {
        if split.len() < 2 {
            communication.send_message(&token, &"Index required!");
            return;
        }

        match split[1].parse::<i32>() {
            Ok(index) => {
                if index < player.deck.thrusters.len() as i32 && index >= 0 {
                    let lob: &mut Lobby = lobbies.get_mut(&player.lobby).unwrap();
                    for (index, player_token) in lob.thrusted_players.iter().enumerate() {
                        if token == *player_token {
                            communication.send_message(
                                &player_token,
                                &format!("You have already THRUSTED, you cannot THRUST again."),
                            );
                            return;
                        }
                    }

                    let picked_thruster = player.deck.thrusters.remove(index as usize);
                    let resulting_thrust =
                        thrust::Deck::thrust(index, &picked_thruster, &lob.current_thrustee);
                    lob.current_thrusts.insert(player.token, resulting_thrust.clone());
                    lob.index_to_token.insert((lob.current_thrusts.len() - 1) as i32, player.token);

                    for player_token in lob.list.iter() {
                        communication.send_message(
                            &player_token,
                            &format!("{}. {}", &(lob.current_thrusts.len() as i32 - 1), &resulting_thrust),
                        );
                    }
                    let replenished_thruster = lob.deck.thrusters.pop().unwrap();
                    player.deck.thrusters.push(replenished_thruster.clone());

                    lob.thrusted_players.push(player.token.clone());
                } else {
                    communication.send_message(&token, &"That shit's out of bound bro");
                }
            }
            _ => {
                communication.send_message(
                    &token,
                    &"That is an invalid parameter, use an index instead",
                );
            }
        };
    }
}

// Users should not delete lobbies manually so this should be private
fn delete_lobby(
    input: std::vec::Vec<&str>,
    id: Token,
    lobbies: &mut HashMap<i32, Lobby>,
    communication: &mut networking::Networking,
) {
    match input[1].parse::<i32>() {
        Ok(name) => {
            lobbies.remove(&name);
        }
        _ => (),
    };
}

pub fn get_thrusters(thrusters: &Vec<String>) -> Vec<String> {
    let mut messages = vec!["Here are your THRUSTERS:".to_string()];
    for (index, thruster) in thrusters.iter().enumerate() {
        messages.push(format!("{}. {}", &index, &thruster).to_string());
    }
    messages
}

pub fn join_lobby(
    input: std::vec::Vec<&str>,
    id: Token,
    lobby: &mut HashMap<i32, Lobby>,
    players: &mut HashMap<Token, player::Player>,
    communication: &mut networking::Networking,
) {
    if input.len() < 2 {
        communication.send_message(&id, &"Lobby name required!");
        return;
    }

    match input[1].to_string().parse::<i32>() {
        Ok(lobby_id) => {
            let lob = lobby.get_mut(&lobby_id);

            let mut messages = Vec::new();
            if let Some(l) = lob {
                let mut p: &mut player::Player = players.get_mut(&id).unwrap();

                p.state = if l.state == LobbyState::Playing {
                    for i in 0..l.hand_size {
                        //let card = l.deck.thrusters.pop();

                        if let Some(card) = l.deck.thrusters.pop() {
                            p.deck.thrusters.push(card.clone());
                        } else {
                            communication.send_message(&id, &format!("Out of cards!"));
                            return;
                        }
                    }
                    
                    l.send_message(&format!("{} has joined the lobby.", p.name), communication);

                    messages.push("You are a THRUSTER.".to_string());
                    messages.push(
                        format!("This is your THRUSTEE: {}", &l.current_thrustee).to_string(),
                    );
                    messages.extend(get_thrusters(&p.deck.thrusters));
                    player::PlayerState::Playing
                } else {
                    l.send_message(&format!("{} has joined the lobby.", p.name), communication);

                    player::PlayerState::InLobby
                };
                p.lobby = l.id;
                l.list.push(p.token);
                messages.push(format!("Joined: {:#?}", &lobby_id));
                communication.send_messages(&id, messages);
                // add players' personal deck (.thrustee/.thruster) to lobby deck
                l
                .deck
                .thrustees
                .append(&mut p.personal_deck.thrustees.clone());
                l
                .deck
                .thrusters
                .append(&mut p.personal_deck.thrusters.clone());
            } else {
                communication.send_message(&id, &format!("Lobby does not exist."));
            }
        }

        _ => communication.send_message(&id, &"Lmao make a lobby first dumbass"),
    }
}

pub fn show_thrusters(
    id: Token,
    players: &mut HashMap<Token, player::Player>,
    communication: &mut networking::Networking,
) {
    let mut p = players.get_mut(&id).unwrap();
    communication.send_messages(&p.token, get_thrusters(&p.deck.thrusters));
}

pub fn show_thrustee(
    id: Token,
    lobbies: &mut HashMap<i32, Lobby>,
    players: &mut HashMap<Token, player::Player>,
    communication: &mut networking::Networking,
) {
    let mut p: &mut player::Player = players.get_mut(&id).unwrap();
    let lob: &mut Lobby = lobbies.get_mut(&p.lobby).unwrap();
    communication.send_message(&id, &format!("Current THRUSTEE: {}", lob.current_thrustee));
}


pub fn list_lobby(
    id: Token,
    lobbies: &mut HashMap<i32, Lobby>,
    communication: &mut networking::Networking,
) {
    for lob in lobbies.values() {
        let state = match &lob.state {
            LobbyState::Playing => "Playing",
            LobbyState::Waiting => "Waiting",
            _ => "wth is this lobby doing?",
        };
        communication.send_message(
            &id,
            &format!(
                "id: {} | {}/{} players | {}",
                lob.id,
                lob.list.len(),
                lob.max,
                state
            ),
        );
    }
}

pub fn set_name(
    input: std::vec::Vec<&str>,
    id: Token,
    players: &mut HashMap<Token, player::Player>,
    communication: &mut networking::Networking,
) {
    if input.len() < 2 {
        communication.send_message(&id, &format!("You need a name!"));
        return;
    }

    let p_name = input[1].to_string();

    let player: &mut player::Player = players.get_mut(&id).unwrap();

    player.name = p_name.clone();

    communication.send_message(&id, &format!("Name set to: {}", &player.name));
}

pub fn list_all_players(
    id: Token,
    players: &mut HashMap<Token, player::Player>,
    communication: &mut networking::Networking,
) {
    let mut messages = Vec::new();
    for pl in players.values() {
        if pl.state == player::PlayerState::InLobby || pl.state == player::PlayerState::Playing {
            messages.push(format!("{} in {}", pl.name, pl.lobby).to_string());
        } else {
            messages.push(format!("{}", pl.name).to_string());
        }
    }
    communication.send_messages(&id, messages);
}

pub fn list_out_commands(id: Token, communication: &mut networking::Networking) {
    communication.send_messages(
        &id,
        vec![
            "Valid commands:".to_string(),
            "'.help' this is it chief".to_string(),
            "'.join [#]' join lobby [#]".to_string(),
            "'.list' list lobbies".to_string(),
            "'.make' make a lobby".to_string(),
            "'.name [name]' change your name to [name]".to_string(),
            "'.thrustee' \"Some thrustee\" to add thrustee".to_string(),
            "'.thruster' \"Some thruster\" to add thruster".to_string(),
            "'.who' list everyone playing".to_string(),
        ],
    );
}

pub fn list_in_commands(id: Token, communication: &mut networking::Networking) {
    communication.send_messages(
        &id,
        vec![
            "Valid commands:".to_string(),
            "'.help' this is it chief".to_string(),
            "'.leave' leave lobby".to_string(),
            "'.start' start game".to_string(),
            "'.thrustee' \"Some thrustee\" to add thrustee".to_string(),
            "'.thruster' \"Some thruster\" to add thruster".to_string(),
            "'.who' list everyone in lobby".to_string(),
        ],
    );
}

pub fn list_playing_commands(id: Token, communication: &mut networking::Networking) {
    communication.send_messages(
        &id,
        vec![
            "Valid commands:".to_string(),
            "'.decide [#]' pick [#] card as THE THRUSTEE".to_string(),
            "'.help' this is it chief".to_string(),
            "'.thrust [#]' THRUST your [#] card".to_string(),
            "'.thrustee' show the current THRUSTEE".to_string(),
            "'.thrusters' show your THRUSTERS".to_string(),
        ],
    );
}

pub fn add_item(
    input: &std::vec::Vec<&str>,
    id: Token,
    lobby: &mut HashMap<i32, Lobby>,
    players: &mut HashMap<Token, player::Player>,
    communication: &mut networking::Networking,
    thruster: bool,
) -> bool {

    if input.len() < 2 {
        communication.send_message(&id, &"Thruster/thrustee required!");
        return true;
    }

    let player: &mut player::Player = players.get_mut(&id).unwrap();

    let mut new_item = String::new();
    for i in 1..input.len() {
        new_item.push_str(input[i as usize]);
        new_item.push_str(" ");
    }
    new_item.pop();

    if new_item.chars().next().unwrap() != "\"".to_string().chars().last().unwrap()
        || new_item.chars().last().unwrap() != "\"".to_string().chars().last().unwrap()
    {
        communication.send_message(&id, &"Please surround the thruster/thrustee with quotes.");
        return true;
    }
    new_item.pop();
    new_item.remove(0);

    if !thruster && !new_item.contains("_") {
        return false;
    }

    if thruster {
        player.personal_deck.add_thruster(&new_item);
        communication.send_message(&id, &format!("Added \"{}\" to thrusters!", &new_item));
    } else {
        player.personal_deck.add_thrustee(&new_item);
        communication.send_message(&id, &format!("Added \"{}\" to thrustees!", &new_item));
    }

    let lobby_option: std::option::Option<&mut Lobby> = lobby.get_mut(&player.lobby);

    if lobby_option.is_some() {
        let lob: &mut Lobby = lobby_option.unwrap();

        if lob.state == LobbyState::Waiting {
            lob
            .deck
            .thrustees
            .append(&mut player.personal_deck.thrustees.clone());
            lob
            .deck
            .thrusters
            .append(&mut player.personal_deck.thrusters.clone());
        }
    }

    true
}

pub fn show_thrusts(id: Token, lobby: &mut HashMap<i32, Lobby>,
    players: &mut HashMap<Token, player::Player>,
    communication: &mut networking::Networking) {
        
    let player: &mut player::Player = players.get_mut(&id).unwrap();
    let lob: &mut Lobby = lobby.get_mut(&player.lobby).unwrap();

    // for identifier in lob.current_thrusts {
    //     communication.send_message(
    //         &id,
    //         &format!("{}. {}", identifier, lob.current_thrusts.get(&identifier))
    //     );
    // }
    let indexes = &mut lob.index_to_token.keys().collect::<Vec<&i32>>();
    indexes.sort();

    for index in indexes {
        communication.send_message(
            &id,
            &format!("{}. {}", index, lob.current_thrusts.get(&lob.index_to_token.get(&index).unwrap()).unwrap())
        );
    }
}
