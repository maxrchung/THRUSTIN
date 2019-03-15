use crate::networking::Networking;
use crate::player::{Player, PlayerState};
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

    pub thrustee_choices: Vec<String>,

    pub max_thrustee_choices: i32
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
            thrustee_choices: Vec::new(),
            max_thrustee_choices: 3,
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

    
    fn send_message(&self, message: &str, communication: &mut Networking) {
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
        communication: &mut Networking,
    ) {
        let max = 64;
        if let Some(player_p) = players.get_mut(&id) {
            let mut player = player_p.borrow_mut();
            player.lobby = lobby_id.clone();
            player.state = PlayerState::InLobby;

            let mut new_lobby = Lobby::new("".to_string(), max, *lobby_id, &mut player.personal_deck);
            new_lobby.list.push(player_p.clone());

            lobbies.insert(lobby_id.clone(), new_lobby.clone());
            communication.send_message(&id, &format!("Created lobby: {}", lobby_id));
        }
        *lobby_id = *lobby_id + 1;
    }

    pub fn start_game(
        &mut self,
        id: Token,
        communication: &mut Networking,
    ) {
        if !self.is_host(&id) {
            communication.send_message(&id, &format!("Only host can start game!"));
            return;            
        }

        self.state = LobbyState::Playing;

        // Setup new thrustee choices
        for _ in 0..self.max_thrustee_choices {
            if let Some(card) = self.deck.thrustees.pop() {
                self.thrustee_choices.push(card);
            } else {
                self.restart_game(communication);
                return;
            }
        }

        for (i, players) in self.list.iter().enumerate() {
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
	        if i == self.thrustee {
                p.state = PlayerState::Choosing;
                let mut messages = vec!["You are the THRUSTEE. CHOOSE NOW..........<br/>".to_string()];
                messages.extend(self.print_thrustee_choices());
                communication.send_messages(&p.token, messages);
            } else {
                communication.send_message(&p.token, "You are a THRUSTER. waiting for a good THRUSTEE; mmm baby!");
            }
        }
    }

    pub fn leave_lobby(
        &mut self,
        id: Token,
        communication: &mut Networking,
    ) -> bool {
        let pl_ind = self.search_token(&id);
        let (lob_id, name) = {
            let pl = &mut self.list[pl_ind];
            let mut player = pl.borrow_mut();
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
        communication: &mut Networking,
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
        communication: &mut Networking
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

    pub fn switch_host(&mut self, 
		split: std::vec::Vec<&str>, 
		token: Token,
        communication: &mut Networking) {
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

    pub fn kick(&mut self, 
		split: std::vec::Vec<&str>, 
		token: Token,
        communication: &mut Networking
	) {
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

        let mut kick_ind = -1;
        for (i, players) in self.list.iter().enumerate() {
            let pl = players.borrow();
            if pl.name == kick {
                communication.send_message(&pl.token, "ur r kicked!!");
                communication.send_message(&token, &format!("u rly kicedk {} out!", pl.name));
                kick_ind = i as i32;
                break;
            } else {
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

    pub fn print_thrustee_choices(&self) -> Vec<String> {
        let mut messages = vec!["your THRUSTEE Choices:".to_string()];
        for (index, thrustee) in self.thrustee_choices.iter().enumerate() {
            messages.push(format!("{}. {}", &index, &thrustee).to_string());
        }
        messages
    }

    pub fn choose(
        &mut self,
        split: std::vec::Vec<&str>,
        token: Token,
        communication: &mut Networking,
    ) {
        if split.len() < 2 {
            communication.send_message(&token, "ya need to pick a NUMERIC, Boy");
            return;
        }

        match split[1].parse::<i32>() {
            Ok(index) => {
                if index < self.max_thrustee_choices && index >= 0 {

                    // Scope refcell borrow
                    {
                        let mut player = self.list[self.search_token(&token)].borrow_mut();
                        // Removed selected choice
                        let card = self.thrustee_choices.remove(index as usize);
                        self.current_thrustee = card;
                        player.state = PlayerState::Deciding;


                        // Put remaining choices back into thrustees deck
                        for choice in self.thrustee_choices.iter() {
                            self.deck.thrustees.push(choice.clone());
                        }
                        self.thrustee_choices.clear();
                    }

                    // Notify players
                    for (i, player_cell) in self.list.iter().enumerate() {
                        let p = player_cell.borrow();
                        let mut messages = vec![format!("{} has chosen this new THRUSTEE:<br/>{}<br/>", &p.name, &self.current_thrustee).to_string()];

                        if i == self.thrustee {
                            messages.push("get Ready to DECIDE best THRUSTER for THRUSTING!".to_string());
                            communication.send_messages(&p.token, messages);
                        }
                        else {
                            messages.extend(get_thrusters(&p.deck.thrusters));
                            communication.send_messages(&p.token, messages);
                        }
                    }
                } else {
                    communication.send_message(&token, &"That shit's out of bound bro");
                }
            }
            _ => {
                communication.send_message(
                    &token,
                    &"That is an invalid parameter my chieftain, use an index instead dawggo.",
                );
            }
        };
    }

    pub fn decide(
        &mut self,
        split: std::vec::Vec<&str>,
        token: Token,
        communication: &mut Networking,
    ) {
        if split.len() < 2 {
            communication.send_message(&token, "ya need to pick a numbert boi");
            return;
        }

        match split[1].parse::<i32>() {
            Ok(index) => {
                if index < self.current_thrusts.len() as i32 && index >= 0 {
                    // Because of multiple mutable references
                    let mut restart = false;
                    let mut name = String::new();
                    let mut chosen_thrust = String::new();
                    {
                        let mut player = self.list[self.search_token(&token)].borrow_mut();
                        name = player.name.clone();

                        // Get chosen thrust
                        chosen_thrust = self
                            .current_thrusts
                            .remove(&self.index_to_token.get(&index).unwrap())
                            .unwrap();

                        // Clear thrust values
                        self.current_thrusts.clear();
                        self.thrusted_players.clear();

                        // Set current THRUSTEE to THRUSTER state
                        player.state = PlayerState::Playing;

                        // Get new thrustee_choices for next THRUSTEE
                        for _ in 0..self.max_thrustee_choices {
                            if let Some(card) = self.deck.thrustees.pop() {
                                self.thrustee_choices.push(card);
                            } else {
                                restart = true;
                            }
                        }
                    }

                    if restart {
                        self.restart_game(communication);
                    }

                    // Assign next THRUSTEE
                    self.thrustee = (self.thrustee + 1) % self.list.len();

                    // Initialized outside so player.name and chosen_thrust can be kept
                    let common = vec![format!(
                        "{} has chosen this THRUSTER as the chosen THRUST, bois:<br/>{}<br/>",
                        &name, &chosen_thrust
                    )
                    .to_string()];


                    for (i, pl) in self.list.iter().enumerate() {
                        let mut messages = common.clone();

                        // If THRUSTEE, then set him up to be choosing next shit
                        if i == self.thrustee {
                            let mut thrustee_player = pl.borrow_mut();
                            thrustee_player.state = PlayerState::Choosing;
                            messages
                                .push("You are the neXt THRUSTEE! GetT ready to CHOOSE a good THRUSTEE!".to_string());
                            messages
                                .extend(self.print_thrustee_choices());
                        } else {
                            messages.push("get rdy to thrust.....".to_string());
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

    pub fn handle_thrust(
        &mut self,
        split: std::vec::Vec<&str>,
        token: Token,
        communication: &mut Networking,
    ) {
        // Check number of inputs
        if split.len() < 2 {
            communication.send_message(&token, &"Index required!");
            return;
        }

        match split[1].parse::<i32>() {
            Ok(index) => {

                // For handling mut borrow
                let mut restart = false;
                let mut resulting_thrust = String::new();
                {
                    let player_clone = self.list[self.search_token(&token)].clone();
                    let mut player = player_clone.borrow_mut();

                    // Check thrust out of bounds
                    for i in 1..split.len() {
                        let dex = split[i].parse::<i32>().unwrap();
                        if dex >= player.deck.thrusters.len() as i32 || index < 0 {
                            communication.send_message(&token, &"That shit's out of bound bro");
                            return;
                        }
                    }

                    // Check if thrusted
                    for player_token in &self.thrusted_players {
                        if token == *player_token {
                            communication.send_message(
                                &player_token,
                                &format!("You have already THRUSTED, you cannot THRUST again."),
                            );
                            return;
                        }
                    }

                    resulting_thrust = self.current_thrustee.clone();
                    let mut to_remove: std::vec::Vec<String> = Vec::new();
                    // Handle mutliple underscores
                    for i in 1..split.len() {
                        let picked_thruster =
                            player.deck.thrusters[split[i].parse::<usize>().unwrap()].clone();
                        to_remove.push(picked_thruster.clone());
                        resulting_thrust = thrust::Deck::thrust(
                            split[i].parse::<i32>().unwrap(),
                            &picked_thruster,
                            &resulting_thrust,
                        );
                    }

                    // Remove thrusted thrusters
                    let mut updated_thrusters: std::vec::Vec<String> = Vec::new();
                    for thruster in &player.deck.thrusters {
                        if !to_remove.contains(thruster) {
                            updated_thrusters.push(thruster.clone())
                        }
                    }
                    player.deck.thrusters = updated_thrusters;
                    self.thrusted_players.push(player.token.clone());

                    // Handle picked
                    let picked_thruster = player.deck.thrusters.remove(index as usize);
                    let resulting_thrust =
                        thrust::Deck::thrust(index, &picked_thruster, &self.current_thrustee);
                    self.current_thrusts
                        .insert(player.token, resulting_thrust.clone());
                    self.index_to_token
                        .insert((self.current_thrusts.len() - 1) as i32, player.token);



                    // Replenish cards
                    if let Some(card) = self.deck.thrusters.pop() {
                        let replenished_thruster = card;
                        player.deck.thrusters.push(replenished_thruster.clone());

                    } else {
                        restart = true;
                    }
                }

                if restart {
                    self.restart_game(communication);
                }
                else {
                    // Notify message
                    self.send_message(&format!("{}. {}", &(self.current_thrusts.len() as i32 - 1), &resulting_thrust), communication);
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

    pub fn join_lobby(
        input: std::vec::Vec<&str>,
        id: Token,
        lobby: &mut HashMap<i32, Lobby>,
        players: &mut HashMap<Token, Rc<RefCell<Player>>>,
        communication: &mut Networking,
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
                        }
                    else {
                        return;
                    }
                } else {
                    communication.send_message(&id, &format!("Lobby does not exist."));
                }
            }

            _ => communication.send_message(&id, &"Lmao make a lobby first dumbass"),
        }
    }
}

pub fn get_thrusters(thrusters: &Vec<String>) -> Vec<String> {
    let mut messages = vec!["Here are your THRUSTERS:".to_string()];
    for (index, thruster) in thrusters.iter().enumerate() {
        messages.push(format!("{}. {}", &index, &thruster).to_string());
    }
    messages
}

pub fn list_lobby(
    id: Token,
    lobbies: &mut HashMap<i32, Lobby>,
    communication: &mut Networking,
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
    communication: &mut Networking,
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

pub fn list_out_commands(id: Token, communication: &mut Networking) {
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

pub fn list_choose_name_commands(id: Token, communication: &mut Networking) {
    communication.send_messages(
        &id,
        vec![
            "Valid commands:".to_string(),
            "'.help' this is it chief".to_string(),
            "'.name [name]' change your name to [name]".to_string(),
        ],
    );
}

pub fn list_in_commands(id: Token, communication: &mut Networking) {
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

pub fn list_playing_commands(id: Token, communication: &mut Networking) {
    communication.send_messages(
        &id,
        vec![
            "Valid commands:".to_string(),
            "'.help' this is it chief".to_string(),
            "'.thrust [#]' THRUST your [#] card".to_string(),
            "'.thrustee' show the current THRUSTEE".to_string(),
            "'.thrusters' show your THRUSTERS".to_string(),
        ],
    );
}

pub fn list_choosing_commands(id: Token, communication: &mut Networking) {
    communication.send_messages(
        &id,
        vec![
            "Valid commands:".to_string(),
            "'.choose [#]' choose [#] card as THE NEXT THRUSTEE".to_string(),
            "'.help' this is it chief".to_string(),
            "'.thrustee' show the current THRUSTEE".to_string(),
            "'.thrusters' show your THRUSTERS".to_string(),
        ],
    );
}

pub fn list_deciding_commands(id: Token, communication: &mut Networking) {
    communication.send_messages(
        &id,
        vec![
            "Valid commands:".to_string(),
            "'.decide [#]' pick [#] card as THE THRUSTEE".to_string(),
            "'.help' this is it chief".to_string(),
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
    communication: &mut Networking,
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