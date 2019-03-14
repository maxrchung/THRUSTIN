use crate::networking;
use crate::thrust;
use std::collections::HashMap;
use ws::util::Token;

#[derive(Clone, PartialEq, Debug)]
pub enum PlayerState {
    ChooseName,
    OutOfLobby,
    InLobby,
    Playing,
    Choosing,
    Deciding
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
    players: &mut HashMap<Token, Player>,
    communication: &mut networking::Networking,
) {
    if input.len() < 2 {
        communication.send_message(&id, &format!("You need a name!"));
        return;
    }

    let p_name = input[1].to_string();

    for names in players.values() {
        if p_name == names.name {
            communication.send_message(
                &id,
                "yo that name exists ya gotta pick something else aight?",
            );
            return;
        }
    }

    if let Some(player) = players.get_mut(&id) {
        player.name = p_name.clone();
        communication.send_message(&id, &format!("Name set to: {}", &player.name));
        if player.state == PlayerState::ChooseName {
            player.state = PlayerState::OutOfLobby;
        }
    } else {
        communication.send_message(&id, "Something ain't right here");
    }
}
