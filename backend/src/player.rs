use crate::networking;
use crate::thrust;
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
    pub name: std::string::String,

    //player state
    pub state: PlayerState,

    pub lobby: i32,

    pub deck: thrust::Deck,

    pub personal_deck: thrust::Deck,

    pub points: u32,

    comm: Rc<RefCell<networking::Networking>>,
}

impl Player {
    pub fn send(&self, message: &str) {
        self.comm.borrow().send_message(&self.token, message);
    }

    pub fn send_multiple(&self, messages: Vec<String>) {
        self.comm.borrow().send_messages(&self.token, messages);
    }
}

pub fn new(token: u32, communication: Rc<RefCell<networking::Networking>>) -> Player {
    Player {
        token: token,
        name: token.to_string(),
        state: PlayerState::ChooseName,
        lobby: -1,
        deck: thrust::Deck::new(),
        personal_deck: thrust::Deck::new(),
        points: 0,
        comm: communication,
    }
}

pub fn new_endless_host(communication: Rc<RefCell<networking::Networking>>) -> Player {
    Player {
        token: 0,
        name: "EndlessLobbyHostDoggo".to_string(),
        state: PlayerState::Playing,
        lobby: 0,
        deck: thrust::Deck::new(),
        personal_deck: thrust::Deck::new(),
        points: 72742069,
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
            player.send("You need a name!");
            return;
        }
    }
    let p_name = input[1].to_string();

    {
        for pl in players.values() {
            let name = &pl.borrow().name;
            if &p_name == name {
                play.borrow()
                    .send("yo that name exists ya gotta pick something else aight?");
                return;
            }
        }
    }

    {
        let mut pl = play.borrow_mut();
        pl.name = p_name.clone();
        pl.send(&format!("Name set to: {}", &pl.name));

        if pl.state == PlayerState::ChooseName {
            pl.state = PlayerState::OutOfLobby;
            pl.send(&format!(
                "ok {}, now ur redy 2 THRUST, try '.help' for sum more information",
                &pl.name
            ));
        }
    }
}
