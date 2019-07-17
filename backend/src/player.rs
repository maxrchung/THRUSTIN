use crate::communication::Communication;
use crate::database::MongoDB;
use crate::player_game::PlayerGame;
use crate::thrust::Deck;
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
    // Whether or not user is logged in through DB
    is_authenticated: bool,
    pub game: PlayerGame,
    pub lobby: i32,
    // name of player
    pub name: String,
    pub personal_deck: Deck,
    // player state
    pub state: PlayerState,
    pub token: u32,

}

impl Player {
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

    pub fn login(&mut self, split: std::vec::Vec<&str>) {
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
                    self.personal_deck.thrustees = MongoDB::bson_array_to_strings(thrustees.to_vec());
                }

                if let Ok(thrusters) = doc.get_array("thrusters") {
                    self.personal_deck.thrusters = MongoDB::bson_array_to_strings(thrusters.to_vec());
                }

                if let Ok(name) = doc.get_str("name") {
                    self.name = String::from(name);
                    self.send_message(&format!(
                        "Welcome back ([USER] >>>\"{}\"<<< [USER]) to THRUSTIN.",
                        name
                    ));
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
        split: std::vec::Vec<&str>,
        pl: Rc<RefCell<Player>>,
        players: &mut HashMap<u32, Rc<RefCell<Player>>>,
    ) {
        let name = {
            let pl = pl.borrow();
            if split.len() != 2 {
                pl.send_message("You need a name!");
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
            is_authenticated: false
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
            is_authenticated: false
        }
    }

    pub fn register(&mut self, split: std::vec::Vec<&str>) {
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
            self.send_message("Lol ok nice you registered and good to go.");
            self.is_authenticated = true;
            self.state = PlayerState::OutOfLobby;
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
                self.db.borrow().update_thrustees(&self.name, added_thrustees.clone());
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
                self.db.borrow().update_thrusters(&self.name, added_thrusters);
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

    pub fn who(
        pl: Rc<RefCell<Player>>,
        players: &mut HashMap<u32, Rc<RefCell<Player>>>,
    ) {
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

            let message =
                if pl.state == PlayerState::InLobby || pl.state == PlayerState::Playing {
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
