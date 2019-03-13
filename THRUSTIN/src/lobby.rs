use crate::networking;
use crate::player::Player;
use crate::player::PlayerState;
use crate::thrust;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use ws::util::Token;
use std::rc::Rc;
use std::cell::RefCell;

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
    pub list: std::vec::Vec<Rc<RefCell<Player>>>,

    //max number of players
    pub max: u32,

    //max hand size
    pub hand_size: u32,

    //points
    pub curr_points: std::vec::Vec<u32>,
    pub max_points: u32,

    //lobby id
    pub id: i32,

    //lobby state
    pub state: LobbyState,

    //host of lobby
    pub host: usize,

    //current thrustee
    pub thrustee: usize,

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
            curr_points: std::vec!(0; max as usize),
            max_points: 7,
            host: 0,
            thrustee: 0,
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
        //thread_rng().shuffle(&mut lobby.deck.thrusters);
        //thread_rng().shuffle(&mut lobby.deck.thrustees);
        lobby.deck.thrusters.shuffle(&mut thread_rng());
        lobby.deck.thrustees.shuffle(&mut thread_rng());
        lobby
    }

    ///////////
    //private//
    ///////////
    fn is_host(&self, player: &Token) -> bool {
        self.host == self.search_token(&player)
    }

    fn search_player(&self, player: &Player) -> usize {
        for (i, pl) in self.list.iter().enumerate() {
            let token = pl.borrow().token;
            if token == player.token {
                return i;
            }
        }

        self.list.len()
    }

    fn search_token(&self, token: &Token) -> usize {
        for (i, pl) in self.list.iter().enumerate() {
            let tok = pl.borrow().token;
            if tok == *token {
                return i;
            }
        }

        self.list.len()
    }

    
    fn send_message(&self, message: &str, communication: &mut networking::Networking) {
        for player in &self.list {
            let pl = player.borrow();
            communication.send_message(&pl.token, &message);
        }
    }



    //////////
    //public//
    //////////

    pub fn make_lobby(
        input: std::vec::Vec<&str>,
        id: Token,
        lobby_id: &mut i32,
        lobbies: &mut HashMap<i32, Lobby>,
        players: &mut HashMap<Token, Rc<RefCell<Player>>>,
        communication: &mut networking::Networking,
    ) {
        let max = 64;

        if let Some(player_p) = players.get_mut(&id) {

            let mut player = player_p.borrow_mut();
            player.lobby = lobby_id.clone() as i32;
            player.state = PlayerState::InLobby;

            let mut new_lobby = Lobby::new("".to_string(), max, lobby_id, &mut player.personal_deck);
            new_lobby.list.push(player_p.clone());

            lobbies.insert(lobby_id.clone(), new_lobby.clone());
            communication.send_message(&id, &format!("Created lobby: {}", lobby_id));
        }
        *lobby_id = *lobby_id + 1;
    }

    pub fn start_game(
        &mut self,
        id: Token,
        communication: &mut networking::Networking,
    ) {
        if !self.is_host(&id) {
            communication.send_message(&id, &format!("Only host can start game!"));
            return;            
        }

        self.current_thrustee = self.deck.thrustees.pop().unwrap();
        self.state = LobbyState::Playing;

        let mut next = "".to_string();
        for (i, players) in &mut self.list.iter().enumerate() {
            let mut p = players.borrow_mut();
            p.state = PlayerState::Playing;

            for _ in 0..self.hand_size {
                if let Some(card) = self.deck.thrusters.pop() {
                    p.deck.thrusters.push(card.clone());
                } else {
                    communication
                        .send_message(&id, &format!("Chief, there ain't enough cards to start"));
                    return;
                }
            }

            let instructions = if i == self.thrustee {
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
            if i != self.thrustee {
                messages.extend(get_thrusters(&p.deck.thrusters));
            }
            communication.send_messages(&p.token, messages);
        }

    }


    pub fn leave_lobby(
        &mut self,
        id: Token,
        communication: &mut networking::Networking,
    ) -> bool {

        let pl_ind = self.search_token(&id);
        let (lob_id, name) = {
            let pl = &mut self.list[pl_ind];
            let mut player = pl.borrow_mut();
            //let lob_id = player.lobby;

            player.state = PlayerState::OutOfLobby;
            (player.lobby, player.name.clone())
        };
        self.list.remove(pl_ind);

        communication.send_message(&id, &format!("Left lobby: {}.", lob_id));
        self.send_message(&format!("{} has left the lobby.", name), communication);

        let len = self.list.len();
        if (len != 0) && (len >= self.host) {
            self.host = 0;
            let host = Rc::get_mut(&mut self.list[self.host]).unwrap().borrow().token;
            communication.send_message(&host, "u host now!!");
        }

        len == 0
    }

    
    pub fn list_lobby_players(
        &self,
        id: Token,
        communication: &mut networking::Networking,
    ) {
        let mut messages = Vec::new();

        for pl in &self.list {
            let player = pl.borrow();
            let name = &player.name;

            let mut person = "";
            if &player.token == &id {
                person = " (You)";
            }
            let message = if self.is_host(&player.token) {
                format!("{}: host{}", name, person).to_string()
            } else {
                format!("{}{}", name, person).to_string()
            };

            messages.push(message);
        }

        if messages.is_empty() {
            messages.push(String::from("There's no players lmfao"));
        }

        communication.send_messages(&id, messages);
    }

    
    pub fn restart_game(
        &mut self,
        communication: &mut networking::Networking,
    ) {
/*
        self.state = LobbyState::Waiting;
        self.curr_points = std::vec!(0; self.max as usize);
        self.deck = thrust::Deck::default();
        self.current_thrustee = String::new();
        self.current_thrusts = HashMap::new();
        self.index_to_token = HashMap::new();
        self.thrusted_players = Vec::new();

        //add all personal decks and change to inlobby state
        for token in &self.list {
            let player: &mut Player = players.get_mut(&token).unwrap();
            self.deck
                .thrustees
                .append(&mut player.personal_deck.thrustees.clone());
            self.deck
                .thrusters
                .append(&mut player.personal_deck.thrusters.clone());
            player.deck = thrust::Deck::new();
            player.state = PlayerState::InLobby;
        }

        self.deck.sort();
        self.deck.thrusters.shuffle(&mut thread_rng());
        self.deck.thrustees.shuffle(&mut thread_rng());

        self.send_message(&"Chief called and he said we're outta cards. Game has restarted and put into waiting state.", communication);
*/
    }

    
    pub fn switch_host(&mut self, split: std::vec::Vec<&str>, token: Token,
                       communication: &mut networking::Networking) {
        if token != self.list[self.host].borrow().token {
            communication.send_message(&token, "Only host can change the host!");
            return;
        }

        if split.len() < 2 {
            communication.send_message(&token, "Who's the new host tho");
            return;
        }

        let new_host = split[1];
        if self.list[self.host].borrow().name == new_host {
            communication.send_message(&token, "You're already host!!");
            return;
        }
        
        for (i, players) in self.list.iter().enumerate() {
            let pl = players.borrow();
            if pl.name == new_host {
                self.host = i;
                communication.send_message(&pl.token, "You are now host!");
                communication.send_message(&token, &format!("{} is now host!", pl.name));
                return;
            } 
        }

        communication.send_message(&token, "Player not in lobby.");
    }


    pub fn kick(&mut self, split: std::vec::Vec<&str>, token: Token,
                       communication: &mut networking::Networking) {
        if token != self.list[self.host].borrow().token {
            communication.send_message(&token, "Only host can kick em!");
            return;
        }

        if split.len() < 2 {
            communication.send_message(&token, "who we kickkin");
            return;
        }

        let kick = split[1];
        if self.list[self.host].borrow().name == kick {
            communication.send_message(&token, "u cant kick ursel!!");
            return;
        }
        
        let mut kick_ind : i32 = -1;
        for (i, players) in self.list.iter().enumerate() {
            let pl = players.borrow();
            if pl.name == kick {
                    communication.send_message(&pl.token, "ur r kicked!!");
                    communication.send_message(&token, &format!("u rly kicedk {} out!", pl.name));
                    kick_ind = i as i32;
                    break;
            }
        }

        if kick_ind >= 0 {
            self.list[kick_ind as usize].borrow_mut().state = PlayerState::OutOfLobby;
            self.list[kick_ind as usize].borrow_mut().lobby = -1;

            self.list.remove(kick_ind as usize);
            return;
        }

        communication.send_message(&token, "Player not in lobby.");
    }


    pub fn show_thrustee(&self,
        id: Token,
        communication: &mut networking::Networking,
    ) {
        communication.send_message(&id, &format!("Current THRUSTEE: {}", self.current_thrustee));
    }


    pub fn show_thrusts(&self,
        id: Token,
        communication: &mut networking::Networking,
    ) {
        let indexes = &mut self.index_to_token.keys().collect::<Vec<&i32>>();
        indexes.sort();

        for index in indexes {
            communication.send_message(
                &id,
                &format!(
                    "{}. {}",
                    index,
                    self.current_thrusts
                        .get(&self.index_to_token.get(&index).unwrap())
                        .unwrap()
                ),
            );
        }
    }


    pub fn decide(&mut self,
        split: std::vec::Vec<&str>,
        token: Token,
        communication: &mut networking::Networking,
    ) {

        let player = {
            let ind = self.search_token(&token);
            if ind != self.thrustee {
                communication.send_message(
                    &token,
                    &"You are not allowed to decide because you are a THRUSTER",
                );

                return;
            }
            self.list[ind].borrow().clone()
        };

        if split.len() < 2 {
            communication.send_message(&token, "ya need to pick a numbert boi");
            return;
        }

        match split[1].parse::<i32>() {
            Ok(index) => {
                if index < self.current_thrusts.len() as i32 && index >= 0 {
                    let chosen_thrust = self
                        .current_thrusts
                        .remove(&self.index_to_token.get(&index).unwrap())
                        .unwrap();
                    self.current_thrusts.clear();
                    self.thrusted_players.clear();

                    // This block also helps solve single player mut reference issues
                    let common = vec![format!(
                        "THRUSTEE {} has chosen this THRUSTER as the chosen THRUST, bois: {}<br/>",
                        &player.name, &chosen_thrust
                    )
                                      .to_string()];
                    if let Some(card) = self.deck.thrustees.pop() {
                        self.current_thrustee = card;
                    } else {
                        self.restart_game(communication);
                        communication.send_message(
                            &token,
                            &"chief has notified us and said that we are out of cards",
                        );
                        return;
                    }
                    //lob.current_thrustee = lob.deck.thrustees.pop().unwrap();

                    let next = "".to_string();

                    self.thrustee = (self.thrustee + 1) % self.list.len();

                    for (i, pl) in self.list.iter().enumerate() {
                        let mut messages = common.clone();

                        if i == self.thrustee {
                            messages
                                .push("You are the neXt THRUSTEE! GetT ready to decide!".to_string());
                            messages.push(
                                format!("HERE Is your THRUSTEE: {}", &self.current_thrustee).to_string(),
                            );
                        } else {
                            messages.push("ur a fkin thruster..now.".to_string());
                            messages.push(
                                format!(
                                    "HERE Is the next THRUSTEE for {}: {}",
                                    next, &self.current_thrustee
                                )
                                    .to_string(),
                            );
                            messages.extend(get_thrusters(&pl.borrow().deck.thrusters));
                        }

                        communication.send_messages(&pl.borrow().token, messages);
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

    pub fn handle_thrust(&mut self,
        split: std::vec::Vec<&str>,
        token: Token,
        communication: &mut networking::Networking,
    ) {
        let mut player = {
            let ind = self.search_token(&token);
            if ind == self.thrustee {
                communication.send_message(
                    &token,
                    &"You are not allowed to THRUST because you are a THRUSTEE",
                );
                
                return;

            }
            self.list[ind].borrow().clone()
        };

        if split.len() < 2 {
            communication.send_message(&token, &"Index required!");
            return;
        }


        for player_token in &self.thrusted_players {
            if token == *player_token {
                communication.send_message(
                    &player_token,
                    &format!("You have already THRUSTED, you cannot THRUST again."),
                );
                return;
            }
        }
        /*
        let count = ;
        //check all of inputs within range
        let index = Vec::with_capacity(split.len());
        for i in 1..split.len() {
            if let Some(dex) = split[i].parse::<i32>() {
                if dex >= player.deck.thrusters.len() as i32 || dex < 0 {
                    communication.send_message(&token, &"That shit's out of bound bro");
                    return;
                }
            } else {
                communication.send_message(&token, &"That is an invalid parameter, use an index instead");
                return;
            }
        }

        let mut resulting_thrust: String = self.current_thrustee.clone();
        let mut to_remove: std::vec::Vec<String> = Vec::new();

        // Handle mutliple underscores
        for i in 1..split.len() {
            let picked_thruster = player.deck.thrusters[split[i].parse::<usize>().unwrap()].clone();
            to_remove.push(picked_thruster.clone());
            resulting_thrust = thrust::Deck::thrust(split[i].parse::<i32>().unwrap(), &picked_thruster, &resulting_thrust);
        }

        // Remove thrusted thrusters
        let mut updated_thrusters: std::vec::Vec<String> = Vec::new();
        for thruster in &player.deck.thrusters {
            if !to_remove.contains(thruster) {
                updated_thrusters.push(thruster.clone())
            }
        }
        player.deck.thrusters = updated_thrusters;
        let picked_thruster = player.deck.thrusters.remove(index as usize);

        let resulting_thrust =
            thrust::Deck::thrust(index, &picked_thruster, &self.current_thrustee);

        self.current_thrusts
            .insert(player.token, resulting_thrust.clone());

        self.index_to_token
            .insert((self.current_thrusts.len() - 1) as i32, player.token);

        for pl in self.list.iter() {
            communication.send_message(
                &pl.borrow().token,
                &format!(
                    "{}. {}",
                    &(self.current_thrusts.len() as i32 - 1),
                    &resulting_thrust
                ),
            );
        }

        if let Some(card) = self.deck.thrusters.pop() {
            let replenished_thruster = card;
            player.deck.thrusters.push(replenished_thruster.clone());
        } else {
            self.restart_game(communication);
            communication.send_message(&token, &"Outta cards, we restartin");
            return;
        }

        self.thrusted_players.push(player.token.clone());
         */
    }
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
    players: &mut HashMap<Token, Rc<RefCell<Player>>>,
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
                if let Some(player_p) = players.get_mut(&id) {
                    let mut p = player_p.borrow_mut();
                    p.state = if l.state == LobbyState::Playing {
                        for _ in 0..l.hand_size {
                            if let Some(card) = l.deck.thrusters.pop() {
                                p.deck.thrusters.push(card.clone());
                            } else {
                                l.restart_game(communication);
                                communication
                                    .send_message(&id, &format!("Not enough thrusters to distribute"));
                                return;
                            }
                        }

                        messages.push("You are a THRUSTER.".to_string());
                        messages.push(
                            format!("This is your THRUSTEE: {}", &l.current_thrustee).to_string(),
                        );
                        messages.extend(get_thrusters(&p.deck.thrusters));
                        PlayerState::Playing
                    } else {
                        PlayerState::InLobby
                    };

                    l.send_message(&format!("{} has joined the lobby.", p.name), communication);

                    p.lobby = l.id;
                    l.list.push(player_p.clone());

                    messages.push(format!("Joined: {:#?}", &lobby_id));
                    communication.send_messages(&id, messages);

                    // add players' personal deck (.thrustee/.thruster) to lobby deck
                    l.deck
                        .thrustees
                        .append(&mut p.personal_deck.thrustees.clone());
                    l.deck
                        .thrusters
                        .append(&mut p.personal_deck.thrusters.clone());
                } else {
                    return;
                }
            } else {
                communication.send_message(&id, &format!("Lobby does not exist."));
            }
        }

        _ => communication.send_message(&id, &"Lmao make a lobby first dumbass"),
    }
}

pub fn show_thrusters(
    id: Token,
    players: &mut HashMap<Token, Rc<RefCell<Player>>>,
    communication: &mut networking::Networking,
) {
    let p = players.get_mut(&id).unwrap().borrow();
    communication.send_messages(&p.token, get_thrusters(&p.deck.thrusters));

}


pub fn list_lobby(
    id: Token,
    lobbies: &mut HashMap<i32, Lobby>,
    communication: &mut networking::Networking,
) {
    let mut messages = Vec::new();

    for lob in lobbies.values() {
        let state = match &lob.state {
            LobbyState::Playing => "Playing",
            LobbyState::Waiting => "Waiting",
        };
        messages.push(
            format!(
                "id: {} | {}/{} players | {}",
                lob.id,
                lob.list.len(),
                lob.max,
                state
            )
            .to_string(),
        );
    }

    if messages.is_empty() {
        messages.push("No lobbies bro...".to_string());
    }

    communication.send_messages(&id, messages);
}
pub fn list_all_players(
    id: Token,
    players: &mut HashMap<Token, Rc<RefCell<Player>>>,
    communication: &mut networking::Networking,
) {
    let mut messages = Vec::new();
    for player in players.values() {
        let pl = player.borrow();
        let mut person = "";
        if pl.token == id {
            person = " (You)";
        }
        
        let message = if pl.state == PlayerState::InLobby
            || pl.state == PlayerState::Playing
        {
            format!("{} in {}{}", pl.name, pl.lobby, person).to_string()
        } else {
            format!("{}{}", pl.name, person).to_string()
        };

        messages.push(message);
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

pub fn list_choose_name_commands(id: Token, communication: &mut networking::Networking) {
    communication.send_messages(
        &id,
        vec![
            "Valid commands:".to_string(),
            "'.help' this is it chief".to_string(),
            "'.name [name]' change your name to [name]".to_string(),
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
            "'.name [name]' change your name to [name]".to_string(),
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
    players: &mut HashMap<Token, Rc<RefCell<Player>>>,
    communication: &mut networking::Networking,
    thruster: bool,
) -> bool {
/*
    if input.len() < 2 {
        communication.send_message(&id, &"Thruster/thrustee required!");
        return true;
    }

    let player: &mut Player = players.get_mut(&id).unwrap();

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

    if let Some(lob) = lobby.get_mut(&player.lobby) {
        if lob.state == LobbyState::Waiting {
            lob.deck
                .thrustees
                .append(&mut player.personal_deck.thrustees.clone());
            lob.deck
                .thrusters
                .append(&mut player.personal_deck.thrusters.clone());
        }
    }
*/
    true
}

