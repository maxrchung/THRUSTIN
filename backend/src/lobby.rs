use crate::player::{Player, PlayerState};
use crate::thrust;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
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
    pub list: std::vec::Vec<Rc<RefCell<Player>>>,

    //max number of players
    pub max: usize,

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

    pub current_thrusts: HashMap<Token, String>,
    //maps thrust index to token (end me)
    pub index_to_token: HashMap<i32, Token>,

    pub thrusted_players: Vec<Token>,

    pub thrustee_choices: Vec<String>,

    pub max_thrustee_choices: i32,

    pub use_house: bool,
}

impl Lobby {
    fn new(
        player: &Rc<RefCell<Player>>,
        pw: String,
        max: usize,
        id: i32,
        pers_deck: &mut thrust::Deck,
    ) -> Lobby {
        let mut lobby = Lobby {
            pw: pw,
            list: std::vec::Vec::with_capacity(max as usize),
            max: max,
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

        lobby
            .deck
            .thrustees
            .append(&mut pers_deck.thrustees.clone()); //addin "personal decks" to lobby(default) deck. .
        lobby
            .deck
            .thrusters
            .append(&mut pers_deck.thrusters.clone());
        lobby.deck.sort();
        lobby.deck.thrusters.shuffle(&mut thread_rng());
        lobby.deck.thrustees.shuffle(&mut thread_rng());
        lobby
    }

    fn new_endless(
        player: &Rc<RefCell<Player>>,
        max: usize
    ) -> Lobby {
        let mut lobby = Lobby {
            pw: "".to_string(),
            list: std::vec::Vec::with_capacity(max as usize),
            max: max,
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
        lobby.deck.sort();
        lobby.deck.thrusters.shuffle(&mut thread_rng());
        lobby.deck.thrustees.shuffle(&mut thread_rng());
        lobby
    }

    ///////////
    //private//
    ///////////
    fn is_host(&self, player: &Token) -> bool {
        (self.host.borrow().token == *player) && (self.host.borrow().name != "EndlessLobbyHostDoggo".to_string())
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

    fn send_message(&self, message: &str) {
        for pl in &self.list {
            pl.borrow().send(&message);
        }
    }

    //////////
    //public//
    //////////

    //////////////////
    //general stuff?//
    //////////////////

    pub fn make_endless_lobby(
        pl_rc: &Rc<RefCell<Player>>,
        lobby_id: &mut i32,
        lobbies: &mut HashMap<i32, Lobby>,
    ) {
        let max = 64;

        let mut new_lobby = Lobby::new_endless(
            pl_rc,
            max,
        );

        lobbies.insert(lobby_id.clone(), new_lobby.clone());
    }

    pub fn make_lobby(
        input: std::vec::Vec<&str>,
        pl_rc: Rc<RefCell<Player>>,
        lobby_id: &mut i32,
        lobbies: &mut HashMap<i32, Lobby>,
    ) {
        let mut pl = pl_rc.borrow_mut();
        let max = 64;

        pl.lobby = lobby_id.clone();
        pl.state = PlayerState::InLobby;

        let mut new_lobby = Lobby::new(
            &pl_rc,
            "".to_string(),
            max,
            *lobby_id,
            &mut pl.personal_deck,
        );
        new_lobby.list.push(pl_rc.clone());

        lobbies.insert(lobby_id.clone(), new_lobby.clone());
        pl.send(&format!("Created lobby: {}", lobby_id));

        *lobby_id = *lobby_id + 1;
    }

    pub fn set_password(&mut self, input: std::vec::Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
        let pl = pl_rc.borrow();
        if !self.is_host(&pl.token) {
            pl.send("only host sets password!!!");
            return;
        }

        if input.len() < 2 {
            pl.send("?? what's the pass boss??");
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

            let message = if self.is_host(&pl_i.token) {
                format!("{}: host{}", name, person).to_string()
            } else {
                format!("{}{}", name, person).to_string()
            };

            messages.push(message);
        }

        if messages.is_empty() {
            messages.push(String::from("There's no players lmfao"));
        }

        pl.send_multiple(messages);
    }

    pub fn info(&self, pl_rc: Rc<RefCell<Player>>) {
        let pl = pl_rc.borrow();
        let mut info = Vec::new();
        info.push(format!("\\\\Lobby info//"));
        info.push(format!("Name: {}", self.id));
        info.push(format!("Players: {} / {}", self.list.len(), self.max));
        info.push(format!("Max points: {}", self.max_points));

        if self.is_host(&pl.token) {
            info.push(format!("Pw: {}", self.pw));
        }

        pl.send_multiple(info);
    }

    pub fn point_max(&mut self, input: std::vec::Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
        let pl = pl_rc.borrow();
        if !self.is_host(&pl.token) {
            pl.send("only host sets points!");
            return;
        }

        if input.len() < 2 {
            pl.send("ya gotta set the new limit");
            return;
        }

        match input[1].to_string().parse::<u32>() {
            Ok(max) => {
                if max == 0 {
                    pl.send("bro dont make it 0 wtf man");
                    return;
                }
                self.max_points = max;
                pl.send(&format!("max points set to {}", self.max_points));
            }

            _ => pl.send(&"only numbers dude!!!"),
        }
    }

    pub fn player_max(&mut self, input: std::vec::Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
        let pl = pl_rc.borrow();
        if !self.is_host(&pl.token) {
            pl.send("only host sets MAXP LAYER!");
            return;
        }

        if input.len() < 2 {
            pl.send("ya gotta set the new limit");
            return;
        }

        match input[1].to_string().parse::<usize>() {
            Ok(max) => {
                if max > 64 {
                    pl.send(&format!("woah thats 2many people chill! haha"));
                    return;
                }

                if max < self.list.len() {
                    pl.send(&format!("too many players in here right now man!"));
                    return;
                }
                self.max = max;
                pl.send(&format!("max players set to {}", self.max));
            }

            _ => pl.send(&"only numbers dude!!!"),
        }
    }

    pub fn switch_host(&mut self, input: std::vec::Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
        let pl = pl_rc.borrow();
        if !self.is_host(&pl.token) {
            pl.send("Only host can change the host!");
            return;
        }

        if input.len() < 2 {
            pl.send("Who's the new host tho");
            return;
        }

        let new_host = input[1];
        if self.host.borrow().name == new_host {
            pl.send("You're already host!!");
            return;
        }

        for players_ in self.list.iter() {
            let players = players_.borrow();
            if players.name == new_host {
                self.host = players_.clone();
                players.send("You are now host!");
                pl.send(&format!("{} is now host!", players.name));
                return;
            }
        }

        pl.send("Player not in lobby.");
    }

    pub fn kick(&mut self, input: std::vec::Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
        let pl = pl_rc.borrow();
        if !self.is_host(&pl.token) {
            pl.send("Only host can kick em!");
            return;
        }

        if input.len() < 2 {
            pl.send("who we kickkin");
            return;
        }

        let kick = input[1];
        if self.host.borrow().name == kick {
            pl.send("u cant kick ursel!!");
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
            {
                let mut player = self.list[kick_ind as usize].borrow_mut();
                player.state = PlayerState::OutOfLobby;
                player.lobby = -1;
                player.send("ur r kicked!!");
                pl.send(&format!("u rly kicedk {} out!", player.name));
            }

            self.list.remove(kick_ind as usize);

            return;
        }

        pl.send("Player not in lobby.");
    }

    pub fn join_lobby(
        input: std::vec::Vec<&str>,
        pl_rc: Rc<RefCell<Player>>,
        lobby: &mut HashMap<i32, Lobby>,
    ) {
        let mut pl = pl_rc.borrow_mut();
        if input.len() < 2 {
            pl.send("Lobby name required!");
            return;
        }

        match input[1].to_string().parse::<i32>() {
            Ok(lobby_id) => {
                let mut messages = Vec::new();
                if let Some(lob) = lobby.get_mut(&lobby_id) {
                    // Lobby full check
                    if lob.list.len() >= lob.max {
                        pl.send("bro this lobbBY is FULLLLL!!");
                        return;
                    }

                    //Lobby Password Check
                    if lob.pw != "" {
                        if input.len() < 3 {
                            pl.send("Ya need a password BR)");
                            return;
                        } else if lob.pw != input[2] {
                            pl.send("loll wrong pw haha");
                            return;
                        }
                    }

                    messages.push(format!("Joined: {:#?}", &lobby_id));

                    // Set points to 0 (just in case?)
                    pl.points = 0;

                    // add players' personal deck (.thrustee/.thruster) to lobby deck
                    lob.deck
                        .thrustees
                        .append(&mut pl.personal_deck.thrustees.clone());
                    lob.deck
                        .thrusters
                        .append(&mut pl.personal_deck.thrusters.clone());

                    pl.state = if lob.state == LobbyState::Playing {
                        // Distribute thrusters to player
                        for _ in 0..lob.hand_size {
                            if let Some(card) = lob.deck.thrusters.pop() {
                                pl.deck.thrusters.push(card.clone());
                            } else {
                                lob.restart_game();
                                pl.send("Not enough thrusters to distribute");
                                return;
                            }
                        }

                        let thrustee = lob.list[lob.thrustee].borrow();
                        let mut wait: bool = false;
                        // Handle cases where thrustee is currently choosing/deciding differently
                        match thrustee.state {
                            PlayerState::Playing => {
                                messages.push(
                                    format!("This is your THRUSTEE: {}", &lob.current_thrustee)
                                        .to_string(),
                                );
                                messages.extend(get_thrusters(&pl.deck.thrusters));
                            }

                            // NOTE: Player is currently able to thrust into PREVIOUS thrustee gotta FIXER it later
                            PlayerState::Choosing => {
                                wait = true;
                                messages.push(
                                    "Thrustee is currently CHOOSING next thrustee. Hold on tight!"
                                        .to_string(),
                                );
                            }

                            PlayerState::Deciding => {
                                messages.push(
                                    format!("This is your THRUSTEE: {}", &lob.current_thrustee)
                                        .to_string(),
                                );
                                messages.extend(get_thrusters(&pl.deck.thrusters));
                            }

                            _ => (),
                        }

                        if wait {
                            PlayerState::Waiting
                        } else {
                            PlayerState::Playing
                        }
                    } else {
                        PlayerState::InLobby
                    };

                    lob.send_message(&format!("{} has joined the lobby.", pl.name));
                    // adding the new player to lobby
                    pl.lobby = lob.id;
                    lob.list.push(pl_rc.clone());
                    pl.send_multiple(messages);
                } else {
                    pl.send("Lobby does not exist.");
                }
            }

            _ => pl.send("nibba that is a invalid input my nibba"),
        }
    }

    pub fn leave_lobby(&mut self, pl_rc: Rc<RefCell<Player>>) -> bool {
        let (lob_id, name) = {
            let pl = pl_rc.borrow();

            let pl_ind = self.search_token(&pl.token);

            self.list.remove(pl_ind);

            (pl.lobby, pl.name.clone())
        };

        let out: bool = self.list.len() == 0;
        if !out {
            if Rc::into_raw(pl_rc.clone()) == Rc::into_raw(self.host.clone()) {
                self.host = self.list[0].clone();
                &self.host.borrow().send("u host now!!");
            }
        }

        let mut pl = pl_rc.borrow_mut();

        pl.send(&format!("Left lobby: {}.", lob_id));
        self.send_message(&format!("{} has left the lobby.", name));

        pl.lobby = -1;
        pl.state = PlayerState::OutOfLobby;

        out
    }

    pub fn toggle_house(&mut self, pl_rc: Rc<RefCell<Player>>) {
        let pl = pl_rc.borrow();
        self.use_house = !self.use_house;
        if self.use_house {
            pl.send(&"Now using house cards!");
        } else {
            pl.send(&"No longer using house cards!...");
        }
    }

    /////////////////
    //game commands//
    /////////////////

    pub fn start_game(&mut self, pl_rc: Rc<RefCell<Player>>) {
        {
            let pl = pl_rc.borrow();

            if !self.is_host(&pl.token) {
                pl.send(&format!("Only host can start game!"));
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
                        .send(&"Chief, there ain't enough cards to start");
                    return;
                }
            }

            if i == self.thrustee {
                pl.state = PlayerState::Choosing;
                let mut messages =
                    vec!["You are the THRUSTEE. CHOOSE NOW..........<br/>".to_string()];
                messages.extend(self.print_thrustee_choices());
                pl.send_multiple(messages);
            } else {
                pl.send("You are a THRUSTER. waiting for a good THRUSTEE; mmm baby!");
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
        self.deck.sort();
        self.deck.thrusters.shuffle(&mut thread_rng());
        self.deck.thrustees.shuffle(&mut thread_rng());
    }

    pub fn handle_winner(&mut self, winner_dex: usize) {
        self.clear_game();
        let winner_name = self.list[winner_dex].borrow_mut().name.clone();
        self.send_message(&format!("Congratulations, {}! You're Winner! Everyone else, You're Loser! Game has been put into waiting state, Thrustin'ers!", winner_name));
    }

    pub fn restart_game(&mut self) {
        self.clear_game();
        self.send_message(&"Chief called and he said we're outta cards. Game has restarted and put into waiting state.");
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
                pl.send("ya need to pick a NUMERIC, Boy");
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

                        // Put remaining choices back into thrustees deck
                        for choice in self.thrustee_choices.iter() {
                            self.deck.thrustees.push(choice.clone());
                        }
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
                                "get Ready to DECIDE best THRUSTER for THRUSTING!".to_string(),
                            );
                            p.send_multiple(messages);
                        } else {
                            messages.extend(get_thrusters(&p.deck.thrusters));
                            p.send_multiple(messages);
                        }
                    }
                } else {
                    pl_rc.borrow().send("That shit's out of bound bro");
                }
            }
            _ => {
                pl_rc.borrow().send(
                    "That is an invalid parameter my chieftain, use an index instead dawggo.",
                );
            }
        };
    }

    pub fn decide(&mut self, input: std::vec::Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
        {
            let pl = pl_rc.borrow();
            if input.len() < 2 {
                pl.send("ya need to pick a numbert boi");
                return;
            }
        }

        match input[1].parse::<i32>() {
            Ok(index) => {
                if index < self.current_thrusts.len() as i32 && index >= 0 {
                    // Because of multiple mutable references
                    let mut restart = false;
                    let mut name = String::new();
                    let mut chosen_thrust = String::new();
                    {
                        let mut pl = pl_rc.borrow_mut();
                        name = pl.name.clone();

                        // Get chosen thrust
                        chosen_thrust = self
                            .current_thrusts
                            .remove(&self.index_to_token.get(&index).unwrap())
                            .unwrap();

                        // Clear thrust values
                        self.current_thrusts.clear();
                        self.thrusted_players.clear();

                        // Set current THRUSTEE to THRUSTER state
                        pl.state = PlayerState::Waiting;

                        // Get new thrustee_choices for next THRUSTEE
                        for _ in 0..self.max_thrustee_choices {
                            if let Some(card) = self.deck.thrustees.pop() {
                                self.thrustee_choices.push(card);
                            } else {
                                restart = true;
                            }
                        }
                    }

                    {
                        // wew lad
                        // Assign picked thruster a point
                        let tkn = self.search_token(self.index_to_token.get(&index).unwrap());

                        let pts: u32 = {
                            let mut chosen_thruster = self.list[tkn].borrow_mut();
                            chosen_thruster.points += 1;
                            chosen_thruster.points.clone()
                        };

                        // Check if winner
                        if pts >= self.max_points {
                            self.handle_winner(tkn);
                            return;
                        }
                    }

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
                            messages.push("get rdy to thrust.....".to_string());
                        }
                        pl.borrow().send_multiple(messages);
                        //communication.send_messages(&pl.borrow().token, messages);
                    }
                } else {
                    pl_rc.borrow().send("That shit's out of bound bro");
                }
            }
            _ => {
                pl_rc
                    .borrow()
                    .send("That is an invalid parameter, use an index instead");
            }
        };
    }

    pub fn handle_thrust(&mut self, input: std::vec::Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
        {
            let mut pl = pl_rc.borrow();
            // Check number of inputs
            if input.len() < 2 {
                pl.send(&"Index required!");
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
                        pl.send("bro that ain't the right number of thrusters");
                        return;
                    }
                    let mut indexes: Vec<i32> = Vec::new();
                    // Check thrust out of bounds
                    for i in 1..input.len() {
                        let dex = input[i].parse::<i32>().unwrap();
                        if indexes.contains(&dex) {
                            // Check if dupes
                            pl.send("y'ain't allowed to thrust duplicate thrusters broski");
                            return;
                        }
                        indexes.push(dex);
                        if dex >= pl.deck.thrusters.len() as i32 || index < 0 {
                            pl.send("That shit's out of bound bro");
                            return;
                        }
                    }

                    // Check if thrusted
                    for player_token in &self.thrusted_players {
                        if pl.token == *player_token {
                            pl.send("You have already THRUSTED, you cannot THRUST again.");
                            return;
                        }
                    }

                    resulting_thrust = self.current_thrustee.clone();
                    let mut to_remove: std::vec::Vec<String> = Vec::new();
                    // Handle mutliple underscores
                    for i in 1..input.len() {
                        let picked_thruster = pl.deck.thrusters[input[i].parse::<usize>().unwrap()].clone();
                        to_remove.push(picked_thruster.clone());
                        // Surround with <u> to underline text
                        let formatted_thruster = format!(
                            "<u>{}</u>",
                            picked_thruster
                        );
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
                    .send("That is an invalid parameter, use an index instead");
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

        pl.send_multiple(messages);
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

    pl.send_multiple(messages);
}

pub fn list_all_players(
    pl_rc: Rc<RefCell<Player>>,
    players: &mut HashMap<Token, Rc<RefCell<Player>>>,
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
    pl.send_multiple(messages);
}

pub fn add_item(
    input: &std::vec::Vec<&str>,
    pl_rc: Rc<RefCell<Player>>,
    lobby: &mut HashMap<i32, Lobby>,
    thruster: bool,
) -> bool {
    let mut pl = pl_rc.borrow_mut();

    if input.len() < 2 {
        pl.send("Thruster/thrustee required!");
        return true;
    }

    let mut new_item = String::new();
    for i in 1..input.len() {
        new_item.push_str(input[i as usize]);
        new_item.push_str(" ");
    }
    new_item.pop();

    if new_item.chars().next().unwrap() != "\"".to_string().chars().last().unwrap()
        || new_item.chars().last().unwrap() != "\"".to_string().chars().last().unwrap()
    {
        pl.send("Please surround the thruster/thrustee with quotes.");
        return true;
    }
    new_item.pop();
    new_item.remove(0);

    if !thruster && !new_item.contains("_") {
        return false;
    }

    if thruster {
        pl.personal_deck.add_thruster(&new_item);
        pl.send(&format!("Added \"{}\" to thrusters!", &new_item));
    } else {
        pl.personal_deck.add_thrustee(&new_item);
        pl.send(&format!("Added \"{}\" to thrustees!", &new_item));
    }

    if let Some(lob) = lobby.get_mut(&pl.lobby) {
        if lob.state == LobbyState::Waiting {
            lob.deck
                .thrustees
                .append(&mut pl.personal_deck.thrustees.clone());
            lob.deck
                .thrusters
                .append(&mut pl.personal_deck.thrusters.clone());
        }
    }

    true
}
