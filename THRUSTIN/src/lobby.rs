pub mod lobby {
    pub enum lobby_state {
        waiting,
        playing
    }


    pub struct Lobby {
        //name of lobby
        pub name: std::string::String,

        //optional password for lobby
        pw: std::string::String,

        //number of players in room
        pub count: u32,

        //max number of players
        pub max: u32,

        //state of the lobby
        pub state: lobby_state,

        //lobby id
        pub id: u32,
    }
    

        pub fn new(name: std::string::String, pw: std::string::String, max: u32, id: u32) -> Lobby {
            Lobby {
                name: name,
                pw: pw,
                count: 0,
                max: max,
                state: lobby_state::waiting,
                id: id
            }
        }

}
