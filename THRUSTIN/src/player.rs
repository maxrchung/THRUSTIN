#[derive(Clone)]
pub enum player_state {
    empty,
    thruster,
    thrustee
}

#[derive(Clone)]
pub struct Player {
    //name of player
    pub name: std::string::String,

    //player state
    pub state: player_state,

    //host
    pub host: bool,

    pub lobby: i32
        
}


pub fn new(name: std::string::String) -> Player{
    Player {
        name: name,
        state: player_state::empty,
        host: false,
        lobby: -1
    }
}
