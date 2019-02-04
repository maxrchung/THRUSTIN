use crate::thrust;

#[derive(Clone, Debug)]
pub enum PlayerState {
    OutOfLobby,
    InLobby,
    Playing
}

#[derive(Clone, Debug)]
pub struct Player {
    //name of player
    pub name: std::string::String,

    //player state
    pub state: PlayerState,

    //host
    pub host: bool,

    pub lobby: i32,

    pub deck: thrust::Deck,
        
}


pub fn new(name: std::string::String) -> Player {
    Player {
        name: name,
        state: PlayerState::OutOfLobby,
        host: false,
        lobby: -1,
        deck: thrust::Deck::new()
    }
}
