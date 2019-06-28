use crate::player::{Player, PlayerState};
use crate::thrust::Deck;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::u8;
use std::usize;

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
    list: Vec<Rc<RefCell<Player>>>,
    //max number of players
    max: usize,
    //max hand size
    hand_size: u8,
    //points
    max_points: u8,
    //lobby id
    id: i32,
    //lobby state
    state: LobbyState,
    //host of lobby
    //pub host: usize,
    host: Rc<RefCell<Player>>,
    //current thrustee (player)
    thrustee: usize,
    //Deck that lobby will be actively playing with
    deck: Deck,
    //Deck that lobby will keep as reference when needing to refill thrusters/thrustees
    deck_reference: Deck,
    //current thrustee (card)
    current_thrustee: String,
    current_thrusts: HashMap<u32, String>,
    //maps thrust index to token (end me)
    index_to_token: HashMap<i32, u32>,
    thrusted_players: Vec<u32>,
    thrustee_choices: Vec<String>,
    max_thrustee_choices: u8,
    use_house: bool,
}

impl Lobby {
    fn new(player: &Rc<RefCell<Player>>, pw: String, max: usize, id: i32) -> Lobby {
        let lobby = Lobby {
            pw: pw,
            list: Vec::with_capacity(max as usize),
            max: max,
            id: id,
            state: LobbyState::Waiting,
            hand_size: 5,
            max_points: 7,
            host: player.clone(),
            thrustee: 0,
            thrustee_choices: Vec::new(),
            max_thrustee_choices: 3,
            deck: Deck::new(),
            deck_reference: Deck::new(),
            current_thrustee: String::new(),
            current_thrusts: HashMap::new(),
            index_to_token: HashMap::new(),
            thrusted_players: Vec::new(),
            use_house: true,
        };
        lobby
    }

    fn new_endless(player: &Rc<RefCell<Player>>) -> Lobby {
        let lobby = Lobby {
            pw: "".to_string(),
            list: Vec::new(),
            max: usize::MAX,
            id: 0,
            state: LobbyState::Waiting,
            hand_size: 5,
            max_points: u8::MAX,
            host: player.clone(),
            thrustee: 0,
            thrustee_choices: Vec::new(),
            max_thrustee_choices: 3,
            deck: Deck::new(),
            deck_reference: Deck::new(),
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

    fn refill_thrusters(&mut self, pl: &mut Player) {
        // Clear dude's deck beforehand
        pl.deck.thrusters.clear();
        // Distribute thrusters to player
        for _ in 0..self.hand_size {
            if let Some(card) = self.deck.thrusters.pop() {
                pl.deck.thrusters.push(card);
            } else {
                // Refill thrusters if empty
                self.deck.thrusters = self.deck_reference.thrusters.clone();
                self.deck.shuffle_thrusters();
                pl.deck.thrusters.push(self.deck.thrusters.pop().unwrap());
            }
        }
    }

    fn refill_thrustees(&mut self) {
        // Setup new thrustee choices
        for _ in 0..self.max_thrustee_choices {
            if let Some(card) = self.deck.thrustees.pop() {
                self.thrustee_choices.push(card);
            } else {
                self.deck.thrustees = self.deck_reference.thrustees.clone();
                self.deck.shuffle_thrustees();
                self.thrustee_choices.push(self.deck.thrustees.pop().unwrap());
            }
        }
    }

    //////////
    //public//
    //////////

    //////////////////
    //general stuff?//
    //////////////////
    pub fn is_host(&self, player: u32) -> bool {
        (self.host.borrow().token == player)
            && (self.host.borrow().name != "EndlessLobbyChiefDoggo".to_string())
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
        pl_rc: Rc<RefCell<Player>>,
        lobby_id: &mut i32,
        lobbies: &mut HashMap<i32, Lobby>,
    ) {
        let mut new_lobby = Lobby::new(&pl_rc, "".to_string(), 10, *lobby_id);

        let mut pl = pl_rc.borrow_mut();

        pl.lobby = lobby_id.clone();
        pl.state = PlayerState::InLobby;

        new_lobby.list.push(pl_rc.clone());

        lobbies.insert(lobby_id.clone(), new_lobby.clone());
        pl.send_message(&format!("Created lobby: {}", lobby_id));

        *lobby_id = *lobby_id + 1;
    }

    pub fn set_password(&mut self, input: Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
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

        messages.sort_unstable_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        pl.send_messages(&messages);
    }

    pub fn info(&self, pl_rc: Rc<RefCell<Player>>) {
        let pl = pl_rc.borrow();
        let mut info = Vec::new();
        info.push(format!("\\\\Lobby info//"));
        info.push(format!("Name: {}", self.id));
        info.push(format!("Players: {} / {}", self.list.len(), self.max));
        info.push(format!("Max points: {}", self.max_points));

        if self.is_host(pl.token) {
            info.push(format!("Pw: {}", self.pw));
        }

        pl.send_messages(&info);
    }

    pub fn point_max(&mut self, input: Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
        let pl = pl_rc.borrow();
        if !self.is_host(pl.token) {
            pl.send_message("only chief sets points!");
            return;
        }

        if input.len() < 2 {
            pl.send_message("ya gotta set the new limit");
            return;
        }

        match input[1].to_string().parse::<u8>() {
            Ok(max) => {
                if max == 0 {
                    pl.send_message("bro dont make it 0 wtf man");
                    return;
                }
                self.max_points = max;
                pl.send_message(&format!("max points set to {}", self.max_points));
            }

            _ => pl.send_message(&"You have provided an invalid parameter."),
        }
    }

    pub fn player_max(&mut self, input: Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
        let pl = pl_rc.borrow();
        if !self.is_host(pl.token) {
            pl.send_message("only chief sets MAXP LAYER!");
            return;
        }

        if input.len() < 2 {
            pl.send_message("ya gotta set the new limit");
            return;
        }

        match input[1].to_string().parse::<usize>() {
            Ok(max) => {
                if max > 64 {
                    pl.send_message(&format!("woah thats 2many people chill! haha"));
                    return;
                }

                if max < self.list.len() {
                    pl.send_message(&format!("too many players in here right now man!"));
                    return;
                }
                self.max = max;
                pl.send_message(&format!("max players set to {}", self.max));
            }

            _ => pl.send_message(&"only numbers dude!!!"),
        }
    }

    pub fn max_thrustee(&mut self, input: Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
        let pl = pl_rc.borrow();
        if !self.is_host(pl.token) {
            pl.send_message(
                "Only chief of the lobby is the only one who may set the THRUSTEE count.",
            );
            return;
        }

        if input.len() < 2 {
            pl.send_message(
                "A value must be provided to determine what the THRUSTEE count is to be.",
            );
            return;
        }

        match input[1].to_string().parse::<u8>() {
            Ok(max) => {
                if max < 2 {
                    pl.send_message(&format!("Brother, you must specify 2 or more for THRUSTEE count. This is so that we can guarantee some sort of picking decision for the THRUSTEE to partake in when selecting a desired THRUSTEE to use. Thank you for your understanding."));
                    return;
                }
                self.max_thrustee_choices = max;
                pl.send_message(&format!("max THRUSTEE set to {}", self.max));
            }

            _ => pl.send_message(&"Thou hast entered a value that we have deemed as unparsable for the requested THRUSTEE command that thou hast witch hath entered hitherto."),
        }
    }

    pub fn max_thruster(&mut self, input: Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
        let pl = pl_rc.borrow();
        if !self.is_host(pl.token) {
            pl.send_message("ONLY CHIEF CAN SET MAX THRUSTERS!!!");
            return;
        }

        if input.len() < 2 {
            pl.send_message("You need to give me a value man");
            return;
        }

        match input[1].to_string().parse::<u8>() {
            Ok(max) => {
                if max < 2 {
                    pl.send_message(&format!("bro u need at least two THRUSTERS, try again?"));
                    return;
                }

                self.hand_size = max;
                pl.send_message(&format!("max THRUSTERS set to {}", self.max));
            }

            _ => {
                pl.send_message(&"only positive numbers dude, please!!! (and not too big neither)")
            }
        }
    }

    pub fn switch_host(&mut self, input: Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
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

    pub fn kick(&mut self, input: Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
        // Scope guards to avoid borrow panic when THRUSTEE is kicked
        let kick_ind = {
            let mut kick_ind = -1;
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

            for (i, players) in self.list.iter().enumerate() {
                let players = players.borrow();
                if players.name == kick {
                    kick_ind = i as i32;
                    break;
                }
            }
            kick_ind
        };

        if kick_ind >= 0 {
            self.leave_lobby(self.list[kick_ind as usize].clone());
            return;
        }

        let pl = pl_rc.borrow();
        pl.send_message("Player not in lobby.");
    }

    pub fn handle_join_cases(
        pl: &RefMut<Player>,
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
                messages.push(
                    format!("This is your THRUSTEE: {}<br/>", &lob.current_thrustee).to_string(),
                );
                messages.extend(Lobby::get_thrusters(&pl.deck.thrusters));
            }

            _ => (),
        }
    }

    pub fn get_joining_pl_state(
        lob: &mut Lobby,
        pl: &mut RefMut<Player>,
        messages: &mut Vec<String>,
        pl_rc: &Rc<RefCell<Player>>,
    ) -> Option<PlayerState> {
        // Clear dude's deck beforehand
        pl.deck.thrusters.clear();
        // Distribute thrusters to player
        lob.refill_thrusters(pl);

        let mut wait: bool = false;

        // If lobby was empty before this guy joined, then they become THRUSTEE, otherwise, basically do what normal join_lobby does (yea this is fucked fk me doggo)
        if lob.list.len() == 0 {
            pl.state = PlayerState::Choosing;
            let mut messages =
                vec!["Welcome to the 『Endless Lobby』, big doggo. You lucky, family, you are THRUSTEE!!!!.. . Choose now...    .".to_string()];
            messages.extend(lob.print_thrustee_choices());
            pl.send_messages(&messages);
            pl.lobby = lob.id;
            lob.list.push(pl_rc.clone());
            lob.thrustee = 0;
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

    pub fn join_lobby(
        input: Vec<&str>,
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
                let mut pl = pl_rc.borrow_mut();
                let mut messages = Vec::new();
                if let Some(mut lob) = lobby.get_mut(&lobby_id) {
                    // Lobby full check
                    if lob.list.len() >= lob.max {
                        pl.send_message("bro this lobbBY is FULLLLL!!");
                        return;
                    }

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

                    pl.state = if lob.state == LobbyState::Playing {
                        lob.deck.shuffle_deck();

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

    // This feeds into leave_lobby()
    // This function is separate from leave_lobby() because this also manages removing the lobby if it is empty
    pub fn leave_from_lobby(pl: Rc<RefCell<Player>>, lobbies: &mut HashMap<i32, Lobby>) {
        let lobby = lobbies.get_mut(&pl.borrow().lobby).unwrap();
        lobby.leave_lobby(pl);
        // Don't delete lobby if it is endless
        if lobby.list.len() == 0 && lobby.id != 0 {
            let id = lobby.id;
            lobbies.remove(&id);
        }
    }

    fn leave_lobby(&mut self, pl_rc: Rc<RefCell<Player>>) {
        // borrow shenanigans
        {
            let pl = pl_rc.borrow();

            // Much easier to clear single person lobby than multiple
            if self.list.len() == 1 {
                self.list.clear();
            } else {
                let pl_ind = self.search_token(pl.token).unwrap();
                let next_ind = (pl_ind + 1) % self.list.len();
                // This needs to be calculated as if pl has been removed
                let next_thrustee = pl_ind % (self.list.len() - 1);

                // Set up messages
                let mut messages = vec![String::from(format!("{} left the lobby..", pl.name))];
                // Handle if player is host
                if pl.token == self.host.borrow().token {
                    self.host = self.list[next_ind].clone();
                    messages.push(String::from(format!(
                        "Chief left so now we got a new one --> {}",
                        self.host.borrow().name
                    )));
                }

                let mut did_thrustee_change = false;
                // Handle if player is Choosing
                if pl.state == PlayerState::Choosing {
                    self.thrustee = next_thrustee;
                    did_thrustee_change = true;
                    // Next player chooses and replenish cards
                    let mut next = self.list[next_ind].borrow_mut();
                    next.state = PlayerState::Choosing;
                    messages.push(String::from(format!(
                        "Lol yo bro 'cause the THRUSTEE left {} is choosin' the next THRUSTEE now!",
                        next.name
                    )));
                }
                // Handle if player is Deciding
                else if pl.state == PlayerState::Deciding {
                    self.thrustee = next_thrustee;
                    did_thrustee_change = true;
                    // Next player decides
                    let mut next = self.list[next_ind].borrow_mut();
                    next.state = PlayerState::Deciding;
                    messages.push(String::from(format!("Lmao the THRUSTEE left and you're next in line, so {} gets to decide which THRUST wins lmfao.", next.name)));
                }
                // Handle if player is Normal
                else {
                    if pl_ind < self.thrustee {
                        self.thrustee = self.thrustee - 1;
                    }
                }

                // Remove player after we are done with managing player state
                self.list.remove(pl_ind);

                // Send appropriate message
                for pl_rc in &self.list {
                    let pl = pl_rc.borrow();
                    if did_thrustee_change && pl.state == PlayerState::Choosing {
                        let mut thrustee_messages = messages.clone();
                        // Add a linebreak for better visibility
                        thrustee_messages.last_mut().unwrap().push_str("<br/>");
                        // Notify new Choosing THRUSTEE of choices
                        thrustee_messages.extend(self.print_thrustee_choices());
                        pl.send_messages(&thrustee_messages);
                    } else {
                        pl.send_messages(&messages);
                    }
                }
            }
        }

        // Player mut and specific operations
        let mut pl_mut = pl_rc.borrow_mut();
        pl_mut.lobby = -1;
        pl_mut.state = PlayerState::OutOfLobby;
        pl_mut.send_message("You have been leaved from the lobby okay!");
    }

    pub fn toggle_house(&mut self, pl_rc: Rc<RefCell<Player>>) {
        let pl = pl_rc.borrow();
        if !self.is_host(pl.token) {
            pl.send_message(&format!("Only chief can decide whether or not toggle the house (default provided) THRUSTS for THRUSTING!"));
            return;
        }

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
        self.deck = Deck::default();
        self.deck_reference = self.deck.clone();
        self.deck.shuffle_deck();
        self.refill_thrustees();
    }

    pub fn start_game(&mut self, pl_rc: Rc<RefCell<Player>>) {
        {
            let pl = pl_rc.borrow();

            if !self.is_host(pl.token) {
                pl.send_message(&format!("Only chief can start game!"));
                return;
            }
        }

        self.deck.clear();
        // Add in house cards to lobby deck if bool is true
        if self.use_house {
            let default_deck = Deck::default();
            self.deck = default_deck;
        }

        // Add each person's deck in
        {
            let decks: Vec<Deck> = self
                .list
                .iter()
                .map(|pl| pl.borrow().personal_deck.clone())
                .collect();
            for deck in decks {
                self.deck.thrustees.append(&mut deck.thrustees.clone());
                self.deck.thrusters.append(&mut deck.thrusters.clone());
            }
        }

        // Validate THRUSTEES
        if self.deck.thrustees.len() < self.max_thrustee_choices as usize {
            let msg = format!("Dude, I can't start the game for you because yall don't got enough THRUSTEES. Here's a lil bit of mathematics:<br/>\
            {} (Total THRUSTEES) < {} (THRUSTEE Choices)", self.deck.thrustees.len(), self.max_thrustee_choices);
            pl_rc.borrow().send_message(&msg);
            return;
        }

        // Validate THRUSTERS
        if self.deck.thrusters.len() < self.hand_size as usize * self.list.len() {
            let msg = format!("Yo... got an issue boss, we don't go enough THRUSTERS. Let me calculate to tell you why:<br/>\
            {} (Total THRUSTERS) < {} (THRUSTER Choices) * {} (Number Of People In Lobby)", self.deck.thrusters.len(), self.hand_size, self.list.len());
            pl_rc.borrow().send_message(&msg);
            return;
        }

        // Validate underscores
        let underscores = self.deck.count_max_underscores();
        if underscores > self.max_thrustee_choices as i32 {
            let msg = format!("Hello, I am unable to start the game. This is because there is a THRUSTEE that requires too many THRUSTERS. Allow me to explain through geometry:<br/>\
            {} (THRUSTER Choices) < {} (THRUSTERS For A THRUSTEE)", self.hand_size, underscores);
            pl_rc.borrow().send_message(&msg);
            return;
        }

        self.deck_reference = self.deck.clone();
        self.deck.shuffle_deck();
        self.state = LobbyState::Playing;
        self.refill_thrustees();

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
        self.deck = Deck::default();
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
            player.deck = Deck::new();
            player.state = PlayerState::InLobby;
        }
        self.deck.shuffle_deck();
    }

    pub fn clear_endless(&mut self) {
        self.deck = Deck::default();

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
        self.deck.shuffle_deck();

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

    pub fn print_thrustee_choices(&self) -> Vec<String> {
        let mut messages = vec!["your THRUSTEE Choices:".to_string()];
        for (index, thrustee) in self.thrustee_choices.iter().enumerate() {
            messages.push(format!("{}. {}", &index, &thrustee).to_string());
        }
        messages
    }

    pub fn choose(&mut self, input: Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
        {
            let pl = pl_rc.borrow();

            if input.len() < 2 {
                pl.send_message("ya need to pick a NUMERIC, Boy");
                return;
            }
        }

        match input[1].parse::<u8>() {
            Ok(index) => {
                if index < self.max_thrustee_choices {
                    // Scope refcell borrow
                    let name;
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
                            messages.extend(Lobby::get_thrusters(&p.deck.thrusters));
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

    pub fn decide(&mut self, input: Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
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
                    let (name, chosen_thrust) = {
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

                        self.refill_thrustees();
                        (name, chosen_thrust)
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
                            "The winning THRUSTER, {} now has {}/{} point(s)! Watch out!<br/>",
                            &chosen_thruster_name, &chosen_thruster_pts, self.max_points
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

    pub fn handle_thrust(&mut self, input: Vec<&str>, pl_rc: Rc<RefCell<Player>>) {
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
                let resulting_thrust = {
                    let mut pl = pl_rc.borrow_mut();

                    // Check correct # of thrusters
                    let num_thrusters = input.len() as i32 - 1;
                    let num_underscore = Deck::count_underscores(&self.current_thrustee);
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

                    let mut resulting_thrust = self.current_thrustee.clone();
                    let mut to_remove: Vec<String> = Vec::new();
                    // Handle mutliple underscores
                    for i in 1..input.len() {
                        let picked_thruster =
                            pl.deck.thrusters[input[i].parse::<usize>().unwrap()].clone();
                        to_remove.push(picked_thruster.clone());
                        // Surround with <u> to underline text
                        let formatted_thruster = format!("<u>{}</u>", picked_thruster);
                        resulting_thrust = Deck::thrust(&formatted_thruster, &resulting_thrust);
                    }

                    // Remove thrusted thrusters
                    let mut updated_thrusters: Vec<String> = Vec::new();
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

                    self.refill_thrusters(&mut pl);
                    resulting_thrust
                };

                // Notify message
                self.send_message(&format!(
                    "{}. {}",
                    &(self.current_thrusts.len() as i32 - 1),
                    &resulting_thrust
                ));
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
        messages.push(format!(
            "This is the Max points to strive for to win: {}",
            self.max_points
        ));

        for rc in &self.list {
            let player = rc.borrow();
            messages.push(format!("{}: {}", player.name, player.points));
        }

        pl.send_messages(&messages);
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
            if pl_i.token == 0 {
                continue;
            }

            let mut person = "";
            if pl_i.token == pl.token {
                person = " (You)";
            }

            let message =
                if pl_i.state == PlayerState::InLobby || pl_i.state == PlayerState::Playing {
                    format!("{} in {}{}", pl_i.name, pl_i.lobby, person).to_string()
                } else {
                    format!("{}{}", pl_i.name, person).to_string()
                };

            messages.push(message);
        }

        messages.sort_unstable_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        pl.send_messages(&messages);
    }
}
