use crate::thrust::Deck;

// Game settings specific to player
// Calling new() will easily reset settings on new game or join
#[derive(Clone, Debug)]
pub struct PlayerGame {
    pub deck: Deck,
    pub points: u8,
}

impl PlayerGame {
    pub fn new() -> PlayerGame {
        PlayerGame {
            deck: Deck::new(),
            points: 0,
        }
    }
}