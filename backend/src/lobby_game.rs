
use crate::thrust::Deck;
use std::collections::HashMap;

// Struct that manages game specific settings
// These are reset to new() at game start
#[derive(Clone, Debug)]
pub struct LobbyGame {
    //current thrustee (player)
    pub thrustee: usize,
    //Deck that lobby will be actively playing with
    pub deck: Deck,
    //Deck that lobby will keep as reference when needing to refill thrusters/thrustees
    pub deck_reference: Deck,
    //current thrustee (card)
    pub current_thrustee: String,
    pub current_thrusts: HashMap<u32, String>,
    //maps thrust index to token (end me)
    // BROKEN: Need to fix because players can leave and shift lobby order, just use token
    pub index_to_token: HashMap<i32, u32>,
    pub thrusted_players: Vec<u32>,
    pub thrustee_choices: Vec<String>,
}

impl LobbyGame {
    pub fn new() -> LobbyGame {
        LobbyGame {
            deck: Deck::new(),
            deck_reference: Deck::new(),
            current_thrustee: String::new(),
            current_thrusts: HashMap::new(),
            index_to_token: HashMap::new(),
            thrusted_players: Vec::new(),
            thrustee: 0,
            thrustee_choices: Vec::new()
        }
    }
}