#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

struct Deck {
	thrusters: Vec<String>,
	thrustees: Vec<String>,
}

impl Default for Deck { 
	fn default() -> Deck {
		Deck{thrusters: vec!["Kenny".to_string(), "Brenn".to_string(), "Jimmy".to_string(), "Max".to_string(), "Royce".to_string(), "Alex".to_string(), "homosexuality".to_string(), "heterosexuality".to_string(),
		"Gmaul".to_string(), "Runescape".to_string(), "dick".to_string(), "big juicy cock".to_string(), "osu!".to_string(), "peppy points".to_string(), "black people".to_string(), "tablet pen".to_string(),
		"ass fucking".to_string(), "Omega Sector".to_string(), "rape and pillage".to_string(), "tribal slaughter".to_string(), "cat ears".to_string(), "degeneracy".to_string(), "anime".to_string(),
		"hentai".to_string(), "big anime titty".to_string(), "gaming".to_string(), "3/5ths Compromise".to_string(), "cotton fields".to_string(), "slavery".to_string(), "furry".to_string(),
		"99 pure str".to_string(), "full combo".to_string(), "niseboi".to_string(), "Kenny getting fucked in the ass".to_string(), "racism".to_string(), "Riku".to_string(), "Rust".to_string(),
		"Rustacean".to_string(), "gay sex".to_string(), "trap".to_string(), "storyboard".to_string(), "sucking dick at osu!".to_string(), "Jews".to_string(), "Minecraft".to_string(),
		"hot anime sex".to_string(), "lolicon".to_string(), "swag42069".to_string(), "rock hard dick".to_string(), "anal sex while playing osu!".to_string(), "cheesy dick".to_string()],
		 thrustees: 
		 vec!["Every day before Kenny sleeps he really likes _____".to_string(), "Before heading over to the cotton fields you need to make sure to grab your _____".to_string(),
		 "I like whacking it to _____ while playing osu!".to_string(), "Before _____ was banned I really liked it".to_string(), "_____, that's what I rub my nipples out to".to_string(),
		 "In class I always make sure to _____ the fucker sitting next to me".to_string(), "_____ really got fucked in the ass by black people".to_string(), "I really want to _____ before I die".to_string(),
		 "There's only one more thing I like more than anime, and it's _____".to_string(), "Whenever I see _____ it reminds me of Kenny".to_string(), "Fuck _____ I'm going to play osu! and suck my own dick".to_string(),
		 "They should really bring back _____, I missed it when it was banned in 2020".to_string()]}
	}
}

impl Deck {
	fn add_thruster(&mut self, thruster: & std::string::String) {
		self.thrusters.push(thruster.to_string());
		self.sort_thrusters();
	}

	fn add_thrustee(&mut self, thrustee: & std::string::String) {
		self.thrustees.push(thrustee.to_string());
		self.sort_thrustees();
	}

	fn sort_thrusters(&mut self) {
		self.thrusters.sort();
	}

	fn sort_thrustees(&mut self) {
		self.thrustees.sort();
	}

	fn view_thrusters(&self) {
		if std::vec::Vec::is_empty(&self.thrusters) {
			write_to_socket(& "No thrusters in deck!".to_string());
		}
		else {
			for thruster in &self.thrusters {
				write_to_socket(& (*thruster).to_string());
			}
		}
	}

	fn view_thrustees(&self) {
		if std::vec::Vec::is_empty(&self.thrustees) {
			write_to_socket(& "No thrustees in deck!".to_string());
		}
		else {
			for thrustee in &self.thrustees {
				write_to_socket(& (*thrustee).to_string());
			}
		}
	}
}

// struct Game {
//     host: String,
//     players: Vec<String>,
// }

// impl Game {
// 	fn add_players(&mut self, player: &mut String) {     /////////Shit 
// 		self.players.push(player.to_string());
// 	}
// }
 
 struct GameDeck {
	player_thrusters: Vec<String>
 }

impl GameDeck {
	fn thrust(&mut self, index: i32, thrustee: & String) -> String {
		lazy_static! {
			static ref RE: Regex = Regex::new("[_]+").unwrap();
		}
		let result = RE.replace_all(&thrustee, &(self.player_thrusters[index as usize])[..]);
		self.player_thrusters.remove(index as usize);
		result.to_string()
	} 
}

fn write_to_socket(message: & String) {
    println!("{}", message);
}

fn read_from_socket() -> String {
	let mut input = String::new();
	std::io::stdin().read_line(&mut input);
	input.trim().to_string()
}

fn handle_input(input: &String, deck: &mut Deck, running: &mut bool) {
	if input == "1" {
		write_to_socket(& String::from("Thrusters:"));
		deck.view_thrusters();
		write_to_socket(& String::from("Thrustees:"));
		deck.view_thrustees();
	}
	else if input == "2" {
		write_to_socket(& String::from("Which would you like to edit?\n1. thrusters\n2. thrustees\n3. Go back\n"));
		let to_edit = read_from_socket();
		if to_edit == "1" {
			write_to_socket(& String::from("Please enter what thruster you would like to add to your thrusters."));
			let new_thruster = read_from_socket();
			deck.add_thruster(&new_thruster);
		}
		else if to_edit == "2" {
			let mut new_thrustee = String::new();
			write_to_socket(& String::from("Please enter what thrustee you would like to add to your thrustees."));
			
			while {
				new_thrustee = read_from_socket();
				if !new_thrustee.contains("_") {
					println!("Not valid thrustee. Please add blank space to allow thrusters to thrust into them.");
				}
				!new_thrustee.contains("_")
			} {}
			
			deck.add_thrustee(&new_thrustee);
		}
	}
	else if input == "3" {
		join_lobby(deck);
	}
	else if input == "4" {
		*running = false;
	}
}

fn join_lobby(deck: &mut Deck) {
	let mut connected = false; //placeholder w/e/wdfdfd fuck you

	connected = true; // DELETE LATER

	if connected {
		game_loop(deck); // <-------------- Royce's Shit
	}
	else {
		write_to_socket(& "Lobby join failed!".to_string());
	}
}

fn game_loop(deck: &mut Deck) {
	////////////////////////////////////////// Royce's Shit
}

fn test() {
	let mut deck = Deck::default();
	let mut game_deck = GameDeck{player_thrusters: deck.thrusters};

	let mut game_thrustees = &deck.thrustees;

	let product = game_deck.thrust(0, &game_thrustees[0]);

	println!("{}", product);

}

fn main() {
	test();

	// let mut running = true;
	// let mut deck = Deck::default();

	// while running {
	// 	write_to_socket(& String::from("What would you like to do?\n\n1. View your deck\n2. Edit your deck\n3. Join a lobby\n4. Quit\n"));
	// 	let input = read_from_socket();
	// 	println!("");

	// 	handle_input(&input, &mut deck, &mut running);
	// }
}

// let mut inst = Deck::default();
	// inst.view_thrusters();
	// inst.view_thrustees();

	// inst.add_thruster(&String::from("fuck yuo"));

	// inst.view_thrusters();


	// let test_thrusters = vec![String::from("Homosexuality"), String::from("Degeneracy"), String::from("Moefag"), String::from("fuckoff")];
	// let test_thrustees  = vec![String::from("Gay is ____"), String::from("Kenny is ____"), String::from("Fuck you _____")];
	// let thruster = String::from("Your Gay");
	// let thrustee = String::from("The reason Kenny is gay is ________.");

	// let mut instance = Deck::default();

	// let mut test = Deck{thrusters: test_thrusters, thrustees: test_thrustees};

	// test.sort_thrusters();

	// for x in &test.thrusters {
	// 	println!("{}", x);
	// }

	// test.sort_thrustees();

	// for x in &test.thrustees {
	// 	println!("{}", x);
	// }