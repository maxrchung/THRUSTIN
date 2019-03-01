use crate::thrust;

#[derive(Clone, PartialEq, Debug)]
pub enum PlayerState {
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

    pub is_thrustee: bool,

    pub personal_deck: thrust::Deck,
}

pub fn new(token: &ws::util::Token) -> Player {
    Player {
        token: token.clone(),
        name: token.0.to_string(),
        state: PlayerState::OutOfLobby,
        lobby: -1,
        deck: thrust::Deck::new(),
        is_thrustee: false,
        personal_deck: thrust::Deck::new(),
    }
}
