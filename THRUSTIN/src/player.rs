use crate::thrust;
use crate::networking;
use std::collections::HashMap;
use ws::util::Token;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, PartialEq, Debug)]
pub enum PlayerState {
    ChooseName,
    OutOfLobby,
    InLobby,
    Playing,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub token: ws::util::Token,

    //name of player
    pub name: std::string::String,

    //player state
    pub state: PlayerState,

    pub lobby: i32,

    pub deck: thrust::Deck,

    pub personal_deck: thrust::Deck,
}

pub fn new(token: &ws::util::Token) -> Player {
    Player {
        token: token.clone(),
        name: token.0.to_string(),
        state: PlayerState::ChooseName,
        lobby: -1,
        deck: thrust::Deck::new(),
        personal_deck: thrust::Deck::new(),
    }
}

pub fn set_name(
    input: std::vec::Vec<&str>,
    id: Token,
    players: &mut HashMap<Token, Rc<RefCell<Player>>>,
    communication: &mut networking::Networking,
) {
    if input.len() < 2 {
        communication.send_message(&id, &format!("You need a name!"));
        return;
    }

    let p_name = input[1].to_string();

    for pl in players.values() {
        let name = &pl.borrow().name;
        if &p_name == name {
            communication.send_message(&id, "yo that name exists ya gotta pick something else aight?");
            return;
        }
    }

    if let Some(pl) = players.get_mut(&id) {
        let mut pl = pl.borrow_mut();
        pl.name = p_name.clone();
        communication.send_message(&id, &format!("Name set to: {}", &pl.name));
        if pl.state == PlayerState::ChooseName {
            pl.state = PlayerState::OutOfLobby;
        }
    } else {
        communication.send_message(&id, "Something ain't right here");
    }
}
