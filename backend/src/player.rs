use crate::communication::WebSocketCommunication;
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

    pub deck: Deck,

    pub personal_deck: Deck,

    pub points: u8,

    comm: Rc<RefCell<WebSocketCommunication>>,
}

impl Player {
    pub fn send_message(&self, message: &str) {
        self.comm.borrow().send_message(&self.token, message);
    }

    pub fn send_messages(&self, messages: &Vec<String>) {
        self.comm.borrow().send_messages(&self.token, messages);
    }

    pub fn new(token: u32, communication: Rc<RefCell<WebSocketCommunication>>) -> Player {
        Player {
            token: token,
            name: token.to_string(),
            state: PlayerState::ChooseName,
            lobby: -1,
            deck: Deck::new(),
            personal_deck: Deck::new(),
            points: 0,
            comm: communication,
        }
    }

    pub fn new_endless_host(communication: Rc<RefCell<WebSocketCommunication>>) -> Player {
        Player {
            token: 0,
            name: "EndlessLobbyHostDoggo".to_string(),
            state: PlayerState::Playing,
            lobby: 0,
            deck: Deck::new(),
            personal_deck: Deck::new(),
            points: 0,
            comm: communication,
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
            pl.send_message(&format!("Name set to: {}", &pl.name));

            if pl.state == PlayerState::ChooseName {
                pl.state = PlayerState::OutOfLobby;
                pl.send_message(&format!(
                    "ok {}, now ur redy 2 THRUST, try '.help' for sum more information",
                    &pl.name
                ));
            }
        }
    }
}

