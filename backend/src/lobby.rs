use crate::player::{Player, PlayerState};
use crate::thrust;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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

    //max hand size
    pub hand_size: u32,

    //points
    pub max_points: u32,

    //lobby id
    pub id: i32,

    //lobby state
    pub state: LobbyState,

    //host of lobby
    //pub host: usize,
    pub host: Rc<RefCell<Player>>,

    //current thrustee (player)
    pub thrustee: usize,

    pub deck: thrust::Deck,
    //current thrustee (card)
    pub current_thrustee: String,

    pub current_thrusts: HashMap<u32, String>,
    //maps thrust index to token (end me)
    pub index_to_token: HashMap<i32, u32>,

    pub thrusted_players: Vec<u32>,

    pub thrustee_choices: Vec<String>,

    pub max_thrustee_choices: i32,

    pub use_house: bool,
}

impl Lobby {
    fn new(player: &Rc<RefCell<Player>>, pw: String, id: i32) -> Lobby {
        let mut lobby = Lobby {
            pw: pw,
            list: Vec::new(),
            id: id,
            state: LobbyState::Waiting,
            hand_size: 5,
            max_points: 7,
            host: player.clone(),
            thrustee: 0,
            thrustee_choices: Vec::new(),
            max_thrustee_choices: 3,
            deck: thrust::Deck::new(),
            current_thrustee: String::new(),
            current_thrusts: HashMap::new(),
            index_to_token: HashMap::new(),
            thrusted_players: Vec::new(),
            use_house: true,
        };

        //addin "personal decks" to lobby(default) deck. .
        Lobby::add_pers_deck_to_lob(&mut lobby, &mut player.borrow_mut());
        lobby
    }

    fn new_endless(player: &Rc<RefCell<Player>>) -> Lobby {
        let lobby = Lobby {
            pw: "".to_string(),
            list: Vec::new(),
            id: 0,
            state: LobbyState::Waiting,
            hand_size: 5,
            max_points: 7,
            host: player.clone(),
            thrustee: 0,
            thrustee_choices: Vec::new(),
            max_thrustee_choices: 3,
            deck: thrust::Deck::new(),
            current_thrustee: String::new(),
            current_thrusts: HashMap::new(),
            index_to_token: HashMap::new(),
            thrusted_players: Vec::new(),
            use_house: true,
        };
        lobby
    }

    ///////////
    //private//
    ///////////
    fn is_host(&self, player: u32) -> bool {
        (self.host.borrow().token == player)
            && (self.host.borrow().name != "EndlessLobbyChiefDoggo".to_string())
    }

    fn search_token(&self, token: u32) -> Option<usize> {
        for (i, pl) in self.list.iter().enumerate() {
            let tok = pl.borrow().token;
            if tok == token {
                return Some(i);
            }
        }

        None
    }

    fn send_message(&self, message: &str) {
        for pl in &self.list {
            pl.borrow().send_message(&message);
        }
    }

    fn send_messages(&self, messages: &Vec<String>) {
        for pl in &self.list {
            pl.borrow().send_messages(&messages);
        }
    }

    //////////
    //public//
    //////////

    //////////////////
    //general stuff?//
    //////////////////

    pub fn add_pers_deck_to_lob(lob: &mut Lobby, pl: &mut std::cell::RefMut<Player>) {
        lob.deck
            .thrustees
            .append(&mut pl.personal_deck.thrustees.clone());
        lob.deck
            .thrusters
            .append(&mut pl.personal_deck.thrusters.clone());
    }

    pub fn remove_pers_deck_from_lob(lob: &mut Lobby, pl: &mut std::cell::RefMut<Player>) {
        lob.deck
            .thrustees
            .retain(|thrustee| (!pl.personal_deck.thrustees.contains(&thrustee)));
        lob.deck
            .thrustees
            .retain(|thruster| (!pl.personal_deck.thrusters.contains(&thruster)));
    }

    pub fn shuffle_deck(lobby: &mut Lobby) {
        lobby.deck.sort();
        lobby.deck.thrusters.shuffle(&mut thread_rng());
        lobby.deck.thrustees.shuffle(&mut thread_rng());
    }

    pub fn make_endless_lobby(
        pl_rc: &Rc<RefCell<Player>>,
        lobby_id: &mut i32,
        lobbies: &mut HashMap<i32, Lobby>,
    ) {
        let mut new_lobby = Lobby::new_endless(pl_rc);
        new_lobby.start_endless();

        lobbies.insert(lobby_id.clone(), new_lobby.clone());
    }

    pub fn make_lobby(
        input: std::vec::Vec<&str>,
        pl_rc: Rc<RefCell<Player>>,
        lobby_id: &mut i32,
        lobbies: &mut HashMap<i32, Lobby>,
    ) {
        let mut new_lobby = Lobby::new(&pl_rc, "".to_string(), *lobby_id);

        let mut pl = pl_rc.borrow_mut();

        pl.lobby = lobby_id.clone();
        pl.state = PlayerState::InLobby;

        new_lobby.list.push(pl_rc.clone());

        lobbies.insert(lobby_id.clone(), new_lobby.clone());
        pl.send_message(&format!("Created lobby: {}", lobby_id));

        *lobby_id = *lobby_id + 1;
    }

    pub fn set_password(&mut self, input: std::vec::Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
        let pl = pl_rc.borrow();
        if !self.is_host(pl.token) {
            pl.send_message("only chief sets password!!!");
            return;
        }

        if input.len() < 2 {
            pl.send_message("?? what's the pass boss??");
            return;
        }

        let password = input[1];
        self.pw = password.to_string();
    }

    pub fn list_lobby_players(&self, pl_rc: Rc<RefCell<Player>>) {
        let pl = pl_rc.borrow();
        let mut messages = Vec::new();

        for pl_i in &self.list {
            let pl_i = pl_i.borrow();
            let name = &pl_i.name;

            let mut person = "";
            if &pl_i.token == &pl.token {
                person = " (You)";
            }

            let message = if self.is_host(pl_i.token) {
                format!("{}: chief{}", name, person).to_string()
            } else {
                format!("{}{}", name, person).to_string()
            };

            messages.push(message);
        }

        if messages.is_empty() {
            messages.push(String::from("There's no players lmfao"));
        }

        pl.send_messages(&messages);
    }

    pub fn info(&self, pl_rc: Rc<RefCell<Player>>) {
        let pl = pl_rc.borrow();
        let mut info = Vec::new();
        info.push(format!("\\\\Lobby info//"));
        info.push(format!("Name: {}", self.id));
        info.push(format!("Max points: {}", self.max_points));

        if self.is_host(pl.token) {
            info.push(format!("Pw: {}", self.pw));
        }

        pl.send_messages(&info);
    }

    pub fn point_max(&mut self, input: std::vec::Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
        let pl = pl_rc.borrow();
        if !self.is_host(pl.token) {
            pl.send_message("only chief sets points!");
            return;
        }

        if input.len() < 2 {
            pl.send_message("ya gotta set the new limit");
            return;
        }

        match input[1].to_string().parse::<u32>() {
            Ok(max) => {
                if max == 0 {
                    pl.send_message("bro dont make it 0 wtf man");
                    return;
                }
                self.max_points = max;
                pl.send_message(&format!("max points set to {}", self.max_points));
            }

            _ => pl.send_message(&"only numbers dude!!!"),
        }
    }

    pub fn switch_host(&mut self, input: std::vec::Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
        let pl = pl_rc.borrow();
        if !self.is_host(pl.token) {
            pl.send_message("Only chief can change the chief!");
            return;
        }

        if input.len() < 2 {
            pl.send_message("Who's the new chief tho");
            return;
        }

        let new_host = input[1];
        if self.host.borrow().name == new_host {
            pl.send_message("You're already chief!!");
            return;
        }

        for players_ in self.list.iter() {
            let players = players_.borrow();
            if players.name == new_host {
                self.host = players_.clone();
                players.send_message("You are now chief!");
                pl.send_message(&format!("{} is now chief!", players.name));
                return;
            }
        }

        pl.send_message("Player not in lobby.");
    }

    pub fn kick(&mut self, input: std::vec::Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
        let pl = pl_rc.borrow();
        if !self.is_host(pl.token) {
            pl.send_message("Only chief can kick em!");
            return;
        }

        if input.len() < 2 {
            pl.send_message("who we kickkin? TELL ME!");
            return;
        }

        let kick = input[1];
        if self.host.borrow().name == kick {
            pl.send_message("u cant kick ursel!!");
            return;
        }

        let mut kick_ind = -1;
        for (i, players) in self.list.iter().enumerate() {
            let players = players.borrow();
            if players.name == kick {
                kick_ind = i as i32;
                break;
            }
        }

        if kick_ind >= 0 {
            self.leave_lobby(self.list[kick_ind as usize].clone());
            return;
        }

        pl.send_message("Player not in lobby.");
    }

    pub fn handle_join_cases(
        pl: &std::cell::RefMut<Player>,
        lob: &Lobby,
        wait: &mut bool,
        messages: &mut Vec<String>,
    ) {
        let thrustee = lob.list[lob.thrustee].borrow();

        match thrustee.state {
            PlayerState::Choosing => {
                *wait = true;
                messages.push(
                    "THRUSTEE is currently CHOOSING next THRUSTEE. Hold on tight!".to_string(),
                );
            }

            PlayerState::Deciding => {
                messages
                    .push(format!("This is your THRUSTEE: {}<br/>", &lob.current_thrustee).to_string());
                messages.extend(get_thrusters(&pl.deck.thrusters));
            }

            _ => (),
        }
    }

    pub fn get_joining_pl_state(
        lob: &mut Lobby,
        pl: &mut std::cell::RefMut<Player>,
        messages: &mut Vec<String>,
        pl_rc: &Rc<RefCell<Player>>,
    ) -> Option<PlayerState> {
        // Clear dude's deck beforehand
        pl.deck.thrusters.clear();
        // Distribute thrusters to player
        for _ in 0..lob.hand_size {
            if let Some(card) = lob.deck.thrusters.pop() {
                pl.deck.thrusters.push(card.clone());
            } else {
                lob.restart_game();
                pl.send_message("Not enough THRUSTERS to distribute");
                return None;
            }
        }

        let mut wait: bool = false;

        // If lobby was empty before this guy joined, then they become thrustee, otherwise, basically do what normal join_lobby does (yea this is fucked fk me doggo)
        if lob.list.len() == 0 {
            pl.state = PlayerState::Choosing;
            let mut messages =
                vec!["Welcome to the 『Endless Lobby』, big doggo. You lucky, family, you are THRUSTEE!!!!.. . Choose now...    .".to_string()];
            messages.extend(lob.print_thrustee_choices());
            pl.send_messages(&messages);
            pl.lobby = lob.id;
            lob.list.push(pl_rc.clone());
            return None; //dude lmao
        } else {
            Lobby::handle_join_cases(&pl, &lob, &mut wait, messages);
        }

        if wait {
            return Some(PlayerState::Waiting);
        } else {
            return Some(PlayerState::Playing);
        }
    }

    pub fn join_endless(
        input: std::vec::Vec<&str>,
        pl_rc: Rc<RefCell<Player>>,
        lobby: &mut HashMap<i32, Lobby>,
        lobby_id: &i32,
    ) {
        let mut pl = pl_rc.borrow_mut();
        let mut messages = Vec::new();
        if let Some(mut lob) = lobby.get_mut(&lobby_id) {
            messages.push(format!("Joined: {:#?}", &lobby_id));

            // Set points to 0 (just in case?)
            pl.points = 0;

            // add players' personal deck (.thrustee/.thruster) to lobby deck
            Lobby::add_pers_deck_to_lob(lob, &mut pl);

            pl.state = {
                if let Some(state) =
                    Lobby::get_joining_pl_state(&mut lob, &mut pl, &mut messages, &pl_rc)
                {
                    state
                } else {
                    return;
                }
            };
            lob.send_message(&format!("{} has joined the lobby.", pl.name));
            // adding the new player to lobby
            pl.lobby = lob.id;
            lob.list.push(pl_rc.clone());
            pl.send_messages(&messages);
        } else {
            pl.send_message("Lobby does not exist.");
        }
    }

    pub fn join_lobby(
        input: std::vec::Vec<&str>,
        pl_rc: Rc<RefCell<Player>>,
        lobby: &mut HashMap<i32, Lobby>,
    ) {
        if input.len() < 2 {
            let pl = pl_rc.borrow();
            pl.send_message("Lobby name required!");
            return;
        }

        match input[1].to_string().parse::<i32>() {
            Ok(lobby_id) => {
                // Handle joining endless lobby (lmao)
                if lobby_id == 0 {
                    Lobby::join_endless(input, pl_rc, lobby, &lobby_id);
                    return;
                }
                let mut pl = pl_rc.borrow_mut();

                let mut messages = Vec::new();
                if let Some(mut lob) = lobby.get_mut(&lobby_id) {
                    //Lobby Password Check
                    if lob.pw != "" {
                        if input.len() < 3 {
                            pl.send_message("Ya need a password BR)");
                            return;
                        } else if lob.pw != input[2] {
                            pl.send_message("loll wrong pw haha");
                            return;
                        }
                    }

                    messages.push(format!("Joined: {:#?}", &lobby_id));

                    // Set points to 0 (just in case?)
                    pl.points = 0;

                    // add players' personal deck (.thrustee/.thruster) to lobby deck
                    Lobby::add_pers_deck_to_lob(lob, &mut pl);

                    pl.state = if lob.state == LobbyState::Playing {
                        if let Some(state) =
                            Lobby::get_joining_pl_state(&mut lob, &mut pl, &mut messages, &pl_rc)
                        {
                            state
                        } else {
                            return;
                        }
                    } else {
                        PlayerState::InLobby
                    };

                    lob.send_message(&format!("{} has joined the lobby.", pl.name));
                    // adding the new player to lobby
                    pl.lobby = lob.id;
                    lob.list.push(pl_rc.clone());
                    pl.send_messages(&messages);
                } else {
                    pl.send_message("Lobby does not exist.");
                }
            }

            _ => {
                let pl = pl_rc.borrow();
                pl.send_message("nibba that is a invalid input my nibba")
            } //i love rust
        }
    }

    pub fn leave_lobby(&mut self, pl_rc: Rc<RefCell<Player>>) {
        {
            let pl = pl_rc.borrow();
            
            // Much easier to clear single person lobby than multiple
            if self.list.len() == 1 {
                self.list.clear();
            }
            else {
                let pl_ind = self.search_token(pl.token).unwrap();
                let next_ind = (pl_ind + 1) % self.list.len();

                // Set up messages
                let mut messages = vec![String::from(format!("{} left the lobby..", pl.name))];
                // Handle if player is host
                if pl.token == self.host.borrow().token {
                    self.host = self.list[next_ind].clone();
                    messages.push(String::from(format!("Chief left so now we got a new one --> {}", self.host.borrow().name)));
                }

                // Flag that checks if host/chief was changed
                let mut new_host = false;
                // Handle if player is Choosing
                if pl.state == PlayerState::Choosing {
                    self.thrustee = next_ind;
                    // Next player chooses and replenish cards
                    let mut next = self.list[next_ind].borrow_mut();
                    next.state = PlayerState::Choosing;
                    messages.push(String::from(format!("Lol yo bro 'cause the THRUSTEE left {} is choosin' the next THRUSTEE now!", next.name)));
                    new_host = true;
                }
                // Handle if player is Deciding
                else if pl.state == PlayerState::Deciding {
                    self.thrustee = next_ind;
                    // Next player decides
                    let mut next = self.list[next_ind].borrow_mut();
                    next.state = PlayerState::Deciding;
                    messages.push(String::from(format!("Lmao the THRUSTEE left and you're next in line, so {} gets to decide which THRUST wins lmfao.", next.name)));
                }

                // Remove player
                self.list.remove(pl_ind);

                // Send appropriate message
                for pl_rc in &self.list {
                    let pl = pl_rc.borrow();
                    if self.is_host(pl.token) && new_host {
                        let mut host_messages = messages.clone();
                        // Add a linebreak for better visibility
                        host_messages.last_mut().unwrap().push_str("<br/>");
                        // Notify chief of choices
                        host_messages.extend(self.print_thrustee_choices());
                        pl.send_messages(&host_messages);
                    }
                    else {
                        pl.send_messages(&messages);
                    }
                }
            }
        }
        
        // Player mut and specific operations
        let mut pl_mut = pl_rc.borrow_mut();
        pl_mut.lobby = -1;
        pl_mut.state = PlayerState::OutOfLobby;
        pl_mut.send_message("You left the lobby okay!");
    }

    pub fn toggle_house(&mut self, pl_rc: Rc<RefCell<Player>>) {
        let pl = pl_rc.borrow();
        self.use_house = !self.use_house;
        if self.use_house {
            pl.send_message(&"Now using house cards!");
        } else {
            pl.send_message(&"No longer using house cards!...");
        }
    }

    /////////////////
    //game commands//
    /////////////////

    pub fn start_endless(&mut self) {
        self.state = LobbyState::Playing;

        // Add in house cards to lobby deck if bool is true
        if self.use_house {
            let default_deck = thrust::Deck::default();
            self.deck.thrusters.extend(default_deck.thrusters);
            self.deck.thrustees.extend(default_deck.thrustees);
        }

        Lobby::shuffle_deck(self);

        // Setup new thrustee choices
        for _ in 0..self.max_thrustee_choices {
            if let Some(card) = self.deck.thrustees.pop() {
                self.thrustee_choices.push(card);
            } else {
                self.restart_game();
                return;
            }
        }
    }

    pub fn start_game(&mut self, pl_rc: Rc<RefCell<Player>>) {
        {
            let pl = pl_rc.borrow();

            if !self.is_host(pl.token) {
                pl.send_message(&format!("Only host can start game!"));
                return;
            }
        }

        self.state = LobbyState::Playing;

        // Add in house cards to lobby deck if bool is true
        if self.use_house {
            let default_deck = thrust::Deck::default();
            self.deck.thrusters.extend(default_deck.thrusters);
            self.deck.thrustees.extend(default_deck.thrustees);
        }

        Lobby::shuffle_deck(self);

        // Setup new thrustee choices
        for _ in 0..self.max_thrustee_choices {
            if let Some(card) = self.deck.thrustees.pop() {
                self.thrustee_choices.push(card);
            } else {
                self.restart_game();
                return;
            }
        }

        for (i, pl) in self.list.iter().enumerate() {
            let mut pl = pl.borrow_mut();
            pl.state = PlayerState::Waiting;

            for _ in 0..self.hand_size {
                if let Some(card) = self.deck.thrusters.pop() {
                    pl.deck.thrusters.push(card.clone());
                } else {
                    self.host
                        .borrow()
                        .send_message(&"Chief, there ain't enough cards to start");
                    return;
                }
            }

            if i == self.thrustee {
                pl.state = PlayerState::Choosing;
                let mut messages =
                    vec!["You are the THRUSTEE. Choose NOW..........<br/>".to_string()];
                messages.extend(self.print_thrustee_choices());
                pl.send_messages(&messages);
            } else {
                pl.send_message("You are a THRUSTER. waiting for a good THRUSTEE; mmm baby!");
            }
        }
    }

    pub fn clear_game(&mut self) {
        self.state = LobbyState::Waiting;
        self.deck = thrust::Deck::default();
        self.current_thrustee = String::new();
        self.current_thrusts = HashMap::new();
        self.index_to_token = HashMap::new();
        self.thrusted_players = Vec::new();
        self.thrustee_choices = Vec::new();

        //add all personal decks and change to inlobby state
        for rc in &self.list {
            let mut player = rc.borrow_mut();
            player.points = 0; // RESET PTS

            self.deck
                .thrustees
                .append(&mut player.personal_deck.thrustees.clone());
            self.deck
                .thrusters
                .append(&mut player.personal_deck.thrusters.clone());
            player.deck = thrust::Deck::new();
            player.state = PlayerState::InLobby;
        }
        Lobby::shuffle_deck(self);
    }

    pub fn clear_endless(&mut self) {
        self.deck = thrust::Deck::default();

        // readd all personal decks to endless lobby
        for rc in &self.list {
            let mut player = rc.borrow_mut();
            self.deck
                .thrustees
                .append(&mut player.personal_deck.thrustees.clone());
            self.deck
                .thrusters
                .append(&mut player.personal_deck.thrusters.clone());
        }
        Lobby::shuffle_deck(self);

        // Handles if lobby for restarts when no one is in there for some reason (not enough house cards during testing)
        if self.list.len() != 0 {
            let mut pl = self.list[self.thrustee].borrow_mut();
            pl.state = PlayerState::Choosing;
            let mut messages =
                vec!["YOOOOOOO!! Endless lobby just ran out of cards. Don't worry, though! EndlessLobbyChiefDoggo helped out and replenished the cards!".to_string(),
                    "You are the THRUSTEE of Endless Lobby! Choose now....".to_string()];
            messages.extend(self.print_thrustee_choices());
            pl.send_messages(&messages);
        }
    }

    pub fn handle_winner(&mut self, winner_dex: usize) {
        self.clear_game();
        let winner_name = self.list[winner_dex].borrow_mut().name.clone();
        self.send_message(&format!("Congratulations, {}! You're Winner! Everyone else, You're Loser! Game has been put into waiting state, THRUSTIN'ers!", winner_name));
    }

    pub fn restart_game(&mut self) {
        if self.host.borrow().name != "EndlessLobbyChiefDoggo".to_string() {
            self.clear_game();
            self.send_message(&"Chief called and he said we're outta cards. Game has restarted and put into waiting state.");
        } else {
            self.clear_endless();
        }
    }

    pub fn print_thrustee_choices(&self) -> Vec<String> {
        let mut messages = vec!["your THRUSTEE Choices:".to_string()];
        for (index, thrustee) in self.thrustee_choices.iter().enumerate() {
            messages.push(format!("{}. {}", &index, &thrustee).to_string());
        }
        messages
    }

    pub fn choose(&mut self, input: std::vec::Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
        {
            let pl = pl_rc.borrow();

            if input.len() < 2 {
                pl.send_message("ya need to pick a NUMERIC, Boy");
                return;
            }
        }

        match input[1].parse::<i32>() {
            Ok(index) => {
                if index < self.max_thrustee_choices && index >= 0 {
                    // Scope refcell borrow
                    let mut name;
                    {
                        let mut pl = pl_rc.borrow_mut();

                        // Removed selected choice
                        let card = self.thrustee_choices.remove(index as usize);
                        self.current_thrustee = card;
                        pl.state = PlayerState::Deciding;

                        // Clear choices
                        self.thrustee_choices.clear();

                        name = pl.name.clone();
                    }

                    // Notify players
                    for (i, player_cell) in self.list.iter().enumerate() {
                        let mut p = player_cell.borrow_mut();
                        let mut messages = vec![format!(
                            "{} has chosen this new THRUSTEE:<br/>{}<br/>",
                            name, &self.current_thrustee
                        )
                        .to_string()];

                        // Change Waiting players to Playing
                        if p.state == PlayerState::Waiting {
                            p.state = PlayerState::Playing;
                        }

                        if i == self.thrustee {
                            messages.push(
                                "get Ready to decide best THRUSTER for THRUSTING!".to_string(),
                            );
                            p.send_messages(&messages);
                        } else {
                            messages.extend(get_thrusters(&p.deck.thrusters));
                            p.send_messages(&messages);
                        }
                    }
                } else {
                    pl_rc.borrow().send_message("That shit's out of bound bro");
                }
            }
            _ => {
                pl_rc.borrow().send_message(
                    "That is an invalid parameter my chieftain, use an index instead dawggo.",
                );
            }
        };
    }

    pub fn decide(&mut self, input: std::vec::Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
        {
            let pl = pl_rc.borrow();
            if input.len() < 2 {
                pl.send_message("ya need to pick a numbert boi");
                return;
            }
        }

        match input[1].parse::<i32>() {
            Ok(index) => {
                if index < self.current_thrusts.len() as i32 && index >= 0 {
                    // Because of multiple mutable references
                    let (restart, name, chosen_thrust) = 
                    {
                        let mut pl = pl_rc.borrow_mut();
                        let name = pl.name.clone();

                        // Get chosen thrust
                        let chosen_thrust = self
                            .current_thrusts
                            .remove(&self.index_to_token.get(&index).unwrap())
                            .unwrap();

                        // Clear thrust values
                        self.current_thrusts.clear();
                        self.thrusted_players.clear();

                        // Set current THRUSTEE to THRUSTER state
                        pl.state = PlayerState::Waiting;

                        // Get new thrustee_choices for next THRUSTEE
                        let mut restart = false;
                        for _ in 0..self.max_thrustee_choices {
                            if let Some(card) = self.deck.thrustees.pop() {
                                self.thrustee_choices.push(card);
                            } else {
                                restart = true;
                            }
                        }

                        (restart, name, chosen_thrust)
                    };

                    let (chosen_thruster_pts, chosen_thruster_name) = {
                        // Assign picked thruster a point
                        match self.search_token(*self.index_to_token.get(&index).unwrap()) {
                            Some(tkn) => {
                                let (pts, name) = {
                                    let mut chosen_thruster = self.list[tkn].borrow_mut();
                                    chosen_thruster.points += 1;
                                    (chosen_thruster.points.clone(), chosen_thruster.name.clone())
                                };

                                // Check if winner
                                if pts >= self.max_points {
                                    self.handle_winner(tkn);
                                    return;
                                }
                                (pts, name)
                            }
                            None => (0, String::from("THE GUY WHO LEFT (look imma be real it's easier right now for me to jam some placeholder text here than properly handle THRUSTERS who leave the game after THRUSTING yeah aite we're just gonna clear his points)"))
                        }
                    };

                    if restart {
                        self.restart_game();
                        return;
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

                        messages.push(format!(
                            "The winning THRUSTER, {} now has {} point(s)! Watch out!<br/>",
                            &chosen_thruster_name, &chosen_thruster_pts
                        ));

                        // If THRUSTEE, then set him up to be choosing next shit
                        if i == self.thrustee {
                            let mut thrustee_player = pl.borrow_mut();
                            thrustee_player.state = PlayerState::Choosing;
                            messages.push(
                                "You are the neXt THRUSTEE! GetT ready to CHOOSE a good THRUSTEE!"
                                    .to_string(),
                            );
                            messages.extend(self.print_thrustee_choices());
                        } else {
                            messages.push("get rdy to THRUST.....".to_string());
                        }
                        pl.borrow().send_messages(&messages);
                    }
                } else {
                    pl_rc.borrow().send_message("That shit's out of bound bro");
                }
            }
            _ => {
                pl_rc
                    .borrow()
                    .send_message("That is an invalid parameter, use an index instead");
            }
        };
    }

    pub fn handle_thrust(&mut self, input: std::vec::Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
        {
            let pl = pl_rc.borrow();
            // Check number of inputs
            if input.len() < 2 {
                pl.send_message(&"Index required!");
                return;
            }
        }

        match input[1].parse::<i32>() {
            Ok(index) => {
                // For handling mut borrow
                let mut restart = false;
                let mut resulting_thrust = String::new();
                {
                    let mut pl = pl_rc.borrow_mut();

                    // Check correct # of thrusters
                    let num_thrusters = input.len() as i32 - 1;
                    let num_underscore = thrust::Deck::count_underscore(&self.current_thrustee);
                    if num_thrusters != num_underscore {
                        pl.send_message("bro that ain't the right number of THRUSTERS");
                        return;
                    }
                    let mut indexes: Vec<i32> = Vec::new();
                    // Check thrust out of bounds
                    for i in 1..input.len() {
                        let dex = input[i].parse::<i32>().unwrap();
                        if indexes.contains(&dex) {
                            // Check if dupes
                            pl.send_message("y'ain't allowed to thrust duplicate THRUSTERS broski");
                            return;
                        }
                        indexes.push(dex);
                        if dex >= pl.deck.thrusters.len() as i32 || index < 0 {
                            pl.send_message("That shit's out of bound bro");
                            return;
                        }
                    }

                    // Check if thrusted
                    for player_token in &self.thrusted_players {
                        if pl.token == *player_token {
                            pl.send_message("You have already THRUSTED, you cannot THRUST again.");
                            return;
                        }
                    }

                    resulting_thrust = self.current_thrustee.clone();
                    let mut to_remove: std::vec::Vec<String> = Vec::new();
                    // Handle mutliple underscores
                    for i in 1..input.len() {
                        let picked_thruster =
                            pl.deck.thrusters[input[i].parse::<usize>().unwrap()].clone();
                        to_remove.push(picked_thruster.clone());
                        // Surround with <u> to underline text
                        let formatted_thruster = format!("<u>{}</u>", picked_thruster);
                        resulting_thrust = thrust::Deck::thrust(
                            input[i].parse::<i32>().unwrap(),
                            &formatted_thruster,
                            &resulting_thrust,
                        );
                    }

                    // Remove thrusted thrusters
                    let mut updated_thrusters: std::vec::Vec<String> = Vec::new();
                    for thruster in &pl.deck.thrusters {
                        if !to_remove.contains(thruster) {
                            updated_thrusters.push(thruster.clone())
                        }
                    }
                    pl.deck.thrusters = updated_thrusters;
                    self.thrusted_players.push(pl.token.clone());

                    // Handle picked
                    self.current_thrusts
                        .insert(pl.token, resulting_thrust.clone());
                    self.index_to_token
                        .insert((self.current_thrusts.len() - 1) as i32, pl.token);

                    // Replenish cards
                    if let Some(card) = self.deck.thrusters.pop() {
                        let replenished_thruster = card;
                        pl.deck.thrusters.push(replenished_thruster.clone());
                    } else {
                        restart = true;
                    }
                }

                if restart {
                    self.restart_game();
                    return;
                } else {
                    // Notify message
                    self.send_message(&format!(
                        "{}. {}",
                        &(self.current_thrusts.len() as i32 - 1),
                        &resulting_thrust
                    ));
                }
            }
            _ => {
                pl_rc
                    .borrow()
                    .send_message("That is an invalid parameter, use an index instead");
            }
        };
    }

    pub fn display_points(&self, pl: Rc<RefCell<Player>>) {
        let pl = pl.borrow();
        let mut messages = Vec::new();
        messages.push(format!("Max: {}", self.max_points));

        for rc in &self.list {
            let player = rc.borrow();
            messages.push(format!("{}: {}", player.name, player.points));
        }

        pl.send_messages(&messages);
    }
}

pub fn get_thrusters(thrusters: &Vec<String>) -> Vec<String> {
    let mut messages = vec!["Here are your THRUSTERS:".to_string()];
    for (index, thruster) in thrusters.iter().enumerate() {
        messages.push(format!("{}. {}", &index, &thruster).to_string());
    }
    messages
}

pub fn list_lobby(pl_rc: Rc<RefCell<Player>>, lobbies: &mut HashMap<i32, Lobby>) {
    let pl = pl_rc.borrow();
    let mut messages = Vec::new();

    for lob in lobbies.values() {
        let state = match &lob.state {
            LobbyState::Playing => "Playing",
            LobbyState::Waiting => "Waiting",
        };
        messages.push(
            format!(
                "id: {} | {} players | {}",
                lob.id,
                lob.list.len(),
                state
            )
            .to_string(),
        );
    }

    if messages.is_empty() {
        messages.push("No lobbies bro...".to_string());
    }

    pl.send_messages(&messages);
}

pub fn list_all_players(
    pl_rc: Rc<RefCell<Player>>,
    players: &mut HashMap<u32, Rc<RefCell<Player>>>,
) {
    let pl = pl_rc.borrow();
    let mut messages = Vec::new();

    for player in players.values() {
        let pl_i = player.borrow();
        let mut person = "";
        if pl_i.token == pl.token {
            person = " (You)";
        }

        let message = if pl_i.state == PlayerState::InLobby || pl_i.state == PlayerState::Playing {
            format!("{} in {}{}", pl_i.name, pl_i.lobby, person).to_string()
        } else {
            format!("{}{}", pl_i.name, person).to_string()
        };

        messages.push(message);
    }
    pl.send_messages(&messages);
}

pub fn handle_thrusteer_commands(
    input: &std::vec::Vec<&str>,
    pl_rc: Rc<RefCell<Player>>,
    lobby: &mut HashMap<i32, Lobby>,
) {
    let pl = pl_rc.borrow_mut();

    if input.len() < 2 {
        display_deck(pl);
        return;
    }

    let mut new_item = String::new();
    for s in input.iter().skip(1) {
        new_item.push_str(s);
        new_item.push_str(" ");
    }

    new_item.pop();

    if let (Some(beginning), Some(ending)) = (new_item.chars().next(), new_item.chars().next()) {
        let quotation = "\"".to_string().chars().last().unwrap();

        if beginning != quotation || ending != quotation {
            pl.send_message("Please surround the THRUST with quotes.");
            return;
        }
    } else {
        display_deck(pl);
        return;
    }

    new_item.pop();
    new_item.remove(0);

    if new_item.contains("_") {
        add_thrustee(pl, lobby, new_item.clone());
        return;
    } else {
        add_thruster(pl, lobby, new_item.clone());
        return;
    }
}

pub fn add_thruster(
    mut pl: std::cell::RefMut<Player>,
    lobby: &mut HashMap<i32, Lobby>,
    new_item: String,
) {
    pl.personal_deck.add_thruster(&new_item);
    pl.send_message(&format!("Added \"{}\" to THRUSTERS!", &new_item));

    if let Some(lob) = lobby.get_mut(&pl.lobby) {
        if lob.state == LobbyState::Waiting {
            Lobby::add_pers_deck_to_lob(lob, &mut pl);
        }
    }
}

pub fn add_thrustee(
    mut pl: std::cell::RefMut<Player>,
    lobby: &mut HashMap<i32, Lobby>,
    new_item: String,
) {
    pl.personal_deck.add_thrustee(&new_item);
    pl.send_message(&format!("Added \"{}\" to THRUSTEES!", &new_item));

    if let Some(lob) = lobby.get_mut(&pl.lobby) {
        if lob.state == LobbyState::Waiting {
            Lobby::add_pers_deck_to_lob(lob, &mut pl);
        }
    }
}

pub fn display_deck(pl: std::cell::RefMut<Player>) {
    let mut messages = Vec::new();

    messages.push("You're THRUSTEES:".to_string());
    for (i, thrustee) in pl.personal_deck.thrustees.iter().enumerate() {
        messages.push(format!("{}. {}", i + 1, &thrustee).clone());
    }

    messages.push("</br>You're THRUSTERS:".to_string());
    for (i, thruster) in pl.personal_deck.thrusters.iter().enumerate() {
        messages.push(format!("{}. {}", i + 1, &thruster).clone());
    }

    pl.send_messages(&messages);
}

pub fn clear_pers_deck(pl_rc: Rc<RefCell<Player>>, lobby: &mut HashMap<i32, Lobby>) {
    let mut pl = pl_rc.borrow_mut();

    if let Some(lob) = lobby.get_mut(&pl.lobby) {
        if lob.state == LobbyState::Waiting {
            Lobby::remove_pers_deck_from_lob(lob, &mut pl);
        }
    }

    pl.personal_deck = thrust::Deck::new();

    pl.send_message("Personal THRUSTS have been cleared! If this was an accident, Good Luck!");
}
