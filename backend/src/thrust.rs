use regex::Regex;

#[derive(Clone, Debug)]
pub struct Deck {
    pub thrusters: Vec<String>,
    pub thrustees: Vec<String>,
}

impl Default for Deck {
    fn default() -> Deck {
        let thrusters = vec![
            "Kenny", 
            "Brenn", 
            "Jimmy", 
            "Max", 
            "Royce", 
            "Alex", 
            "homosexuality", 
            "heterosexuality",
            "Gmaul", 
            "Runescape", 
            "dick", 
            "big juicy cock", 
            "osu!", 
            "peppy points", 
            "black people", 
            "tablet pen",
            "ass fucking", 
            "Omega Sector", 
            "rape and pillage", 
            "tribal slaughter", 
            "cat ears", 
            "degeneracy", 
            "anime",
            "hentai", 
            "big anime titty", 
            "gaming", 
            "3/5ths Compromise", 
            "cotton fields", 
            "slavery", 
            "furry",
            "99 pure str", 
            "full combo", 
            "niseboi", 
            "Kenny getting fucked in the ass", 
            "racism", 
            "Riku", 
            "Rust",
            "Rustacean", 
            "gay sex", 
            "trap", 
            "storyboard", 
            "sucking dick at osu!", 
            "Jews", 
            "Minecraft",
            "hot anime sex", 
            "lolicon", 
            "swag42069", 
            "rock hard dick", 
            "anal sex while playing osu!", 
            "cheesy dick",
            "Dream Solister", 
            "osu! UCI", 
            "McDonalds", 
            "Burger King", 
            "McSad", 
            "Pippins", 
            "bro I wanna really die", 
            "moist", 
            "while still in my panties, I vomited from my jowls a chunky curd of phlegm", 
            "pulsating purple cock", 
            "gurgling a moist egg yolk while I blogged about it", 
            "thanks for the protein",
            "keep my cock in your mouth until we get home", 
            "he told me that you were going to adore and cherish this big black cock", 
            "cum box", 
            "I'm harder than steel",
            "I'm being raped by a monster", 
            "eating cum from my ass", 
            "BDSM", 
            "the salty drips of precum that are rubbed over my tongue", 
            "I roll my hips, his slick, hot crown, teasing at my star, until finally I feel it loosen enough",
            "naked woman", 
            "young boy", 
            "aborted fetus", 
            "fucking a pregnant woman and her fetus at the same time", 
            "The Boy in the Mirror", 
            "getting a tablet pen stick in my dick",
            "wiring up my dick to work as a tablet pen", 
            "a fast forward/backward movement of the hips, usually in sexual activity", 
            "when a guy goes in and out of a girl's vagina really fast getting ready to cum kind of like humping",
            "a sexual act in which the man inserts a cigar into the vagina of his female partner, and then he smokes the cigar while blowing the smoke into his partner's orafice",
            "trust, the hardest thing to gain and the easiest thing to lose", 
            "I actually want to die", "I'm going to fucking kill you", 
            "shitting in a girl's vagina and fucking it",
            "digging up a mostly rotted corpse, hook it up with wires that connect to a source of electricity, puppeteer it and have people pay to have sex with it", 
            "gayme jam", 
            "who wrote these", 
            "I gurgled a moist egg yolk and vomited from my jowls a chunky curd of phlegm that was dripping with roaches, maggots and mucus.",
            "Slackjaws drooling on my dick", "jacking off in a public restroom and accidentally cumming on the guy next to me", 
            "secretly whacking your meat to hentai in class while the guy next to you tries to sleep", "how old I was when my older sister made me her sex slave",
            "calling my teammates trash in League of Legends and getting fucking banned for it",
            "big ass anime tiddies",
            "humongous bubble butt",
            "literally so fat you can swallow the earth",
            "if I could send you to a prison on the moon, I'd do it so you would not only be so far away from human beings but you'd also die (because of lack of air in space)",
            "the feel when I get my body cut up and eaten over the course of the next couple days because I'm into that",
            "honestly, I have no idea",
            "fuck that pussy in the ass",
            "taking these rocket THRUSTERS to the moon",
            "3DPD",
            "I love it",
            "that's exactly what I'm talking about",
            "I know what you are talking about doggy",
            "absolute negging to the ten thousandth degree of negging",
            "literally wet socks",
            "figuratively sad",
            "fucking myself in the ass",
            "buying figures",
            "buying keycaps",
            "getting baited",
            "Good Vibes",
            "hitting retards on the head with a bat",
            "snapped my spine when I tried sucking my own dick",
            "accidentally whacked it to a dude",
            "letting out a smelly hot load in my pants",
            "going to a slave auction",
            "getting a hand injury slamming immigrants in the face",
            "a billion dollars",
            "smearing shit all over my face",
            "bomb vests",
            "Peppy",
            "shoving my fresh shit up someone's asshole",
            "dominating my sister before going to sleep",
            "the taste of dog meat",
            "blowing up disability centers",
            "sharpening the tip of my dick",
            "the pain of a thousand slaves",
            "the feel when you go take a piss and lift up the seat butt then you want to take a dump so you put down the seat and poop",
            "come on and slam",
            "welcome to the jam",
            "giga braining",
            "guys I'm sperging out",
            "dude, fucking OMEGALUL",
            "okay, I have trust issues",
            "Actually, It's Not",
            "it's lit",
            "you may reach the United States 24 hour suicide hotline at 1-800-273-TALK (8255)",
            "stuffing some cookie crumbs up my butthole",
            "RSI, dear god RSI ithurts please i canteven typ anymo e",
            "y01042069sw4g0u1",
            "meme",
            "memes",
            "dying lol (epic yeet!)",
            "ass flesh",
            "water the plants",
            "the feel when you're drinking boba but suck the straw too hard and choke on boba",
            "vanilla ice cream, literally the most vanilla, plain, boring thing in the world",
            "NTR",
            "yeah I like getting cucked",
            "taking 5 dicks up the ass",
            "taking 5 dicks up the butt",
            "hindsight is 2020",
            "holy shit it’s 2019 i’m gay as fk",
            "the THRUSTIN dustin is a home-made double ended butthole with a moustache attached to it so two gay dudes can bone it while gazing deeply into each others eyes and cum on each others dick head",
            "THRUSTIN",
            "shut up",
            "camellia",
            "in a hot minute",
        ];

        let thrustees = vec![
            "Every day before Kenny sleeps he really likes _____.",
            "Before heading over to the cotton fields you need to make sure to grab your _____.",
            "I like whacking it to _____ while playing osu!.", 
            "Before _____ was banned I really liked it.", 
            "_____, that's what I rub my nipples out to.",
            "In class I always make sure to _____ the fucker sitting next to me.", 
            "_____ really got fucked in the ass by black people.", 
            "I really want to _____ before I die.",
            "There's only one more thing I like more than anime, and it's _____.", 
            "Whenever I see _____ it reminds me of Kenny.", "Fuck _____ I'm going to play osu! and suck my own dick.",
            "They should really bring back _____, I missed it when it was banned in 2020.", 
            "Do you ever wake up and _____?", "I sit and ponder why _____.", 
            "Whenever I see _____ I can't help but drop my pants.", 
            "The secret to getting to best _____ is slavery.",
            "While I was on the way to school I ran into _____ and started slamming my dick.", 
            "The only thing worse than getting caught playing osu! is _____.", 
            "The three steps to getting good at osu are:\n1. Buying a tablet\n2. Getting a good keyboard\n3. _____.",
            "_____, the only thing better than programming in Rust.", 
            "Instead of paying for _____ I would rather slap Kenny on the ass.", 
            "I am _____ and I want to fucking die.",
            "Everytime I see a retard in a wheelchair I just get the urge to _____.",
            "_____ keeps me up at night.",
            "What to say to someone who is very sad: _____.",
            "_____ would like a word with me",
            "I'm so full of swag, my swag is full of _____.",
            "That feel when you get wet socks: _____.",
            "I would totally _____ if I had a loaded gun and I was alone.",
            "Sometimes, _____ is the only way to keep me calm.",
            "_____ is an anime Royce would give a 10/10.",
            "If I could only take one thing to an island, I would take _____.",
            "If I'm really mad at someone, I _____.",
            "Contrary to popular belief, I really like Kenny. I think he's good at _____.",
            "Bruh, can you like not _____.",
            "Whenever I see a black person on the streets I always make sure to _____.",
            "The only thing better than the satisfaction of _____ is reading through articles of the Holocaust.",
            "My favorite topping for icecream is _____.",
            "While pissing on the nearest retard's head I put a _____ on my dick.",
            "I expected a local to blow me up when I visited the Middle East but I was pleasantly surprised by _____.",
            "The reason my gun clip is empty is because of _____.",
            "I make my eggs typically by _____.",
            "One reason why League of Legends is the greatest game in the world: _____.\nOne reason why League of Legends is not the greatest game in the world and has destroyed my life: _____.",
            "The feel when you get a _____ at 4:20 in the morning: _____.",
            "Chef is cooking some sweet _____.",
            "Chief ain't _____...",
            "One time I _____ my dick, lol!",
            "Hi, I'm a doctor with a PhD, and I'd like to tell you about the _____ epidemic.",
            "Children are _____ in the world, why are you wasting your time _____?",
            "Dead _____ give(s) me the weirdest boner?",
            "When I die, make sure to _____",
            "I like _____, but I love _____.",
            "When I _____, I tend to _____.",
            "If _____ happens, you know shit’s about to _____.",
            "Monstrata will rank Dualive in 2020. _____.",
            "_____.",
            "_____ _____.",
            "_____? Yeah that’ll do it, chief.",
            "Chief called, he said that _____.",
            "Hello ^^! Please describe me in one THRUST 😊: _____.",
            "Just _____ 4Head.",
            "Something I would never say, like, ever: _____.",
            "Hey man, sorry I missed your message, I was cooking up some  _____.",
            "I’ll tell you about _____ in 10 years.",
            "Yeah, I’ll be writing about _____ in my diary for the next 6 months.",
        ];
        Deck {
            thrustees: thrustees.iter().map(ToString::to_string).collect(),
            thrusters: thrusters.iter().map(ToString::to_string).collect(),
        }
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
            write_to_socket(&"No THRUSTERS in deck!".to_string());
        } else {
            for thruster in &self.thrusters {
                write_to_socket(&(*thruster).to_string());
            }
        }
    }

    fn view_thrustees(&self) {
        if std::vec::Vec::is_empty(&self.thrustees) {
            write_to_socket(&"No THRUSTEES in deck!".to_string());
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
        let result = RE.replace(&thrustee, &(thruster)[..]);
        result.to_string()
    }

    pub fn count_underscore(thrustee: &String) -> i32 {
        lazy_static! {
            static ref RE: Regex = Regex::new("([_]+)").unwrap();
        }
        let mut count: i32 = 0;
        for _ in RE.find_iter(thrustee) {
            count += 1;
        }
        return count;
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
