use crate::communication::Communication;
use crate::database::MongoDB;
use crate::lobby::Lobby;
use crate::player_game::PlayerGame;
use crate::thrust::Deck;
use mongodb::Bson;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, PartialEq, Debug)]
pub enum PlayerState {
    ChooseName,
    OutOfLobby,
    InLobby,
    Playing,
    Choosing,
    Deciding,
    Waiting,
}

#[derive(Clone, Debug)]
pub struct Player {
    comm: Rc<RefCell<dyn Communication>>,
    db: Rc<RefCell<MongoDB>>,
    pub game: PlayerGame,
    // Whether or not user is logged in through DB
    pub is_authenticated: bool,
    pub lobby: i32,
    // name of player
    pub name: String,
    pub personal_deck: Deck,
    // player state
    pub state: PlayerState,
    pub token: u32,
}

impl Player {
    pub fn account(&self) {
        if !self.is_authenticated {
            self.send_message("You cannot do this. You must be fully authenticated and logged in in order to get your account info with a registered account.");
            return;
        }

        match self.db.borrow().find_name_doc(&self.name) {
            Some(doc) => {
                let mut messages = vec![String::from("A display of your account information and statistical information. Please enjoy THRUSTIN!")];
                if let Some(Bson::String(user)) = doc.get("user") {
                    messages.push(format!("Username - {}", user));
                }
                messages.push(format!("Name - {}", self.name));
                messages.push(String::from("Password - [ENCRYPTED_CONTENT__UNVIEWABLE]"));
                if let Some(points) = doc.get("points_gained") {
                    messages.push(format!("Pointed Earned So Far - {}", points));
                } else {
                    messages.push(String::from("Pointed Earned - 0"));
                }
                if let Some(games) = doc.get("games_played") {
                    messages.push(format!("Games Played So Far - {}", games));
                } else {
                    messages.push(String::from("Games Played So Far - 0"));
                }
                if let Some(games) = doc.get("games_won") {
                    messages.push(format!("Games Won So Far - {}", games));
                } else {
                    messages.push(String::from("Games Won So Far - 0"));
                }
                self.send_messages(&messages);
            }
            None => {
                self.send_message("Yo there's a bit of an epic problem. We couldn't find your account data lmao. What is going on.");
            }
        }
    }

    pub fn display_deck(&self) {
        let mut messages = Vec::new();

        messages.push("You're THRUSTEES:".to_string());
        for (i, thrustee) in self.personal_deck.thrustees.iter().enumerate() {
            messages.push(format!("{}. {}", i + 1, &thrustee).clone());
        }

        messages.push("<br/>You're THRUSTERS:".to_string());
        for (i, thruster) in self.personal_deck.thrusters.iter().enumerate() {
            messages.push(format!("{}. {}", i + 1, &thruster).clone());
        }

        self.send_messages(&messages);
    }

    pub fn login(&mut self, split: Vec<&str>, lobbies: &mut HashMap<i32, Lobby>) {
        if split.len() != 3 {
            self.send_message("You must provide USER and PASSWORD for your account.");
            return;
        }

        let user = split[1];
        let pass = split[2];
        match self.db.borrow().login(user, pass) {
            // lol I'm not gonna handle multiple logins.
            // You get hacked u lose lmao
            // You login in from another device you chillin
            Some(doc) => {
                if let Ok(thrustees) = doc.get_array("thrustees") {
                    self.personal_deck.thrustees =
                        MongoDB::bson_array_to_strings(thrustees.to_vec());
                }

                if let Ok(thrusters) = doc.get_array("thrusters") {
                    self.personal_deck.thrusters =
                        MongoDB::bson_array_to_strings(thrusters.to_vec());
                }

                if let Ok(name) = doc.get_str("name") {
                    self.name = String::from(name);
                    let mut messages = vec![format!(
                        "Welcome back ([USER] >>>\"{}\"<<< [USER]) to THRUSTIN.",
                        name
                    )];
                    messages.push(String::new());
                    messages.append(&mut Lobby::list_messages(lobbies));
                    self.send_messages(&messages);
                }
                self.state = PlayerState::OutOfLobby;
                self.is_authenticated = true;
            }
            None => {
                self.send_message("Failed to login lol are you sure you know what you're doing?");
            }
        }
    }

    // static function so pl borrow can be compared against players
    pub fn name(
        split: Vec<&str>,
        pl: Rc<RefCell<Player>>,
        lobbies: &mut HashMap<i32, Lobby>,
        players: &mut HashMap<u32, Rc<RefCell<Player>>>,
    ) {
        let name = {
            let pl = pl.borrow();
            if split.len() != 2 {
                pl.send_message("You need to provide the correct arguments to the name command. Please, try again. By the way spaces are probably not allowed in game I think.");
                return;
            }
            split[1].to_string()
        };

        let msg_name_exists = "yo that name exists ya gotta pick something else aight?";
        // Check if name exists in players list
        {
            for player in players.values() {
                if name == player.borrow().name {
                    pl.borrow().send_message(msg_name_exists);
                    return;
                }
            }
        }
        // Check if name exists in db
        if pl.borrow().db.borrow().does_name_exist(&name) {
            pl.borrow().send_message(msg_name_exists);
            return;
        }

        let mut pl = pl.borrow_mut();
        pl.name = name.clone();
        let mut messages = vec![format!("Name set to: {}", &pl.name)];

        if pl.state == PlayerState::ChooseName {
            pl.state = PlayerState::OutOfLobby;
            messages.push(format!(
                "ok {}, now ur redy 2 THRUST, try '.help' for sum updated information",
                &pl.name
            ));
            messages.push(String::new());
            messages.append(&mut Lobby::list_messages(lobbies));
        }

        pl.send_messages(&messages);
    }

    pub fn new(
        token: u32,
        comm: Rc<RefCell<dyn Communication>>,
        db: Rc<RefCell<MongoDB>>,
    ) -> Player {
        Player {
            token: token,
            name: String::new(),
            state: PlayerState::ChooseName,
            lobby: -1,
            personal_deck: Deck::new(),
            comm,
            game: PlayerGame::new(),
            db,
            is_authenticated: false,
        }
    }

    pub fn new_endless_host(
        comm: Rc<RefCell<dyn Communication>>,
        db: Rc<RefCell<MongoDB>>,
    ) -> Player {
        Player {
            token: 0,
            name: "EndlessLobbyHostDoggo".to_string(),
            state: PlayerState::Playing,
            lobby: 0,
            personal_deck: Deck::new(),
            comm,
            game: PlayerGame::new(),
            db,
            is_authenticated: false,
        }
    }

    pub fn password(&mut self, split: Vec<&str>) {
        if !self.is_authenticated {
            self.send_message("Brethren, you must be authenticated to do this...");
            return;
        }

        if split.len() != 3 {
            self.send_message("INVALID!!!! InvaliDDD!!!!!! YOUR PASSWORD commands needs to be formatted correctly with the right arguments... God...");
            return;
        }

        if split[1] != split[2] {
            self.send_message("It looks like your password confirmation failed. Chances are you probably mistyped.");
            return;
        }

        if self.db.borrow().password(&self.name, split[1]) {
            self.send_message("Awesome, your password was changed. Don't forget that the next time you login. Duh.");
        } else {
            self.send_message("Catastrophic error probably occurred. I don't know, but it looks like your password was NOT saved.");
        }
    }

    pub fn register(&mut self, split: Vec<&str>, lobbies: &mut HashMap<i32, Lobby>) {
        if split.len() != 4 {
            self.send_message("Ok you've got an invalid number of parameters for registration.");
            return;
        }

        let pass = split[2];
        let pass_conf = split[3];
        if pass != pass_conf {
            self.send_message("Registration failed. The given password confirmation does not match the given password.");
            return;
        }

        let user = split[1];
        if self.db.borrow().register(user, pass) {
            self.name = String::from(user);
            self.is_authenticated = true;
            self.state = PlayerState::OutOfLobby;

            let mut messages = vec![String::from("Lol ok nice you registered and good to go.")];
            messages.push(String::new());
            messages.append(&mut Lobby::list_messages(lobbies));
            self.send_messages(&messages);
        } else {
            self.send_message("Registration has failed. Unable to add user to database. Maybe username isn't unique?");
        }
    }

    pub fn send_message(&self, message: &str) {
        self.comm.borrow().send_message(&self.token, message);
    }

    pub fn send_messages(&self, messages: &Vec<String>) {
        self.comm.borrow().send_messages(&self.token, messages);
    }

    pub fn thrust(&mut self, input: &str, split: &Vec<&str>) {
        if split.len() < 2 {
            self.display_deck();
            return;
        }

        // Add thrust depending if we detect underscore or not
        let thrusts = Deck::find_thrusts(input);
        if thrusts.is_empty() {
            self.send_message("No THRUST arguments found. Did you forget quotations? Try something like .t \"Hello there!\"");
            return;
        }

        let mut added_thrustees = Vec::new();
        let mut added_thrusters = Vec::new();
        for thrust in thrusts {
            if thrust.contains("_") {
                self.personal_deck.add_thrustee(&thrust);
                added_thrustees.push(thrust);
            } else {
                self.personal_deck.add_thruster(&thrust);
                added_thrusters.push(thrust);
            }
        }

        self.personal_deck.sort();

        let mut messages = Vec::new();
        if !added_thrustees.is_empty() {
            added_thrustees.sort();
            messages.push(String::from("Added to THRUSTEES:"));
            for (index, thrustee) in added_thrustees.iter().enumerate() {
                messages.push(format!("{}. {}", index + 1, thrustee));
            }
            if self.is_authenticated {
                self.db
                    .borrow()
                    .thrustees(&self.name, added_thrustees.clone());
            }
        }

        if !added_thrusters.is_empty() {
            added_thrusters.sort();
            // Add a new line if there is a message for added THRUSTEES
            if !added_thrustees.is_empty() {
                messages.push(String::new());
            }
            messages.push(String::from("Added to THRUSTERS:"));
            for (index, thruster) in added_thrusters.iter().enumerate() {
                messages.push(format!("{}. {}", index + 1, thruster));
            }
            if self.is_authenticated {
                self.db.borrow().thrusters(&self.name, added_thrusters);
            }
        }

        self.send_messages(&messages);
    }

    pub fn unthrust(&mut self) {
        self.personal_deck = Deck::new();
        if self.is_authenticated {
            self.db.borrow().unthrust(&self.name);
        }
        self.send_message(
            "Personal THRUSTS have been cleared! If this was an accident, Good Luck!",
        );
    }

    pub fn username(&mut self, split: Vec<&str>) {
        if !self.is_authenticated {
            self.send_message("Brethren, you must be authenticated to do this...");
            return;
        }

        if split.len() != 3 {
            self.send_message("Invalid number of arguments have been provided to .username command. Please try: `.username [USER] [USER]`");
            return;
        }

        if split[1] != split[2] {
            self.send_message(
                "I'm sorry. There was an error confirming your username. (Did you mistype?)",
            );
            return;
        }

        if self.db.borrow().username(&self.name, split[1]) {
            self.send_message("Congrats, your username was changed. Don't forget that the next time you login. Duh.");
        } else {
            self.send_message("Man I don't know what to say. There was an error saving the username to database MongoDB.");
        }
    }

    pub fn up_games_played(&self) {
        if self.is_authenticated {
            self.db.borrow().up_games_played(&self.name);
        }
    }

    pub fn up_games_won(&self) {
        if self.is_authenticated {
            self.db.borrow().up_games_won(&self.name);
        }
    }

    pub fn up_points_gained(&self) {
        if self.is_authenticated {
            self.db.borrow().up_points_gained(&self.name);
        }
    }

    pub fn who(pl: Rc<RefCell<Player>>, players: &mut HashMap<u32, Rc<RefCell<Player>>>) {
        let pl = pl.borrow();
        let token = pl.token;
        let mut messages = Vec::new();
        for pl_rc in players.values() {
            let pl = pl_rc.borrow();
            if pl.token == 0 {
                continue;
            }

            let mut person = "";
            if token == pl.token {
                person = " (You)";
            }

            let message = if pl.state == PlayerState::InLobby || pl.state == PlayerState::Playing {
                format!("{} in {}{}", pl.name, pl.lobby, person).to_string()
            } else {
                format!("{}{}", pl.name, person).to_string()
            };

            messages.push(message);
        }

        messages.sort_unstable_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        pl.send_messages(&messages);
    }
}
