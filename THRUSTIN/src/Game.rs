pub struct Game {
    host: String,
    players: Vec<String>,

}

impl Default for Game {
    fn default() -> Game {
        Game{host: "Fuckoff".to_string(), players: Vec::new()} // insert default shit here
    }
}

impl Game {
}