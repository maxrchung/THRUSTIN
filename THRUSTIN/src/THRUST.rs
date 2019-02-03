use std::io::Read;

struct Deck {
	thrusters: Vec<String>,
	thrustees: Vec<String>,
}

impl Default for Deck {
	fn default() -> Deck {
		Deck{thrusters: Vec::new(), thrustees: Vec::new()}
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
		else if to_edit == "3" {
		}
	}
	else if input == "3" {
		join_lobby();
	}
	else if input == "4" {
		*running = false;
	}
}

fn join_lobby() {
}

fn main() {
	let mut running = true;
	let mut deck = Deck::default();

	while running {
		write_to_socket(& String::from("What would you like to do?\n\n1. View your deck\n2. Edit your deck\n3. Join a lobby\n4. Quit\n"));
		let input = read_from_socket();
		println!("");

		handle_input(&input, &mut deck, &mut running);
	}
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