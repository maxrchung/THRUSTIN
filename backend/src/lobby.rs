use crate::lobby_game::LobbyGame;
use crate::player::{Player, PlayerState};
use crate::player_game::PlayerGame;
use crate::thrust::Deck;
use std::cell::{RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::u8;
use std::usize;

#[derive(Clone, PartialEq, Debug)]
pub enum LobbyState {
    Playing,
    Waiting,
}

#[derive(Clone, Debug)]
pub struct Lobby {
    game: LobbyGame,
    //max hand size
    hand_size: u8,
    //host of lobby
    host: Rc<RefCell<Player>>,
    // How many house THRUSTS are being used
    house: usize,
    //list of players
    list: Vec<Rc<RefCell<Player>>>,
    //lobby id
    id: i32,
    //max number of players
    max_players: usize,
    //points
    max_points: u8,
    max_thrustee_choices: u8,
    //optional password for lobby
    pw: String,
    //lobby state
    state: LobbyState,
}

impl Lobby {
    fn build_thrusters_messages(thrusters: &Vec<String>) -> Vec<String> {
        let mut messages = vec!["Here are your THRUSTERS:".to_string()];
        for (index, thruster) in thrusters.iter().enumerate() {
            // Convert from 0-indexing to 1-indexing
            messages.push(format!("{}. {}", &index + 1, &thruster).to_string());
        }
        messages
    }

    fn get_joining_pl_state(
        lob: &mut Lobby,
        pl: &mut RefMut<Player>,
        messages: &mut Vec<String>,
        pl_rc: &Rc<RefCell<Player>>,
    ) -> Option<PlayerState> {
        // Distribute thrusters to player
        lob.refill_thrusters(pl);

        let mut wait: bool = false;

        // If lobby was empty before this guy joined, then they become THRUSTEE, otherwise, basically do what normal join_lobby does (yea this is fucked fk me doggo)
        if lob.list.len() == 0 {
            pl.state = PlayerState::Choosing;
            let mut messages =
                vec!["Welcome to the 『Endless Lobby』, big doggo. You lucky, family, you are THRUSTEE!!!!.. . Choose now...    .".to_string()];
            messages.extend(lob.print_thrustee_choices());
            pl.send_messages(&messages);
            pl.lobby = lob.id;
            lob.list.push(pl_rc.clone());
            lob.game.thrustee = 0;
            return None; //dude lmao
        } else {
            Lobby::handle_join_cases(&pl, &lob, &mut wait, messages);
        }

        if wait {
            return Some(PlayerState::Waiting);
        } else {
            return Some(PlayerState::Playing);
        }
    }

    fn handle_join_cases(
        pl: &RefMut<Player>,
        lob: &Lobby,
        wait: &mut bool,
        messages: &mut Vec<String>,
    ) {
        let thrustee = lob.list[lob.game.thrustee].borrow();

        match thrustee.state {
            PlayerState::Choosing => {
                *wait = true;
                messages.push(format!(
                    "THRUSTEE {} is currently CHOOSING next THRUSTEE. Hold on tight!",
                    &thrustee.name
                ));
            }

            PlayerState::Deciding => {
                messages.push(format!(
                    "This is {}'s THRUSTEE for you: {}<br/>",
                    &thrustee.name, &lob.game.current_thrustee
                ));
                messages.extend(Lobby::build_thrusters_messages(&pl.game.deck.thrusters));
            }

            _ => (),
        }
    }

    fn leave(&mut self, pl: Rc<RefCell<Player>>) {
        // borrow shenanigans
        {
            let pl = pl.borrow();

            // Much easier to clear single person lobby than multiple
            if self.list.len() == 1 {
                self.list.clear();
            } else {
                let pl_ind = self.search_token(pl.token).unwrap();
                let next_ind = (pl_ind + 1) % self.list.len();
                // This needs to be calculated as if pl has been removed
                let next_thrustee = pl_ind % (self.list.len() - 1);

                // Set up messages
                let mut messages = vec![String::from(format!("{} left the lobby..", pl.name))];
                // Handle if player is host
                if pl.token == self.host.borrow().token {
                    self.host = self.list[next_ind].clone();
                    messages.push(String::from(format!(
                        "Chief left so now we got a new one --> {}",
                        self.host.borrow().name
                    )));
                }

                let mut did_thrustee_change = false;
                // Handle if player is Choosing
                if pl.state == PlayerState::Choosing {
                    self.game.thrustee = next_thrustee;
                    did_thrustee_change = true;
                    // Next player chooses and replenish cards
                    let mut next = self.list[next_ind].borrow_mut();
                    next.state = PlayerState::Choosing;
                    messages.push(String::from(format!(
                        "Lol yo bro 'cause the THRUSTEE left {} is choosin' the next THRUSTEE now!",
                        next.name
                    )));
                }
                // Handle if player is Deciding
                else if pl.state == PlayerState::Deciding {
                    self.game.thrustee = next_thrustee;
                    did_thrustee_change = true;
                    // Next player decides
                    let mut next = self.list[next_ind].borrow_mut();
                    next.state = PlayerState::Deciding;
                    messages.push(String::from(format!("Lmao the THRUSTEE left and you're next in line, so {} gets to decide which THRUST wins lmfao.", next.name)));
                }
                // Handle if player is Normal
                else {
                    if pl_ind < self.game.thrustee {
                        self.game.thrustee = self.game.thrustee - 1;
                    }
                }

                // Remove player after we are done with managing player state
                self.list.remove(pl_ind);

                // Send appropriate message
                for pl_rc in &self.list {
                    let pl = pl_rc.borrow();
                    if did_thrustee_change && pl.state == PlayerState::Choosing {
                        let mut thrustee_messages = messages.clone();
                        // Add a linebreak for better visibility
                        thrustee_messages.last_mut().unwrap().push_str("<br/>");
                        // Notify new Choosing THRUSTEE of choices
                        thrustee_messages.extend(self.print_thrustee_choices());
                        pl.send_messages(&thrustee_messages);
                    } else {
                        pl.send_messages(&messages);
                    }
                }
            }
        }

        // Player mut and specific operations
        let mut pl = pl.borrow_mut();
        pl.lobby = -1;
        pl.state = PlayerState::OutOfLobby;
        pl.send_message("You have been leaved from the lobby okay!");
    }

    fn new(
        player: &Rc<RefCell<Player>>,
        id: i32,
        pass: &str,
        max_players: usize,
        max_points: u8,
        house: usize,
    ) -> Lobby {
        Lobby {
            hand_size: 5,
            id,
            max_points,
            game: LobbyGame::new(),
            host: player.clone(),
            house,
            list: Vec::new(),
            max_players,
            max_thrustee_choices: 3,
            pw: String::from(pass),
            state: LobbyState::Waiting,
        }
    }

    fn print_thrustee_choices(&self) -> Vec<String> {
        let mut messages = vec!["your THRUSTEE Choices:".to_string()];
        for (index, thrustee) in self.game.thrustee_choices.iter().enumerate() {
            // Convert from 0-indexing to 1-indexing
            messages.push(format!("{}. {}", &index + 1, &thrustee).to_string());
        }
        messages
    }

    fn search_token(&self, token: u32) -> Option<usize> {
        for (i, pl) in self.list.iter().enumerate() {
            let tok = pl.borrow().token;
            if tok == token {
                return Some(i);
            }
        }

        None
    }

    fn send_message(&self, message: &str) {
        for pl in &self.list {
            pl.borrow().send_message(&message);
        }
    }

    fn send_messages(&self, messages: Vec<String>) {
        for pl in &self.list {
            pl.borrow().send_messages(&messages);
        }
    }

    // refill_thrusters() but does not clear deck beforehand
    fn refill_remaining_thrusters(&mut self, pl: &mut Player) {
        // Distribute thrusters to player to fill thrusters
        for _ in pl.game.deck.thrusters.len()..self.hand_size as usize {
            if let Some(card) = self.game.deck.thrusters.pop() {
                pl.game.deck.thrusters.push(card);
            } else {
                // Refill thrusters if empty
                self.game.deck.thrusters = self.game.deck_reference.thrusters.clone();
                self.game.deck.shuffle_thrusters();
                pl.game
                    .deck
                    .thrusters
                    .push(self.game.deck.thrusters.pop().unwrap());
            }
        }
    }

    fn refill_thrustees(&mut self) {
        // Setup new thrustee choices
        for _ in 0..self.max_thrustee_choices {
            if let Some(card) = self.game.deck.thrustees.pop() {
                self.game.thrustee_choices.push(card);
            } else {
                self.game.deck.thrustees = self.game.deck_reference.thrustees.clone();
                self.game.deck.shuffle_thrustees();
                self.game
                    .thrustee_choices
                    .push(self.game.deck.thrustees.pop().unwrap());
            }
        }
    }

    fn refill_thrusters(&mut self, pl: &mut Player) {
        // Clear dude's deck beforehand
        pl.game.deck.thrusters.clear();
        self.refill_remaining_thrusters(pl);
    }

    pub fn end(&mut self, pl: Rc<RefCell<Player>>) {
        {
            let pl = pl.borrow();
            if !self.is_host(pl.token) {
                pl.send_message(&format!(
                    "Only chief shall have the privilege to end the game."
                ));
                return;
            }
        }

        self.end_game("Yo guys, the game's been manually ended by the chief almighty. Yall have been returned to the lobby setup area.".to_string());
    }

    pub fn end_game(&mut self, end_message: String) {
		// Each player will have a unique message depending on EXP gained & whether authenticated or not. End_messages contains each unique one
		let end_messages: Vec<String> = self.give_players_exp(end_message);
        self.state = LobbyState::Waiting;
        // Change players to inlobby state and send respective end_message to each. 
		// Assumes playerlist is accessed in same order as give_players_exp loop does
		for i in 0..end_messages.len() {
			let mut player = self.list[i].borrow_mut();
			player.state = PlayerState::InLobby;
			player.send_message(&end_messages[i]);
		}
    }

	pub fn give_players_exp(&mut self, end_message: String) -> Vec<String> {
		let points_in_lobby = self.points_in_lobby();
		let num_player = self.list.len();
		let mut messages = vec![end_message; self.list.len()];
		let mut level_messages = vec![];

		for i in 0..self.list.len() {
			let mut player = self.list[i].borrow_mut();
			if player.is_authenticated {
				let mut exp_gained = 0;
				let points_won = player.game.points;

				// if winner
				if points_won >= self.max_points {
					exp_gained += points_in_lobby + num_player as u8;
				}
				else {
					exp_gained += points_won + num_player as u8;
				}

				player.up_exp(exp_gained as i32);
				messages[i] = format!("{}<br/>Bro congratulation! You have receive {} experience points, congratulation!", messages[i], exp_gained);

				let mut leveled = false;
				// Possible to level multiple times in earlier levels so I used While Loop
				while player.level != 100 && self.ready_to_level(player.level, player.exp) {
					leveled = true;
					let current_level: i32 = player.level;
					player.up_level(self.get_exp_from_level(current_level));
				}

				if leveled {
					level_messages.push(format!(
						"{} has LEVELED UP!!!! Congratulation, {}, you are now level {}!!", 
						player.name, player.name, player.level));
				}
			}
		}
		let level_messages = level_messages.join("<br/>");
		// Append leveling messages to individual players' EXP gain messages
		for i in 0..messages.len() {
			messages[i] = format!("{}<br/>{}", messages[i], level_messages);
		}
		return messages;
	}

	pub fn ready_to_level(&self, level: i32, exp: i32) -> bool {
		return exp >= self.get_exp_from_level(level);
	}

	pub fn get_exp_from_level(&self, level: i32) -> i32 {
		let exponent: f32 = 2.15;
		let level = level as f32;
    	level.powf(exponent).round() as i32
	}

	pub fn points_in_lobby(&self) -> u8 {
		let mut total_points = 0;
		for rc in &self.list {
			let player = rc.borrow();
			total_points += player.game.points;
		}
		return total_points
	}

    pub fn choose(&mut self, input: Vec<&str>, pl: Rc<RefCell<Player>>) {
        if input.len() != 2 {
            pl.borrow().send_message("ya need to pick a NUMERIC, Boy");
            return;
        }

        // Use i32 so index-1 doesn't underflow
        match input[1].parse::<i32>() {
            Ok(index) => {
                // Convert from 1-indexing to 0-indexing
                let index = index - 1;
                if index < self.max_thrustee_choices as i32 && index > -1 {
                    // Scope refcell borrow
                    let name;
                    {
                        let mut pl = pl.borrow_mut();

                        // Removed selected choice
                        let card = self.game.thrustee_choices.remove(index as usize);
                        self.game.current_thrustee = card;
                        pl.state = PlayerState::Deciding;

                        // Refill choices
                        self.game.thrustee_choices.clear();
                        self.refill_thrustees();
                        name = pl.name.clone();
                    }

                    // Notify players
                    for (i, pl_rc) in self.list.iter().enumerate() {
                        let mut pl = pl_rc.borrow_mut();
                        let mut messages = vec![format!(
                            "{} has chosen this new THRUSTEE:<br/>{}<br/>",
                            name, &self.game.current_thrustee
                        )
                        .to_string()];

                        // Change Waiting players to Playing
                        if pl.state == PlayerState::Waiting {
                            pl.state = PlayerState::Playing;
                        }

                        if i == self.game.thrustee {
                            messages.push(
                                "get Ready to decide best THRUSTER for THRUSTING!".to_string(),
                            );
                            pl.send_messages(&messages);
                        } else {
                            messages
                                .extend(Lobby::build_thrusters_messages(&pl.game.deck.thrusters));
                            pl.send_messages(&messages);
                        }
                    }
                } else {
                    pl.borrow().send_message("That shit's out of bound bro");
                }
            }
            _ => {
                pl.borrow().send_message(
                    "That is an invalid parameter my chieftain, use an index instead dawggo.",
                );
            }
        };
    }

    pub fn decide(&mut self, input: Vec<&str>, pl: Rc<RefCell<Player>>) {
        if input.len() != 2 {
            pl.borrow().send_message("ya need to pick a numbert boi");
            return;
        }

        // Use i32 so index-1 doesn't underflow
        match input[1].parse::<i32>() {
            Ok(index) => {
                // Convert from 1-indexing to 0-indexing
                let index = index - 1;
                if index < self.game.current_thrusts.len() as i32 && index > -1 {
                    // Because of multiple mutable references
                    let (name, token, chosen_thrust) = {
                        let mut pl = pl.borrow_mut();
                        let name = pl.name.clone();

                        // Get chosen thrust
                        let (token, chosen_thrust) = self
                            .game
                            .current_thrusts
                            .get(&(index as usize))
                            .unwrap()
                            .clone();

                        // Clear thrust values
                        self.game.current_thrusts.clear();
                        self.game.thrusted_players.clear();

                        // Set current THRUSTEE to THRUSTER state
                        pl.state = PlayerState::Waiting;
                        (name, token, chosen_thrust)
                    };

                    let (chosen_thruster_pts, chosen_thruster_name) = {
                        // Assign picked thruster a point
                        match self.search_token(token) {
                            Some(index) => {
                                let (pts, winner) = {
                                    let mut chosen_thruster = self.list[index].borrow_mut();
                                    chosen_thruster.game.points += 1;
                                    chosen_thruster.up_points_gained();
                                    (chosen_thruster.game.points.clone(), chosen_thruster.name.clone())
                                };

                                // Check if winner
                                if pts >= self.max_points {
                                    self.list[index].borrow().up_games_won();
                                    let messages = vec![
                                        format!("{} has chosen this THRUSTER as the chosen THRUST, bois:<br/>{}<br/>", &name, &chosen_thrust),
                                        format!("Congratulations, {}! You're Winner! Everyone else, You're Loser! Game has been put into waiting state, THRUSTIN'ers!<br/>",  winner)
                                    ];
                                    self.end_game(messages.join("<br/>"));
                                    return;
                                }
                                (pts, winner)
                            }
                            None => (0, String::from("[[[THE GUY WHO LEFT (look imma be real it's easier right now for me to jam some placeholder text here than properly handle THRUSTERS who leave the game after THRUSTING yeah aite we're just gonna clear his points)]]]"))
                        }
                    };

                    // Assign next THRUSTEE
                    self.game.thrustee = (self.game.thrustee + 1) % self.list.len();

                    // Initialized outside so player.name and chosen_thrust can be kept
                    let common = vec![format!(
                        "{} has chosen this THRUSTER as the chosen THRUST, bois:<br/>{}<br/>",
                        &name, &chosen_thrust
                    )
                    .to_string()];

                    let thrustee_name = self.list[self.game.thrustee].borrow().name.clone();
                    for (i, pl) in self.list.iter().enumerate() {
                        let mut messages = common.clone();

                        messages.push(format!(
                            "The winning THRUSTER, {} now has {}/{} point(s)! Watch out!<br/>",
                            &chosen_thruster_name, &chosen_thruster_pts, self.max_points
                        ));

                        // If THRUSTEE, then set him up to be choosing next shit
                        if i == self.game.thrustee {
                            let mut thrustee_player = pl.borrow_mut();
                            thrustee_player.state = PlayerState::Choosing;
                            messages.push(
                                "You are the neXt THRUSTEE! GetT ready to CHOOSE a good THRUSTEE!"
                                    .to_string(),
                            );
                            messages.extend(self.print_thrustee_choices());
                        } else {
                            let mut thruster_player = pl.borrow_mut();
                            // Set players to Waiting so they can't submit THRUSTER after THRUST is already chosen
                            thruster_player.state = PlayerState::Waiting;
                            messages.push(format!(
                                "{} is choosing.... get rdy to THRUST.....",
                                &thrustee_name
                            ));
                        }
                        pl.borrow().send_messages(&messages);
                    }
                } else {
                    pl.borrow().send_message("That shit's out of bound bro");
                }
            }
            _ => {
                pl.borrow()
                    .send_message("That is an invalid parameter, use an index instead");
            }
        };
    }

    pub fn host(&mut self, input: Vec<&str>, pl: Rc<RefCell<Player>>) {
        let pl = pl.borrow();
        if !self.is_host(pl.token) {
            pl.send_message("Only chief can change the chief!");
            return;
        }

        if input.len() != 2 {
            pl.send_message("Who's the new chief tho");
            return;
        }

        let new_host = input[1];
        if self.host.borrow().name == new_host {
            pl.send_message("You're already chief!!");
            return;
        }

        for players_ in self.list.iter() {
            let players = players_.borrow();
            if players.name == new_host {
                self.host = players_.clone();
                players.send_message("You are now chief!");
                pl.send_message(&format!("{} is now chief!", players.name));
                return;
            }
        }

        pl.send_message("Player not in lobby.");
    }

    pub fn house(&mut self, input: Vec<&str>, pl: Rc<RefCell<Player>>) {
        let pl = pl.borrow();
        if !self.is_host(pl.token) {
            pl.send_message(&format!("Only chief can decide whether or not to use the house (default provided) THRUSTS for THRUSTING!"));
            return;
        }

        if input.len() != 2 {
            pl.send_message("Hello, you need to provide a value to the ~.house~ command.");
            return;
        }

        match input[1].to_string().parse::<usize>() {
            Ok(house) => {
                self.house = house;
                pl.send_message(&format!("House THRUSTS usage set to: {}", self.house));
            }

            _ => pl.send_message(&"Failed house command... only positive numbers dude!!!"),
        }
    }

    pub fn info(&self, pl: Rc<RefCell<Player>>) {
        let pl = pl.borrow();
        let mut info = Vec::new();
        info.push(format!("\\\\Lobby info//"));
        info.push(format!("ID: {}", self.id));
        if self.is_host(pl.token) {
            info.push(format!(
                "***(Only chief [that's you!] may see this!) Password: {}",
                self.pw
            ));
        }
        info.push(format!("Chief: {}", self.host.borrow().name));
        info.push(format!("Players: {}/{}", self.list.len(), self.max_players));
        info.push(format!("Max points? {}", self.max_points));
        info.push(format!("House THRUSTS? {}", self.house));
        info.push(format!("THRUSTEES? {}", self.max_thrustee_choices));
        info.push(format!("THRUSTERS? {}", self.hand_size));
        pl.send_messages(&info);
    }

    pub fn is_host(&self, player: u32) -> bool {
        if self.host.borrow().token == player {
            return true;
        } else if let Some(ind) = self.search_token(player) {
            return self.list[ind].borrow().is_chieftain();
        }
        false
    }

    pub fn kick(&mut self, input: Vec<&str>, pl: Rc<RefCell<Player>>) {
        // Scope guards to avoid borrow panic when THRUSTEE is kicked
        let kick_ind = {
            let mut kick_ind = -1;
            let pl = pl.borrow();
            if !self.is_host(pl.token) {
                pl.send_message("Only chief can kick em!");
                return;
            }

            if input.len() != 2 {
                pl.send_message("who we kickkin? TELL ME!");
                return;
            }

            let kick = input[1];
            if self.host.borrow().name == kick {
                pl.send_message("u cant kick ursel!!");
                return;
            }

            for (i, players) in self.list.iter().enumerate() {
                let players = players.borrow();
                if players.name == kick {
                    kick_ind = i as i32;
                    break;
                }
            }
            kick_ind
        };

        if kick_ind >= 0 {
            self.leave(self.list[kick_ind as usize].clone());
            return;
        }

        let pl = pl.borrow();
        pl.send_message("Player not in lobby.");
    }

    pub fn join(input: Vec<&str>, pl: Rc<RefCell<Player>>, lobby: &mut HashMap<i32, Lobby>) {
        if input.len() < 2 {
            pl.borrow().send_message("Lobby name required!");
            return;
        }

        match input[1].to_string().parse::<i32>() {
            Ok(lobby_id) => {
                let mut pl_mut = pl.borrow_mut();
                let mut messages = Vec::new();
                if let Some(mut lob) = lobby.get_mut(&lobby_id) {
                    // Lobby full check
                    if lob.list.len() >= lob.max_players {
                        pl_mut.send_message("bro this lobbBY is FULLLLL!!");
                        return;
                    }

                    //Lobby Password Check
                    if lob.pw != "" {
                        if input.len() != 3 {
                            pl_mut.send_message("Ya need a password BR)");
                            return;
                        } else if lob.pw != input[2] {
                            pl_mut.send_message("loll wrong pw haha");
                            return;
                        }
                    }
                    messages.push(format!("Joined: {:#?}", &lobby_id));

                    // Reset player game settings on join
                    pl_mut.game = PlayerGame::new();
                    pl_mut.state = if lob.state == LobbyState::Playing {
                        if let Some(state) =
                            Lobby::get_joining_pl_state(&mut lob, &mut pl_mut, &mut messages, &pl)
                        {
                            state
                        } else {
                            return;
                        }
                    } else {
                        PlayerState::InLobby
                    };

                    lob.send_message(&format!("{} has joined the lobby.", pl_mut.name));
                    // adding the new player to lobby
                    pl_mut.lobby = lob.id;
                    pl_mut.up_games_played();
                    lob.list.push(pl.clone());
                    pl_mut.send_messages(&messages);
                } else {
                    pl_mut.send_message("Lobby does not exist.");
                }
            }

            _ => pl
                .borrow()
                .send_message("nibba that is a invalid input my nibba"), //i love rust
        }
    }

    // This feeds into leave()
    // This function is separate from leave() because this also manages removing the lobby if it is empty
    pub fn leave_and_delete(pl: Rc<RefCell<Player>>, lobbies: &mut HashMap<i32, Lobby>) {
        let lobby = pl.borrow().lobby.clone();
        if let Some(lobby) = lobbies.get_mut(&lobby) {
            lobby.leave(pl);
            // Don't delete lobby if it is endless
            if lobby.list.len() == 0 && lobby.id != 0 {
                let id = lobby.id;
                lobbies.remove(&id);
            }
        }
    }

    pub fn list(pl: Rc<RefCell<Player>>, lobbies: &mut HashMap<i32, Lobby>) {
        let pl = pl.borrow();
        let messages = Lobby::list_messages(lobbies);
        pl.send_messages(&messages);
    }

    // This function is extrapolated out so it can be used for all .login, .register, .name, .list commands
    pub fn list_messages(lobbies: &mut HashMap<i32, Lobby>) -> Vec<String> {
        let mut messages = vec![String::from("A current exploration of lobbies that are available to be joined into is as follows below. Simply `.join [ID]` to enter. Lobby 0 is an endless lobby. It's always gonna be there.")];
        for lob in lobbies.values() {
            let state = match &lob.state {
                LobbyState::Playing => "Playing",
                LobbyState::Waiting => "Waiting",
            };
            messages.push(
                format!(
                    "ID: {} | Password: {} | Players: {}/{} | Currently: {}",
                    lob.id,
                    if lob.pw.is_empty() { "❌" } else { "✅" },
                    lob.list.len(),
                    lob.max_players,
                    state
                )
                .to_string(),
            );
        }

        messages.sort_unstable_by(|a, b| a.cmp(&b));
        messages
    }

    pub fn make(
        input: Vec<&str>,
        pl_rc: Rc<RefCell<Player>>,
        lobby_id: &mut i32,
        lobbies: &mut HashMap<i32, Lobby>,
    ) {
        let mut pl = pl_rc.borrow_mut();
        if input.len() > 2 {
            pl.send_message("Yo you gotta give the right parameters into .make bro!");
            return;
        }

        let pass = if input.len() > 1 { input[1] } else { "" };

        let mut lobby = Lobby::new(&pl_rc, *lobby_id, pass, 10, 7, 420);
        pl.lobby = lobby_id.clone();
        pl.state = PlayerState::InLobby;
        lobby.list.push(pl_rc.clone());

        lobbies.insert(lobby_id.clone(), lobby.clone());
        pl.send_message(&format!("Created lobby: {}", lobby_id));
        *lobby_id = *lobby_id + 1;
    }

    pub fn make_endless(
        pl_rc: &Rc<RefCell<Player>>,
        lobby_id: &mut i32,
        lobbies: &mut HashMap<i32, Lobby>,
    ) {
        let mut new_lobby = Lobby::new(&pl_rc, 0, "", usize::MAX, u8::MAX, usize::MAX);
        new_lobby.start_endless();

        lobbies.insert(lobby_id.clone(), new_lobby.clone());
    }

    pub fn password(&mut self, input: Vec<&str>, pl: Rc<RefCell<Player>>) {
        let pl = pl.borrow();
        if !self.is_host(pl.token) {
            pl.send_message("only chief sets password!!!");
            return;
        }

        if input.len() != 2 {
            pl.send_message("?? what's the pass boss??");
            return;
        }

        let password = input[1];
        self.pw = password.to_string();
        pl.send_message(&format!(
            "Now, the password has now been locked and loaded, my dude, now it's: {}",
            password
        ));
    }

    pub fn players(&mut self, input: Vec<&str>, pl: Rc<RefCell<Player>>) {
        let pl = pl.borrow();
        if !self.is_host(pl.token) {
            pl.send_message("only chief sets MAXP LAYER!");
            return;
        }

        if input.len() != 2 {
            pl.send_message("ya gotta set the new limit");
            return;
        }

        match input[1].to_string().parse::<usize>() {
            Ok(max) => {
                if max > 64 {
                    pl.send_message(&format!("woah thats 2many people chill! haha"));
                    return;
                }

                if max < self.list.len() {
                    pl.send_message(&format!("too many players in here right now man!"));
                    return;
                }
                self.max_players = max;
                pl.send_message(&format!("max players set to {}", self.max_players));
            }

            _ => pl.send_message(&"only numbers dude!!!"),
        }
    }

    pub fn points(&mut self, input: Vec<&str>, pl: Rc<RefCell<Player>>) {
        let pl = pl.borrow();
        if !self.is_host(pl.token) {
            pl.send_message("only chief sets points!");
            return;
        }

        if input.len() != 2 {
            pl.send_message("ya gotta set the new limit");
            return;
        }

        match input[1].to_string().parse::<u8>() {
            Ok(max) => {
                if max == 0 {
                    pl.send_message("bro dont make it 0 wtf man");
                    return;
                }
                self.max_points = max;
                pl.send_message(&format!("max points set to {}", self.max_points));
            }

            _ => pl.send_message(&"You have provided an invalid parameter."),
        }
    }

    pub fn send_message_from(&self, from: &Player, message: &str) {
        for pl in &self.list {
            pl.borrow().send_message_from(&from.name, &from.bg, &from.fg, message, from.level);
        }
    }

    pub fn thrustees(&mut self, input: Vec<&str>, pl: Rc<RefCell<Player>>) {
        let pl = pl.borrow();
        if !self.is_host(pl.token) {
            pl.send_message(
                "Only chief of the lobby is the only one who may set the THRUSTEE count.",
            );
            return;
        }

        if input.len() != 2 {
            pl.send_message(
                "A value must be provided to determine what the THRUSTEE count is to be.",
            );
            return;
        }

        match input[1].to_string().parse::<u8>() {
            Ok(max) => {
                if max < 2 {
                    pl.send_message(&format!("Brother, you must specify 2 or more for THRUSTEE count. This is so that we can guarantee some sort of picking decision for the THRUSTEE to partake in when selecting a desired THRUSTEE to use. Thank you for your understanding."));
                    return;
                }
                self.max_thrustee_choices = max;
                pl.send_message(&format!("max THRUSTEE set to {}", self.max_players));
            }

            _ => pl.send_message(&"Thou hast entered a value that we have deemed as unparsable for the requested THRUSTEE command that thou hast witch hath entered hitherto."),
        }
    }

    pub fn thrusters(&mut self, input: Vec<&str>, pl: Rc<RefCell<Player>>) {
        let pl = pl.borrow();
        if !self.is_host(pl.token) {
            pl.send_message("ONLY CHIEF CAN SET MAX THRUSTERS!!!");
            return;
        }

        if input.len() != 2 {
            pl.send_message("You need to give me a value man");
            return;
        }

        match input[1].to_string().parse::<u8>() {
            Ok(max) => {
                if max < 2 {
                    pl.send_message(&format!("bro u need at least two THRUSTERS, try again?"));
                    return;
                }

                self.hand_size = max;
                pl.send_message(&format!("max THRUSTERS set to {}", self.max_players));
            }

            _ => {
                pl.send_message(&"only positive numbers dude, please!!! (and not too big neither)")
            }
        }
    }

    pub fn thrust(&mut self, input: Vec<&str>, pl: Rc<RefCell<Player>>) {
        {
            let pl = pl.borrow();

            // Check if thrusted
            if self.game.thrusted_players.contains(&pl.token) {
                pl.send_message("You have already THRUSTED, you cannot THRUST again.");
                return;
            }

            // Check number of inputs
            if input.len() < 2 {
                pl.send_message(&"Index required!");
                return;
            }
        }

        // For handling mut borrow
        let resulting_thrust = {
            let mut pl = pl.borrow_mut();

            // Check correct # of thrusters
            let num_thrusters = input.len() as i32 - 1;
            let num_underscore = Deck::count_underscores(&self.game.current_thrustee);
            if num_thrusters != num_underscore {
                pl.send_message("bro that ain't the right number of THRUSTERS");
                return;
            }

            let mut resulting_thrust = self.game.current_thrustee.clone();
            let mut to_remove = HashSet::new();
            // Handle mutliple underscores
            for i in 1..input.len() {
                // Convert from 1-indexing to 0-indexing
                // Use i32 to account for underflow
                let index = match input[i].parse::<i32>() {
                    Ok(value) => value - 1,
                    Err(_) => {
                        pl.send_message("Yeah it looks like your THRUST didn't work. Index failed to be provided?");
                        return;
                    }
                };

                // Check if valid index
                if index >= pl.game.deck.thrusters.len() as i32 || index < 0 {
                    pl.send_message("That shit's out of bound bro");
                    return;
                }

                let picked_thruster = pl.game.deck.thrusters[index as usize].clone();
                to_remove.insert(picked_thruster.clone());
                // Surround with <u> to underline text
                let formatted_thruster = format!("<u>{}</u>", picked_thruster);
                resulting_thrust = Deck::thrust(&formatted_thruster, &resulting_thrust);
            }

            // Remove thrusted thrusters
            let mut updated_thrusters = Vec::new();
            for thruster in &pl.game.deck.thrusters {
                if !to_remove.contains(thruster) {
                    updated_thrusters.push(thruster.clone())
                }
            }
            pl.game.deck.thrusters = updated_thrusters;
            self.game.thrusted_players.push(pl.token.clone());

            // Handle picked
            self.game.current_thrusts.insert(
                self.game.current_thrusts.len(),
                (pl.token, resulting_thrust.clone()),
            );

            self.refill_remaining_thrusters(&mut pl);
            resulting_thrust
        };

        let mut messages = vec![format!(
            "{}. {}",
            // Use 1-indexing for showing result
            &self.game.current_thrusts.len(),
            &resulting_thrust
        )];

        // Check if everyone has thrusted
        let mut did_everyone_thrust = true;
        for (_, pl) in self.list.iter().enumerate() {
            let pl = pl.borrow();
            if pl.state != PlayerState::Deciding && !self.game.thrusted_players.contains(&pl.token)
            {
                did_everyone_thrust = false;
                break;
            }
        }
        if did_everyone_thrust {
            messages.push(String::from("<br/>Everyone has THRUSTED! By popular demand we are adding this message in to notify everyone that it's fine to choose a THRUST now. I didn't want it to have to come down to this, but I added it in due to pressure from our publishers."));
        }

        // Notify message
        self.send_messages(messages);
    }

    pub fn start(&mut self, pl: Rc<RefCell<Player>>) {
        {
            let pl = pl.borrow();

            if !self.is_host(pl.token) {
                pl.send_message(&format!("Only chief can start game!"));
                return;
            }
        }

        // Reset game settings
        self.game = LobbyGame::new();
        self.game.deck.clear();
        // Add in house cards to lobby deck if bool is true
        if self.house > 0 {
            let mut default_deck = Deck::default();
            default_deck.shuffle_deck();
            // Truncate to house limit
            default_deck.limit(self.house);
            self.game.deck = default_deck;
        }

        // Add each person's deck in
        {
            let decks: Vec<Deck> = self
                .list
                .iter()
                .map(|pl| pl.borrow().personal_deck.clone())
                .collect();
            for deck in decks {
                self.game.deck.thrustees.append(&mut deck.thrustees.clone());
                self.game.deck.thrusters.append(&mut deck.thrusters.clone());
            }
        }

        // Validate THRUSTEES (need 1 THRUSTEE)
        if self.game.deck.thrustees.is_empty() {
            pl.borrow().send_message("Dude, I can't start the game for you because yall don't got enough THRUSTEES. Here's a lil bit of mathematics:<br/>Total THRUSTEES HAS to BE GREATER THAN 0");
            return;
        }

        // Validate THRUSTERS (need 1 THRUSTER)
        if self.game.deck.thrusters.is_empty() {
            pl.borrow().send_message("Yo... got an issue boss, we don't go enough THRUSTERS. Let me calculate to tell you why:<br/>Total THRUSTERS HAS to BE GREATER THAN 0");
            return;
        }

        self.game.deck_reference = self.game.deck.clone();
        self.game.deck.shuffle_deck();
        self.state = LobbyState::Playing;
        self.refill_thrustees();

        // Elaborateness to call &mut self
        for i in 0..self.list.len() {
            let pl = self.list[i].clone();
            // While we're at it reset the player's game settings omegalul
            pl.borrow_mut().game = PlayerGame::new();
            self.refill_thrusters(&mut pl.borrow_mut());
        }

        let thrustee_name = self.list[self.game.thrustee].borrow().name.clone();
        for (i, pl) in self.list.iter().enumerate() {
            let mut pl = pl.borrow_mut();
            pl.state = PlayerState::Waiting;
            pl.up_games_played();

            if i == self.game.thrustee {
                pl.state = PlayerState::Choosing;
                let mut messages =
                    vec!["You are the THRUSTEE. Choose NOW..........<br/>".to_string()];
                messages.extend(self.print_thrustee_choices());
                pl.send_messages(&messages);
            } else {
                pl.send_message(&format!(
                    "You are a THRUSTER. waiting for a good THRUSTEE from {}; mmm baby!",
                    &thrustee_name
                ));
            }
        }
    }

    pub fn start_endless(&mut self) {
        self.state = LobbyState::Playing;
        self.game.deck = Deck::default();
        self.game.deck_reference = self.game.deck.clone();
        self.game.deck.shuffle_deck();
        self.refill_thrustees();
    }

    pub fn who(&self, pl: Rc<RefCell<Player>>) {
        let pl = pl.borrow();
        let token = &pl.token;
        let mut messages = Vec::new();

        for pl_rc in &self.list {
            let pl = pl_rc.borrow();
            let name = &pl.name;
            let level = &pl.level;

            let mut person = "";
            if token == &pl.token {
                person = " (You)";
            }

            let message = if self.is_host(pl.token) {
                format!("{} ({}): chief{}", name, level, person).to_string()
            } else {
                format!("{} ({}){}", name, level, person).to_string()
            };

            messages.push(message);
        }

        messages.sort_unstable_by(|a, b| a.cmp(&b));

        pl.send_messages(&messages);
    }

    pub fn who_in_game(&self, pl: Rc<RefCell<Player>>) {
        let pl = pl.borrow();
        let token = pl.token;
        let mut messages = Vec::new();
        for rc in &self.list {
            let player = rc.borrow();
            let you = if token == player.token {
                " (Yourself)"
            } else {
                ""
            };
            messages.push(format!(
                "{}/{} points: {}{}",
                player.game.points, self.max_points, player.name, you
            ));
        }

        // Sort the messages so that they are ordered by highest points
        messages.sort_unstable_by(|a, b| a.cmp(b).reverse());

        pl.send_messages(&messages);
    }
}
