use crate::communication::Communication;
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
    pub token: u32,
    //name of player
    pub name: String,
    //player state
    pub state: PlayerState,
    pub lobby: i32,
    pub personal_deck: Deck,
    comm: Rc<RefCell<dyn Communication>>,
    pub game: PlayerGame,
}

impl Player {
    pub fn send_message(&self, message: &str) {
        self.comm.borrow().send_message(&self.token, message);
    }

    pub fn send_messages(&self, messages: &Vec<String>) {
        self.comm.borrow().send_messages(&self.token, messages);
    }

    pub fn new(token: u32, communication: Rc<RefCell<dyn Communication>>) -> Player {
        Player {
            token: token,
            name: String::new(),
            state: PlayerState::ChooseName,
            lobby: -1,
            personal_deck: Deck::new(),
            comm: communication,
            game: PlayerGame::new(),
        }
    }

    pub fn new_endless_host(communication: Rc<RefCell<dyn Communication>>) -> Player {
        Player {
            token: 0,
            name: "EndlessLobbyHostDoggo".to_string(),
            state: PlayerState::Playing,
            lobby: 0,
            personal_deck: Deck::new(),
            comm: communication,
            game: PlayerGame::new(),
        }
    }

    pub fn set_name(
        input: std::vec::Vec<&str>,
        play: Rc<RefCell<Player>>,
        players: &mut HashMap<u32, Rc<RefCell<Player>>>,
    ) {
        {
            let player = play.borrow();
            if input.len() < 2 {
                player.send_message("You need a name!");
                return;
            }
        }
        let p_name = input[1].to_string();

        {
            for pl in players.values() {
                let name = &pl.borrow().name;
                if &p_name == name {
                    play.borrow()
                        .send_message("yo that name exists ya gotta pick something else aight?");
                    return;
                }
            }
        }

        {
            let mut pl = play.borrow_mut();
            pl.name = p_name.clone();
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
    }

    pub fn handle_thrusteer_commands(&mut self, input: &str, split: &Vec<&str>) {
        if split.len() < 2 {
            self.display_deck();
            return;
        }

        // Add thrust depending if we detect underscore or not
        let thrusts = Deck::find_thrusts(input);
        for thrust in thrusts {
            if thrust.contains("_") {
                self.personal_deck.add_thrustee(&thrust);
                self.send_message(&format!("Added \"{}\" to THRUSTEES!", &thrust));
            } else {
                self.personal_deck.add_thruster(&thrust);
                self.send_message(&format!("Added \"{}\" to THRUSTERS!", &thrust));
            }
        }

        self.personal_deck.sort();
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

    pub fn clear_pers_deck(&mut self) {
        self.personal_deck = Deck::new();
        self.send_message(
            "Personal THRUSTS have been cleared! If this was an accident, Good Luck!",
        );
    }
}
