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
		"digging up a mostly rotted corpse, hook it up with wires that connect to a source of electricity, puppeteer it and have people pay to have sex with it".to_string(), "gayme jam".to_string(), "who wrote these".to_string(), "I gurgled a moist egg yolk and vomited from my jowls a chunky curd of phlegm that was dripping with roaches, maggots and mucus.".to_string(),
		"Slackjaws drooling on my dick".to_string(), "jacking off in a public restroom and accidentally cumming on the guy next to me".to_string(), "secretly whacking your meat to hentai in class while the guy next to you tries to sleep".to_string(), "how old I was when my older sister made me her sex slave".to_string(),
		"calling my teammates trash in League of Legends and getting fucking banned for it".to_string(),
		"big ass anime tiddies".to_string(),
		"humongous bubble butt".to_string(),
		"literally so fat you can swallow the earth".to_string(),
		"if I could send you to a prison on the moon, I'd do it so you would not only be so far away from human beings but you'd also die (because of lack of air in space)".to_string(),
		"the feel when I get my body cut up and eaten over the course of the next couple days because I'm into that".to_string(),
		"honestly, I have no idea".to_string(),
		"fuck that pussy in the ass".to_string(),
		"taking these rocket thrusters to the moon".to_string(),
		"3DPD".to_string(),
		"I love it".to_string(),
		"that's exactly what I'm talking about".to_string(),
		"I know what you are talking about doggy".to_string(),
		"absolute negging to the ten thousandth degree of negging".to_string(),
		"literally wet socks".to_string(),
		"figuratively sad".to_string(),
		"fucking myself in the ass".to_string(),
		"buying figures".to_string(),
		"buying keycaps".to_string(),
		"getting baited".to_string(),
		"Good Vibes".to_string(),
		"hitting retards on the head with a bat".to_string(),
		"snapped my spine when I tried sucking my own dick".to_string(),
		"accidentally whacked it to a dude".to_string(),
		"letting out a smelly hot load in my pants".to_string(),
		"going to a slave auction".to_string(),
		"getting a hand injury slamming immigrants in the face".to_string(),
		"a billion dollars".to_string(),
		"smearing shit all over my face".to_string(),
		"bomb vests".to_string(),
		"Peppy".to_string(),
		"shoving my fresh shit up someone's asshole".to_string(),
		"dominating my sister before going to sleep".to_string(),
		"the taste of dog meat".to_string(),
		"blowing up disability centers".to_string(),
		"sharpening the tip of my dick".to_string(),
		"the pain of a thousand slaves".to_string(),
		"the feel when you go take a piss and lift up the seat butt then you want to take a dump so you put down the seat and poop".to_string(),
		"come on and slam".to_string(),
		"welcome to the jam".to_string(),
		],

		 thrustees: 
		 vec!["Every day before Kenny sleeps he really likes _____.".to_string(), "Before heading over to the cotton fields you need to make sure to grab your _____.".to_string(),
		 "I like whacking it to _____ while playing osu!.".to_string(), "Before _____ was banned I really liked it.".to_string(), "_____, that's what I rub my nipples out to.".to_string(),
		 "In class I always make sure to _____ the fucker sitting next to me.".to_string(), "_____ really got fucked in the ass by black people.".to_string(), "I really want to _____ before I die.".to_string(),
		 "There's only one more thing I like more than anime, and it's _____.".to_string(), "Whenever I see _____ it reminds me of Kenny.".to_string(), "Fuck _____ I'm going to play osu! and suck my own dick.".to_string(),
		 "They should really bring back _____, I missed it when it was banned in 2020.".to_string(), "Do you ever wake up and _____?".to_string(), "I sit and ponder why _____.".to_string(), "Whenever I see _____ I can't help but drop my pants".to_string(), "The secret to getting to best _____ is slavery".to_string(),
		 "While I was on the way to school I ran into _____ and started slamming my dick".to_string(), "The only thing worse than getting caught playing osu! is _____".to_string(), "The three steps to getting good at osu are:\n1. Buying a tablet\n2. Getting a good keyboard\n3. _____".to_string(),
		 "_____, the only thing better than programming in Rust".to_string(), "Instead of paying for _____ I would rather slap Kenny on the ass".to_string(), "I am _____ and I want to fucking die".to_string(),
		 "Everytime I see a retard in a wheelchair I just get the urge to _____".to_string(),
		 "_____ keeps me up at night.".to_string(),
		 "What to say to someone who is very sad: _____.".to_string(),
		 "_____ would like a word with me".to_string(),
		 "I'm so full of swag, my swag is full of _____.".to_string(),
		 "That feel when you get wet socks: _____.".to_string(),
		 "I would totally _____ if I had a loaded gun and I was alone.".to_string(),
		 "Sometimes, _____ is the only way to keep me calm.".to_string(),
		 "_____ is an anime Royce would give a 10/10.".to_string(),
		 "If I could only take one thing to an island, I would take _____.".to_string(),
		 "If I'm really mad at someone, I _____.".to_string(),
	 	 "Contrary to popular belief, I really like Kenny. I think he's good at _____".to_string(),
		 "Bruh, can you like not _____.".to_string(),
		 "Whenever I see a black person on the streets I always make sure to _____".to_string(),
		 "The only thing better than the satisfaction of _____ is reading through articles of the Holocaust".to_string(),
		 "My favorite topping for icecream is _____".to_string(),
		 "While pissing on the nearest retard's head I put a _____ on my dick".to_string(),
		 "I expected a local to blow me up when I visited the Middle East but I was pleasantly surprised by _____".to_string(),
		 "The reason my gun clip is empty is because of _____".to_string()
		 ]}
    }
}

impl Deck {
    pub fn new() -> Deck {
        Deck {
            thrusters: Vec::new(),
            thrustees: Vec::new(),
        }
    }

    pub fn add_thruster(&mut self, thruster: &std::string::String) {
        self.thrusters.push(thruster.to_string());
        self.sort_thrusters();
    }

    pub fn add_thrustee(&mut self, thrustee: &std::string::String) {
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
            write_to_socket(&"No thrusters in deck!".to_string());
        } else {
            for thruster in &self.thrusters {
                write_to_socket(&(*thruster).to_string());
            }
        }
    }

    fn view_thrustees(&self) {
        if std::vec::Vec::is_empty(&self.thrustees) {
            write_to_socket(&"No thrustees in deck!".to_string());
        } else {
            for thrustee in &self.thrustees {
                write_to_socket(&(*thrustee).to_string());
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

fn write_to_socket(message: &String) {
    println!("{}", message);
}

fn read_from_socket() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input.trim().to_string()
}

fn handle_input(input: &String, deck: &mut Deck, running: &mut bool) {
    if input == "1" {
        write_to_socket(&String::from("Thrusters:"));
        deck.view_thrusters();
        write_to_socket(&String::from("Thrustees:"));
        deck.view_thrustees();
    } else if input == "2" {
        write_to_socket(&String::from(
            "Which would you like to edit?\n1. thrusters\n2. thrustees\n3. Go back\n",
        ));
        let to_edit = read_from_socket();
        if to_edit == "1" {
            write_to_socket(&String::from(
                "Please enter what thruster you would like to add to your thrusters.",
            ));
            let new_thruster = read_from_socket();
            deck.add_thruster(&new_thruster);
        } else if to_edit == "2" {
            let mut new_thrustee = String::new();
            write_to_socket(&String::from(
                "Please enter what thrustee you would like to add to your thrustees.",
            ));

            while {
                new_thrustee = read_from_socket();
                if !new_thrustee.contains("_") {
                    println!("Not valid thrustee. Please add blank space to allow thrusters to thrust into them.");
                }
                !new_thrustee.contains("_")
            } {}

            deck.add_thrustee(&new_thrustee);
        }
    } else if input == "3" {
    } else if input == "4" {
        *running = false;
    }
}
