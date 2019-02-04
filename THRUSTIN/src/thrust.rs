use regex::Regex;

#[derive(Clone, Debug)]
pub struct Deck {
	pub thrusters: Vec<String>,
	pub thrustees: Vec<String>,
}

impl Default for Deck { 
	fn default() -> Deck {
		Deck{thrusters: vec!["Kenny".to_string(), "Brenn".to_string(), "Jimmy".to_string(), "Max".to_string(), "Royce".to_string(), "Alex".to_string(), "homosexuality".to_string(), "heterosexuality".to_string(),
		"Gmaul".to_string(), "Runescape".to_string(), "dick".to_string(), "big juicy cock".to_string(), "osu!".to_string(), "peppy points".to_string(), "black people".to_string(), "tablet pen".to_string(),
		"ass fucking".to_string(), "Omega Sector".to_string(), "rape and pillage".to_string(), "tribal slaughter".to_string(), "cat ears".to_string(), "degeneracy".to_string(), "anime".to_string(),
		"hentai".to_string(), "big anime titty".to_string(), "gaming".to_string(), "3/5ths Compromise".to_string(), "cotton fields".to_string(), "slavery".to_string(), "furry".to_string(),
		"99 pure str".to_string(), "full combo".to_string(), "niseboi".to_string(), "Kenny getting fucked in the ass".to_string(), "racism".to_string(), "Riku".to_string(), "Rust".to_string(),
		"Rustacean".to_string(), "gay sex".to_string(), "trap".to_string(), "storyboard".to_string(), "sucking dick at osu!".to_string(), "Jews".to_string(), "Minecraft".to_string(),
		"hot anime sex".to_string(), "lolicon".to_string(), "swag42069".to_string(), "rock hard dick".to_string(), "anal sex while playing osu!".to_string(), "cheesy dick".to_string(),
		"Dream Solister".to_string(), "osu! UCI".to_string(), "McDonalds".to_string(), "Burger King".to_string(), "McSad".to_string(), "Pippins".to_string(), "bro I wanna really die".to_string(), "moist".to_string(), 
		"while still in my panties, I vomited from my jowls a chunky curd of phlegm".to_string(), "pulsating purple cock".to_string(), "gurgling a moist egg yolk while I blogged about it".to_string(), "thanks for the protein".to_string(),
		"keep my cock in your mouth until we get home".to_string(), "he told me that you were going to adore and cherish this big black cock".to_string(), "cum box".to_string(), "I'm harder than steel".to_string(),
		"I'm being raped by a monster".to_string(), "eating cum from my ass".to_string(), "BDSM".to_string(), "the salty drips of precum that are rubbed over my tongue".to_string(), "I roll my hips, his slick, hot crown, teasing at my star, until finally I feel it loosen enough".to_string(),
		"naked woman".to_string(), "young boy".to_string(), "aborted fetus".to_string(), "fucking a pregnant woman and her fetus at the same time".to_string(), "The Boy in the Mirror".to_string(), "getting a tablet pen stick in my dick".to_string(),
		"wiring up my dick to work as a tablet pen".to_string(), "a fast forward/backward movement of the hips, usually in sexual activity".to_string(), "when a guy goes in and out of a girl's vagina really fast getting ready to cum kind of like humping".to_string(),
		"a sexual act in which the man inserts a cigar into the vagina of his female partner, and then he smokes the cigar while blowing the smoke into his partner's orafice".to_string(),
		"trust, the hardest thing to gain and the easiest thing to lose".to_string(), "I actually want to die".to_string(), "I'm going to fucking kill you".to_string(), "shitting in a girl's vagina and fucking it".to_string(),
		"digging up a mostly rotted corpse, hook it up with wires that connect to a source of electricity, puppeteer it and have people pay to have sex with it".to_string(), "gayme jam".to_string(), "who wrote these".to_string(), "I gurgled a moist egg yolk and vomited from my jowls a chunky curd of phlegm that was dripping with roaches, maggots and mucus.".to_string()],

		 thrustees: 
		 vec!["Every day before Kenny sleeps he really likes _____".to_string(), "Before heading over to the cotton fields you need to make sure to grab your _____".to_string(),
		 "I like whacking it to _____ while playing osu!".to_string(), "Before _____ was banned I really liked it".to_string(), "_____, that's what I rub my nipples out to".to_string(),
		 "In class I always make sure to _____ the fucker sitting next to me".to_string(), "_____ really got fucked in the ass by black people".to_string(), "I really want to _____ before I die".to_string(),
		 "There's only one more thing I like more than anime, and it's _____".to_string(), "Whenever I see _____ it reminds me of Kenny".to_string(), "Fuck _____ I'm going to play osu! and suck my own dick".to_string(),
		 "They should really bring back _____, I missed it when it was banned in 2020".to_string(), "Do you ever wake up and ____?".to_string(), "I sit and ponder why ____.".to_string()]}
	}
}

impl Deck {
	pub fn new() -> Deck {
		Deck {
			thrusters: Vec::new(),
			thrustees: Vec::new(),
		}
	}

	fn add_thruster(&mut self, thruster: & std::string::String) {
		self.thrusters.push(thruster.to_string());
		self.sort_thrusters();
	}

	fn add_thrustee(&mut self, thrustee: & std::string::String) {
		self.thrustees.push(thrustee.to_string());
		self.sort_thrustees();
	}

	pub fn sort_thrusters(&mut self) {
		self.thrusters.sort();
	}

	pub fn sort_thrustees(&mut self) {
		self.thrustees.sort();
	}

	pub fn sort(&mut self) {
		self.sort_thrusters();
		self.sort_thrustees();
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

	fn remove_from_thrusters(&mut self) -> String {
		self.thrusters.pop().unwrap()
	}

	fn remove_from_thrustees(&mut self) -> String {
		self.thrustees.pop().unwrap()
	}

	pub fn thrust(index: i32, thruster: &String, thrustee: &String) -> String {
		lazy_static! {
			static ref RE: Regex = Regex::new("[_]+").unwrap();
		}
		let result = RE.replace_all(&thrustee, &(thruster)[..]);
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
	}
	else if input == "4" {
		*running = false;
	}
}
