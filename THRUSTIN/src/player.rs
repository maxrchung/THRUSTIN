use crate::thrust;

#[derive(Clone, Debug)]
pub enum PlayerState {
    OutOfLobby,
    InLobby,
    Playing
}

#[derive(Clone, Debug)]
pub struct Player {
    pub token: ws::util::Token,

    //name of player
    pub name: std::string::String,

    //player state
    pub state: PlayerState,

    //host
    pub host: bool,

    pub lobby: i32,

    pub deck: thrust::Deck,

    pub is_thrustee: bool,
        
}


pub fn new(token: &ws::util::Token) -> Player {
    Player {
        token: token.clone(),
        name: "some_SHIT".to_string(),
        state: PlayerState::OutOfLobby,
        host: false,
        lobby: -1,
        deck: thrust::Deck::new(),
        is_thrustee: false,
    }
}
