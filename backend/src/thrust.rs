use rand::seq::SliceRandom;
use rand::thread_rng;
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
            "getting a tablet pen stuck in my dick",
            "wiring up my dick to work as a tablet pen", 
            "a fast forward/backward movement of the hips, usually in sexual activity", 
            "when a guy goes in and out of a girl's vagina really fast getting ready to cum kind of like humping",
            "a sexual act in which the man inserts a cigar into the vagina of his female partner, and then he smokes the cigar while blowing the smoke into his partner's orafice",
            "trust, the hardest thing to gain and the easiest thing to lose", 
            "I actually want to die", 
            "I'm going to fucking kill you", 
            "shitting in a girl's vagina and fucking it",
            "digging up a mostly rotted corpse, hook it up with wires that connect to a source of electricity, puppeteer it and have people pay to have sex with it", 
            "gayme jam", 
            "who wrote these", 
            "I gurgled a moist egg yolk and vomited from my jowls a chunky curd of phlegm that was dripping with roaches, maggots and mucus.",
            "Slackjaws drooling on my dick",
             "jacking off in a public restroom and accidentally cumming on the guy next to me", 
            "secretly whacking your meat to hentai in class while the guy next to you tries to sleep", 
            "how old I was when my older sister made me her sex slave",
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
            "omegaalphahypergigaSUPERLUL, literally the biggest laugh out loud ever",
            "lmao there’s literally no better time to be alive than now",
            "I can’t stop yeetin on dem haters",
            "literally garbage",
            "slurping sips of sewer water",
            "when you’re planning to cook some boiled eggs but drop your eggs too fast into your pot of boiling water and crack the bottom of your eggs and it’s too late now because the water’s too hot to take the eggs out but it’s probably gonna be fine anyway right",
            "aliens lol",
            "a couple shots of soy sauce make me go woozy",
            "boiling some pee pee",
            "drinking some boiled pee pee",
            "we’re just living in a simulation",
            "nothing matters because this is all a dream",
            "I am afraid that I have allowed myself to rust in many ways",
            "watching paint dry",
            "singe off my butt hairs with a blowtorch",
            "feeling like hella comfortable in bed sleeping in on a Sunday morning",
            "drinking soup out of a cup",
            "drinking tea from a bowl",
            "in the morning I spent about 20 minutes drawing eyeballs in sharpie over my body and yeah I cover them up under a long sleeve shirt so no one sees them",
            "taking a bath and dipping your head underwater so you can blow some water bubbles",
            "that’s a yeet from me doggy",
            "your grand daughter joined a terrorist organization so you take a two-day road trip to eliminate her for bringing disrespect to your family only for her to shoot and kill you two minutes later, also she’s hopelessly one-sided in love with the terrorist organization leader",
            "gonna have to reach deep into the meme bank for this one, ah sorry I got nothing",
            "WAX CHUG & DA GWADS",
            "godspeed",
            "extremely smooth brain idea",
            "you snooze you lose brain cells",
            "getting stuck in a vat of shit and having to breathe poop through your nose",
            "sex noises",
            "cow ass",
            "blood is lubrication",
            "they literally implanted a nuclear bomb in my brain that could blow at any time, watch out",
            "like, there’s an alien race coming from the next universe over ready to fire a galaxy disintegrating explosion laser, and I’m like what am I gonna make for dinner tonight",
            "gimme your money if you wanna die",
            "hardening shit into the form of a penis—it’s like clay—then penetrating myself with it",
            "anal beads made out of squishy eyeballs",
            "clone myself so I can rape myself",
            "urine trouble",
            "peepee machine",
            "I’m slappin’ my knees right now",
            "you’re addicted",
            "I find my sister sexually attractive",
            "GunZ: The Duel",
            "I need a drink",
            "",
            "somebody once told me the world is gonna roll me I ain't the sharpest tool in the shed she was looking kind of dumb with her finger and her thumb in the shape of an \"L\" on her forehead well the years start coming and they don't stop coming and they don't stop coming and they don't stop coming and they don't stop coming and they don't stop coming and they don't stop coming and they don't stop coming and they don't stop coming and they don't stop coming and they don't stop coming",
            "I print anime girls out on paper from the library printer so I can feel my girls in real life",
            "Camellia - Exit This Earth's Atomosphere | Roblox death sound remix",
            "just kidding bitch lol",
			"fucking Astolfo in the ass while getting fucked in the ass by Astolfo",
			"homosapiens of high melanin concentration",
			"silverback gorilla",
			"I’m fucking shaking. I don’t know what to do with my hands. They’re flailing. What is going on? I am so enraged, furious, and upset. I can’t believe this. I want to cry but the tears won’t come up. Please tell me this isn’t true?",
			"smelling so bad that flies stick to your body from sweat and grime",
			"Jimmy getting upset while playing League of Legends",
			"Nickin' it out",
			"727",
			"the shigetora prime",
			"penis envy",
			"marrying your body pillow and getting shamed by society",
			"Astolfo",
			"white people",
			"osugame retards",
			"League of Legends: Team Fight Tactics",
			"I’m not gay. I AM straight. I love Titties, Vaginas, and Assholes (of girls). I love to have sex with girls with my penis. Gay people? Fucking awesome, But I am not one of them, nope I’m not",
			"poopoo peepee kaka juice",
			"I have done it, 80 years and not a single nut bust. Thanks to my incredible goal I have obtained an IQ of 156 which I have used to build a pc that deletes system 32 when it loads any type of NSFW. I’m currently in a hospital bed dying of terminal disease, however, this is a victory as in 72 hours I’m expected to die. Wish me luck in this final run",
			"playing Gunz: The Duel until 7AM and getting a sharp pain in your pinkie from Butterflying",
			"being 99.99% genetically similar to monkeys",
			"smoothing my brain out by removing the grooves after applying electric polisher fitted with an 50-grit sandpaper, working with incrementally finer sandpaper until getting down to 3000-grit, and then applying a buffer wheel for the perfect shine",
			"my sister",
			"your sister",
			"trap cosplay",
			"FREEDOM DiVE [FOUR DIMENSIONS]",
			"From the deepest depths of my heart, Fuck you",
			"choking a new top play, getting visibly upset, and throwing your XP-Pen G640 against the wall only to get a new one next week",
			"blankly gaze at my ceiling while rhythmically concaving my head in with a hammer to my favorite osu! song",
			"Fortnite",
			"sexual frustration",
			"Pokemon",
			"cleaning out the tube of my dick",
			"very young children",
			"a cute dog",
			"visibly upset",
			"a large black man with muscular forearms stripping you down, bending you over, and sensually licking your asshole until you climax",
			"intense despair looking at anime girls knowing that they will never be real",
			"playing osu! For 8 hours straight and getting nerve pain from the tip of your fingers to the side of your neck",
			"tripping while running away from the police, getting your dick stuck up your nose and accidentally cumming from nose penetration",
			"THRUSTIN bugs",
			"normies",
			"Okay, retard",
			"the revenge of Paxton",
			"Paxton getting fired",
			"FU🈵CK!!!! PL😵EASE H e 😖LP m EEE!! I’M SPE😜😜RGIN G OU T!!❗!!!!❗😨😫",
			"cock and ball torture",
			"looping your favorite song for 8 hours straight and not liking it anymore",
			"why are we here on this Earth",
			"the inevitable heat death of the universe",
			"shamelessly watching hentai in the school library",
			"getting put on the cringe compilation",
			"put underground without a sound",
			"ARAM",
			"closeted homosexual",
			"slapping your knee so hard that your kneecap pops off, hits someone in the face, and lands you a lifetime sentence in prison",
			"Anime Expo",
			"Jlin",
			"Nick",
			"being extremely racist",
			"sent into a downward-spiral of depression",
			"public execution",
			"erotic asphyxiation",
			"bald and depressed",
			"the entire human race",
			"Hatsune Miku",
			"Cookiezi",
			"fuck off, retard",
			"N-word pass",
			"N-word",
			"Visual Studio Code",
			"I am sexually attracted to men",
			"spending hundreds of hours learning how to Butterfly but then realizing that K-style is useless",
			"Yakub the Big Headed Scientist",
			"the strange, satisfying feeling of twisting a neck",
			"The Boy in the Mirror walking out of the mirror and forcefully pushing you into the mirror where you will be trapped for the rest of your life",
			"divine intervention",
			"massaging your dick with the vibrations of your headphones when playing dubstep on max volume",
			"kpop",
			"being a fucking idiot",
			"I want to beat a man to death",
			"Nanahira",
			"jerking off to the background of an osu! map",
			"yiffing",
			"fursuit",
			"yiffing in your fursuit, getting cum splattered all over it, and never taking it off to be in a constant state of sexual arousal which leads to a grimy buildup of sweat and filth over the course of 3 months",
			"cock",
            "I bought 5 pounds of protein powder and it doesn’t taste very good and I don’t know what to do about it",
            "when your heart feels all warm and fuzzy when you do something nice for someone",
            "fuck off Kenny",
            "Kenny is an awesome person",
            "Kenny is an awesome person said no one ever",
            "that’s such a dumb idea I have RSI in my brain",
            "cutting your wrist too hard and end up chopping your hand off",
            "attach jumper cables to my nipples and shock myself to the next galaxy",
            "HDHRDT",
            "don’t say that shit man",
            "it be like that sometimes",
            "sixty nine",
            "end the gays",
            "day by day I’m so gay",
            "it’s ok cause I’m gay",
            "Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny Kenny",
            "yes",
            "no",
            "maybe",
            "i dunno man",
			"splitting yourself in two with a cleaver which results in your body & consciousness to become two separate beings",
			"achieving dependency from one's own consciousness and living life in 3rd person (Maybe this is the true identity of The Boy in the Mirror... The man staring back is the one controlling your body, and you are merely a simulation...)",
			"the seething rage of a thousand burning suns",
			"making an audible thump on your desk in voice call",
			"i hate you with every fibre of my being",
			"mad and bad",
			"you are literally a piece of shit",
			"anime is not real and it will never be real",
			"ruining your life by playing osu!",
			"hoping that you will be reborn into an anime when you die",
			"Anteatery",
			"going to sleep and hoping that you dont wake up",
			"being uncomfortable around dark-skinned people",
			"no, I am not okay",
			"God is not real but Yakub is real",
			"Matrix",
			"Terry Davis",
			"TempleOS",
			"beanie",
			"looking slackjawed while playing an osu! map, drool dripping on your keyboard and short circuiting it",
			"I literally do not care",
			"that's so pathetic I feel so bad for you",
			"buying nutrient-enriched dirt from locals who dug it up from a mountain far away for 5 dollars a bag, mixing it in water and kneading it into a dirt cookie which is dried in the sun for 24 hours, and feeding it to the villages' hungry kids",
			"hajimemashou",
			"spending your youth designing a realistic anime girl AI, retiring, and spending the rest of your life married to it",
			"sticking your dick in a USB port",
			"frying your brain by connecting it to your computer through your ear and trying to upload your consciousness onto Google docs",
			"meeting Yakub the Scientist in your dreams",
			"eliminating the white race",
			"mountain of testosterone and muscle",
			"feminine dick",
			"gay rights",
			"Yakub the Scientist, a man of rippling muscle ",
			"Jacob Graves",
			"Chest Flattener",
			"lolis",
			"getting seriously hard when you glide your hands over a loli's flat chest",
			"panic!",
			"Sean rage",
			"Cool!",
			"Wow!",
			"too cool for school",
			"cumming while watching love live",
			"edit the values",
			"licking a young boy's nipples",
			"Malcolm X and Muhammad Ali believed in Yakub",
			"the black man blaming white man for being evil even though Yakub, a black man, created the white race 6,000 years ago"
			"achieving dependency from one's own consciousness and living life in 3rd person (Maybe this is the true identity of The Boy in the Mirror... The man staring back is the one controlling your body, you are merely a simulation...)",
            "_____",
            "chillin’ on the beach with my boyfriend sippin’ on some peñis colada",
            "god that’s disgusting",
            "Î̶̹̻͎͉̻̻̌̓̽͛̓̚͠͝ ̶̠̾w̸̝̤̠̦͒͜a̴͖̳͙͎̲̳̠̗̔̋̊͊̈́̈̇͋n̸̬̞̻͕̄̅̇ţ̶̡̨̬̗̗̦͙̀̉̅͒͐̓ ̶̡̛̱̳̩̟͙͇̥̯͗̂̍̓̽́̍̕t̶̹̒́͆́̇ọ̵͎̰̑̏͌́̄́͊̌ͅ ̷̻͉̣̼̪͉͖̑̔͂͛̈́͆͜͝͠ͅd̸͚̰͈͂͆͐͝͝i̷͙̜̞̠͆̒̉̔̈́̃̈́͂͘͝e̵͚̦̲͚͆̒",
            "ｖａｐｏｒｗａｖｅ",
            "ｍａｋｅｉｔｓｔｏｐ",
            "gun",
            "neanderthal",
            "DNB remix",
            "I can’t go outside because I’m afraid of interacting with human beings",
            "getting thrown into the trash can but finding it kind of nice in there",
            "when I was in kindergarten, I got in trouble for kissing a girl and had to sit outside on a bench for half an hour",
            "Mount Everest",
            "AIDS",
            "cancer",
            "whipping slaves",
            "micromeme",
            "Fake News",
            "slurping aborted feces",
            "blending stillborn babies",
            "god isn’t real",
            "watching your parents fuck",
            "drinking coffee from a bowl",
            "sticks and stones will break my bones and words will always hurt me",
            "hook me up to a sperm tank and pump me full of cum",
            "manifest destiny",
            "covering the mouth so the screams don't alert the neighbors",
            "The Irvine Company",
            "ah shit, here we go again",
            "living in a scripted 9 to 5 comedy",
            "The Mona Lisa"
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
            "Whenever I see _____ it reminds me of Kenny.", 
            "Fuck _____, I'm going to play osu! and suck my own dick.",
            "They should really bring back _____, I missed it when it was banned in 2020.", 
            "Do you ever wake up and _____?", 
            "I sit and ponder why _____.", 
            "Whenever I see _____ I can't help but drop my pants.", 
            "The secret to getting the best _____ is slavery.",
            "While I was on the way to school I ran into _____ and started slamming my dick.", 
            "The only thing worse than getting caught playing osu! is _____.", 
            "<br/>The three steps to getting good at osu are:<br/>1). Buying a tablet<br/>2). Getting a good keyboard<br/>3). _____.",
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
            "<br/>One reason why League of Legends is the greatest game in the world: _____.<br/>One reason why League of Legends is not the greatest game in the world and has destroyed my life: _____.",
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
            "1969 meme: _____, 2019 meme: _____, 2069 meme: _____",
            "Guilty Crown is _____.",
            "Guilty Crown is literally garbage. _____",
            "I like a good cuck or two, but then there’s _____.",
            "A perfect night out with _____ and _____.",
            "Rust is a multi-paradigm _____ focused on safety, especially safe concurrency.",
            "Rust is a reddish- or yellowish-brown flaky coating of iron oxide that is formed on iron or steel by oxidation, especially in the presence of _____.",
            "<br/>Nobody:<br/>Me: _____",
            "Ah yeah, it feels good to _____.",
            "I know what you are saying, but what are you _____?",
            "_____ decided to _____.",
            "Brilliant, here’s my PhD on _____.",
            "When I look in the mirror, I see _____.",
            "Nobody’s safe when _____.",
            "DOCTORS HATE HIM, with this one WEIRD but simple TRICK you could solve ALL OF THE WORLD PEACE IN THE WORLD: _____.",
            "The dark ages began because of _____.",
            "The coup started because _____.",
            "Chicken, broccoli, and _____, a suitable dish for each and every occasion.",
            "Nothing beats a weekend getaway to _____.",
            "When you go to a fancy restaurant and eat hella expensive _____.",
            "I would probably consider _____ a mild discomfort.",
            "With a little black magic you can easily turn _____ into _____.",
            "Everyone makes mistakes. You can’t help but _____ sometimes.",
            "After touching the forbidden _____, you’ve been cursed by the _____.",
            "With next-generation food science technology, they can now make artificially synthesized _____ to eat.",
            "Nobody would complain if _____ just fell from the sky.",
            "_____. That’s a banger.",
            "I can’t stop crying. _____.",
            "I love dicks but not when they’re _____.",
            "After playtesting THRUSTIN for some time, I’ve noticed that there are quite a few THRUSTERS and THRUSTEES that are targeted towards Kenny. Because this game can be quite derogatory in nature, Kenny has unfairly been the butt of many offensive jokes. This THRUSTEE is here to acknowledge that Kenny is a good person. Kenny is a good person. _____.",
            "A baby is born when a _____ loves a _____ very much.",
            "Hold my beer, _____.",
            "-__-",
            "Xxx_AzNKniGhT1337_xxX",
			"Whenever I see _____ I can't help but get very, very, upset.",
			"God, I really love the smell of _____.",
			 "<br/>_____ ✅<br/>_____ ❌",
			"Meeting _____ IRL for the first time ever and then _____.",
			"♿_____♿",
			"_____ _____ _____ _____ _____",
			"Whenever I see _____ I can't help but give my knees a quick slap.",
			"This can't be happening. Please, don't _____ me. I'm on my knees begging, please don't do this to me.",
			"Love it or hate it, you can’t help but agree with ____.",
			"Being alone with ____ makes me extremely uncomfortable.",
			"I’m sorry. I couldn’t help but cum all over _____. Please forgive me.",
			"God please help me I just _____. I'm definitely going to hell please God allow me to repent by _____.",
			"The look normies give me after _____ sends me into a fit of rage before going into a downward-spiral of _____.",
			"Whenever I _____ it makes me unreasonably hard.",
			"I have a very primal, animalistic urge to impregnate _____.",
			"🐒 🐒 🐒 THE MONKEYS HAVE INVADED!!!! WHAT DO YOU DO??? : ____.",
			"_____ smells so fucking bad🤮🤮🤮💩💩🤮 Oh my GOD 💩 I can’t stand it what the F UCK!!!! 💩🤮🤮🤮🤮",
			"_____ looks so delicious! It would be perfect with a sprinkle of salt and pepper, yum!",
			"_____ is one of the best things in the world. It warms my heart and brings me joy.",
			"_____ is my worst enemy. I would quit my job, sell my house, leave my wife and children just to track it down and eliminate it.",
			"Laying in bed all night and wondering if it is biologically possible to impregnate _____.",
			"The best thing to use to concave your head in is _____.",
			"Before _____ comes to invade, I’ll make sure to ______ one last time.",
			"I couldn’t believe my eyes when I saw the source code of _____. It was a mess.",
			"Scientists took a cross-section of _____ and studied it under the microscope. They have discovered that it was filled with _____.",
			"I bet you think you’re so fucking cool. Just wait until _____ comes. Then I’ll be the one laughing.",
			"You fool. You absolute buffoon. I can't believe you actually fell for _____!",
			"Walking into a pit of _____ and coming out splattered with cum.",
			"Okay, I might be gay, but you are _____. Fuck you.",
			"You freeze yourself in a Cryogenic chamber and come out in the year 69420, only to be disappointed when met with _____.",
			"I've been wrongly put into a psych ward for _____. No matter how much _____ I do I cannot convince the doctors that I am sane.",
			"_____ is too overpowered in THRUSTIN. No matter what I do I cannot win when my opponents get that THRUST. Please come out with a balance patch, I am literally begging on my knees and sobbing.",
			"After taking _____ on a long walk off a short pier, I kissed them on the lips",
			"You still play ARAM, that outdated game? Come on man, play _____ with us.",
			"I get an odd sense of sexual satisfaction by staring at _____ for long periods of time.",
			"The date is 2072. The world is on fire from global warming but here I am _____.",
			"No one knows, but I secretly enjoy _____. My life will be over if anyone finds out about it.",
			"I am sexually attracted to _____.",
			"Only the mighty fist of a powerful _____ can stop _____ from destroying the world.",
			"After the human race came in contact with aliens, the aliens found out about our _____ which sent us into a 1000-year long war.",
			"I accidentally yelled out \"_____\" in public which got me beat up and tossed in the dumpster.",
			"Imagine _____, OMEGALUL.",
			"The words I always wanted someone to say to me is _____.",
			"<br/>“Damnit......!”<br/>In that case— How about this—?!<br/>Kirito changed his attack pattern and activated «_____», the<br/>highest level Dual Blades skill. Like the tips of an enveloping<br/>corona, his swords sent twenty-seven consecutive attacks<br/>towards Kayaba.",
			"_____ _____? Are you kidding me?",
			"My supervisor got really angry when he saw me _____ at work.",
			"Recently I've been having a little too much _____ in life. I think I need some _____ to change it up.",
			"I got rich by selling _____.",
			"After meeting up with Dean Herbert, the creator of osu!, I asked him what he had planned. Dean said he would be adding _____ in the future.",
			"My balls are so red, bruised, and swollen it feels so good 🥊🥊🥊🎊🥴 Please _____ it more.",
			"🤡_____🤡",
            "I’m getting cucked by _____.",
            "There’s no easy way to say it, but _____.",
            "_____ and _____.",
            "They recently discovered some cave paintings depicting _____.",
            "@( ^ _ ^ )@",
            "This is yet another Kenny DLC THRUSTEE to combat the rampant hate unfairly targeted against him. You’re a good kid, man, keep on chillin’. Haters gonna hate, but you’re above that shit. _____.",
            "m🙈nkEy see, M🙉nkEy Do.. m🙊nKey _____.",
            "E-girls (electronic girls) only want one thing and it’s disgusting: _____.",
            "That’s pretty good, but can you _____?",
            "Everyone knows _____ is better than _____.",
            "I’m a worthless waste of space. All I want to do is waste my time, playing stupid fucking games, dumbing myself down, hopelessly floundering around while the future comes every second closer. I’m never gonna achieve anything in my life. I don’t have a single ounce of dedication, or motivation, or of anything god dammit, in my body. Yeah, I wanna do better. Yeah, I want to be in better shape. Yeah, I want to be a fucking astronaut for all I could say. But I don’t actually care. You don’t fucking care. You never put in the effort, not even a speck of hard work. All you do is complain, complain everyday, wasting away every day crawling through your worthless life. Fuck, I want to kill myself, but god damn am I a coward. _",
            "_____ went to the _____ in order to _____.",
            "A once in a life-time never before seen epic masterpiece you-don’t-want-to-miss-this showdown: _____ vs _____.",
			"I was born with the rare disease, Priapism, which gives victims constant, painful erections. But once I saw _____, I developed erectile dysfunction.",
			"<br/>👁️‍&nbsp;&nbsp;&nbsp;&nbsp👁️‍<br/>&nbsp;&nbsp;&nbsp👃<br/>&nbsp;&nbsp;&nbsp👄<br/>Greetings human. It is I, the Epicman. What words do you have for me? _____.",
			"Every fucking day my upstair neighbors make noise by _____. It's driving me insane.",
			"Alright, fucker. Hand over the _____ before I fucking kill you.",
			"Is it just me or does anyone get ____ and ____, mix it in with their fresh shit and mould it into a shit-ball that they cure, bake & harden, and keep on their desk for good luck?",
			"<br/>Nick: _____<br/>Everyone else: Ok retard.",
			"Bro stop _____ it's really pissing me off man.",
			"10 years ago I made a wrong turn in life by _____. After that, I was never the same.",
			"One day Yakub the Scientist blessed me with his presence. He told me the world is in danger and the only way to save it is to _____.",
			"The one thing Jacob wants in life: _____",
			"_____ is so fucking unintelligble and incomprehensible I literally can't understand it.",
			"<br/>pub fn thrust(thruster: &String, thrustee: &String) -> String {<br/>&nbsp;&nbsp;&nbsp;&nbsplazy＿static! {<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbspstatic ref RE: Regex = Regex::new(\"_____\").unwrap();<br/>&nbsp;&nbsp;&nbsp;&nbsp}<br/>&nbsp;&nbsp&nbsp;&nbsplet result = RE.replace(&thrustee, &(thruster)[..]);<br/>&nbsp;&nbsp;&nbsp;&nbspresult.to＿string()<br/>}",
            "What’s your favorite type of music? _____.",
            "When life gives you lemons, you make _____.",
            "Yo momma so fat, she _____.",
            "Yo momma so _____, she _____.",
            "Guys, let’s take a 30 second break to stretch and break the monotony of our miserable lives.",
            "Professional _____ here, let me take care of it.",
            "I'm not going to say this again. For the last time, _____.",
            "<br/>&nbsp;&nbsp;.*/#%&&@@@@@@@@@@@@/*,,*/##*<br/>&nbsp;(@&@@@@@@@@@@@@@@@(.&nbsp;&nbsp;.,*,,,*(#&/<br/>&nbsp;%@@@@@@@@@&&@@&&/&nbsp;/&@&&&%#(***(%@&.<br/>&nbsp;(@*&nbsp;&nbsp;&nbsp;..,,..%@%(%&%&&@%###(/(%&&&.<br/>&nbsp;*&&@@@@@@@@&@@&(*#@%/*,.&nbsp;.**/((%&%&&..<br/>&nbsp;*@@@@@@@@(%@&*/(.(@%&nbsp;&nbsp;&nbsp;.*/(/(&&%%&%*<br/>..@@@&(*&nbsp;&nbsp;&nbsp;&nbsp;./@@**##(//(##(/((%&&%%&%*.<br/>&/#@@//%,..,/(@@#*(((######(#&&%%&(*.<br/>%%%@@@@@@@@@@@@@%((((((((((&&%&%&/(#,<br/>%%%%@@@@@@@@@@@@((((((((&&%&%&(((##<br/>%%%%&@@@@@@@@@@&&&(((((/#&&%&%&&((((###<br/>%%%%%@@@@@@@@@@&&&@%//(#&&&%%&&((((##((<br/>%%%%%&@@@@@@@@@@@@(,,,%&%&&&%(((((##(((<br/>%%%%%%@@@@@@@@&..(**/#&&&/(((((#(((((<br/>%%%%%%%@@@@@@@@@(/%#/#((#%#((((((((((((<br/>%%%%%%%%@@@@@@@#(/(%#######((((((((((#/<br/>%%%%%%%%%&@@@@#(%%/,,,,,*//#####(((#(..<br/>%%%%%%%%%%%&@&///(#%%##/*/########/..,*<br/>%%%%%%%%%%%%%&@%/**,,,/#&&&%((#(**//*,*<br/><br/>Shrackner<br/>_____, 2011<br/>Digital<br/>69 in × 69 in",
            "<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;.*************.<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;,*********************,<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;***************************.<br/>&nbsp;&nbsp;&nbsp;&nbsp;*******************************<br/>&nbsp;&nbsp;,*********************************.<br/>&nbsp;.***********************************<br/>&nbsp;*************************************/<br/>***************(.******(&nbsp;**************<br/>***************#&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;(***************<br/>***************(&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;****************<br/>***************&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;****************<br/>.***********.,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;,***************<br/>&nbsp;#*********&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;******.*************.<br/>&nbsp;&nbsp;&nbsp;.***(#&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;.**********************,<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;************************<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;***********************,<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;**********************<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;********************,<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;/***************/<br/><br/>WAX CHUG & da GWADS<br/>_____, 2019<br/>Meme on canvas<br/>420 in × 420 in",
            "<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;____<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;o8%8888,<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;o88%8888888.<br/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;8'-&nbsp;&nbsp;&nbsp;&nbsp;-:8888b<br/>&nbsp;&nbsp;&nbsp;&nbsp;8'&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;8888<br/>&nbsp;&nbsp;&nbsp;d8.-=.&nbsp;,==-.:888b<br/>&nbsp;&nbsp;&nbsp;>8&nbsp;`~`&nbsp;:`~'&nbsp;d8888<br/>&nbsp;&nbsp;&nbsp;88&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;,88888<br/>&nbsp;&nbsp;&nbsp;88b.&nbsp;`-~&nbsp;&nbsp;':88888<br/>&nbsp;&nbsp;&nbsp;888b&nbsp;~==~&nbsp;.:88888<br/>&nbsp;&nbsp;&nbsp;88888o--:':::8888<br/>&nbsp;&nbsp;&nbsp;`88888|&nbsp;:::'&nbsp;8888b<br/>&nbsp;&nbsp;&nbsp;8888^^'&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;8888b<br/>&nbsp;&nbsp;d888&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;,%888b.<br/>&nbsp;d88%&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;%%%8--'-.<br/>/88:.--&nbsp;,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-%-'&nbsp;---&nbsp;&nbsp;-<br/>&nbsp;&nbsp;&nbsp;&nbsp;'''::===..-'&nbsp;&nbsp;&nbsp;=&nbsp;&nbsp;--.<br/><br/>Leonardo da Vinci<br/>_____, 1503-1511<br/>Oil on canvas<br/>30¼ in × 21 in<br/>",
            "Guys, let’s take a 30 second break to stretch and break the monotony of our miserable lives.<br/>1…<br/>2…<br/>3…<br/>4…<br/>5…<br/>6…<br/>7…<br/>8…<br/>9…<br/>10…<br/>11…<br/>12…<br/>13…<br/>14…<br/>15…<br/>16…<br/>17…<br/>18…<br/>19…<br/>20…<br/>21…<br/>22…<br/>23…<br/>24…<br/>25…<br/>26…<br/>27…<br/>28…<br/>29…<br/>30…<br/>Okay awesome! Back to _____.",
            r#"<br/>&nbsp;○&nbsp;&nbsp;&nbsp;●&nbsp;<br/>/|\&nbsp;/|\<br/>/&nbsp;\&nbsp;/&nbsp;\<br/>&nbsp;_____"#,
			"<br/>. ..... .. . .....  ...  .... . .. .... .............:=.................. .......
			<br/>...... .. . . .. . . .    . ...  .,?I7I7777II7$I+??=$$I... . ...... . . .  .... 
			<br/>......... ... .    .      ...7++IIIIIOIIII?II7777?ZI$$$$..... . ..    .         
			<br/>......... ......   . .  ..Z~?IIIIII7=+?I?I++++7+OZ77$8D8...+=~,... .. .  .  ... 
			<br/>...... ...... ....    .I:????I????+++I+I++++?+7Z77$7I7II$7DO7$?+.     .         
			<br/>...... .....I~ ... ..+:=~====+II7~+++=+++++I=ZI777777IIII$7$ZI7==.    .     . . 
			<br/>...........=I77,...?,,~:~~=+=?Z,==++++++++I+7II$??II+???I7$7$$ZO===...... ......
			<br/>...... ....:8777=:,,,,,:~=~=I?,~=+=++++++?+IIIII7?+?~=??IIII$77$Z===............
			<br/>.... .. ..,+IOO$Z=,,,,,~~~++I:::========?=IIII$I++7???????II+7I7Z=+==.... ......
			<br/>......  ...=$$O888,,,,=~~+++,::========?=?I?=I+I=+?++????II$?7I=?O7=~...........
			<br/>....... ..I$88DO8D+8$=~7$ND8Z$==~~====+=I?++?Z=+=+=+++???I?$?Z777$?I+?..........
			<br/>...... . =$$Z8NND7$D~:O8DDNNNNZ:~======O?==??$===+=+=+=+?I?7?$I777Z$?=~.Z++=. ..
			<br/>..=+.....?77ZO88N88ND8888888D8:~:~~~===?===++I===+======???7?7II77Z7$I=NDD+=.  .
			<br/>....+777$77O888N$$NDN88888DND:~:::::~~?~====?+===========??I?7I777777IIDNDD=....
			<br/>....ZII7DI88DNN==+NNOO888DN8:::::::~:I=~===+=?=++===+====+?I?III7I777II?DDD,....
			<br/>...7I8888NNZ~==ON8888888O~:~::::~~:=::~==O=7=?+========+??????II77777+DDD.....
			<br/>...ZO8888NO:===NND$8888D:=~:::::~:+~::~:?+=$=?==========+?????I?IZ7II??7O=....
			<br/>.....I888O=~,~==D88D$$DDD8=:~~=~~=~:=:::::I:=?~===========I?I??????Z?I?+=D?+$. .
			<br/>..... =,=:=,:~Z8888=I$DDN$========~+=~~=:7I:~=?=?~========7+I??????$?I??+7ND888O
			<br/>.....?,::~,+~77O8$I~I=NN8=~=======~7==+=?II:~~I=?~=~~~====?=I??????7?II?++NDD888
			<br/>. ..:,,,~=,Z?777$I~=++,+==~~~~~~~~~~~~~~I?I~=+?I+I~~~~=~=~=~???????I?II?===DDDDZ
			<br/>....,,,~+,,~,+7I$:~O+=,:~~~~=~~~~~?~~?~????==++7=?++~~+~~~~?++?=??+??II?===DNN..
			<br/>..,,,:~=+,~,~=+$,?7I+:,~~~~~~~~~~~+:=:=+~++~====???+++=~=I~O+==+?+++?I+===Z=D...
			<br/>..,,::=+,,~~==I,Z77+:,~~~~~+~~~~~~~~+:+~:=+~=====++I=++++?=$+===I+++?I?===I+7...
			<br/>,,,::==?,~,==7,7777~:,~~~~=~~~~~~?=7,+~~=+7I???+===?I===++Z7+7III++??$7I?+I+++ .
			<br/>,:::==+,::~=II+7I7+:,~~~~~=~=~~~ZZI:?:::::+I~+=======+==I=Z7?III$I?I?$I77=+++?~.
			<br/>::+=~~~,~:+=++77??,:,~~~~=~~:~~~7I:?==~::~~?~+~+==~=====?=?7?IIII7III7777+=,.:I?
			<br/>:~~~~+,,:~=++ZI7?:~,~~~~~::~:::D8~DNN?O$O~==~I=~========+??7???8?7I7I77$I+=+. ..
			<br/>=~~~=:,~:~=?$7+77+,,::::=:+:::$8?$Z8ZODNINND7I+:~~+~====+??7?+?$?II7I77$7+=?....
			<br/>~~=~$,,~~=+$I7?7::,,:::=~=~~~~+7~~O77$$D?~~OI:+:::~~=~~I?~+I==$7+7?7?II77+=+. ..
			<br/>~~~I,::~~=I?7I+:?,,~:~~~====:?~7~~$+=+I8,:~:~~++::::~:~==:+7~=7++$I$?II7I+==.  .
			<br/>=~?7::~~=?I?7??=~,=~====?==~+~:7~::=::?,,,:::~~++::::::7::+I~=?=~$7Z??III+$=....
			<br/>~+?,~:~~=87?Z?:?,=~====+===Z:~:7:~~~~:7:::::::~=+=::::=:::~?:$++~$7???III=8==. .
			<br/>=?+:+,~=?$77Z~?~:~=====?==I=:~:?:~~~~~~~:::::::=~+:~::=INNO+~++I~$$??II$7=I=I...
			<br/>+~:~=:~+Z?77?I?,~=====?==?~~:~:=::~~~~~~~~~::::::~=~:::ODDD+8DZ7=$Z?77$$7=.==...
			<br/>=?~I~~=+???I?7,===+==?=++?~~::~::::~~~~~~~::::::::::~:8Z88O~~N?II7+?$7777+.+=...
			<br/>.,+?~~+$?I77?,===+==?7+?+?=~::?=~::::::::::::::::::::~==I$I~~DIZ7O+7III?$?.?++..
			<br/>:,?=~=++=77?7~?++=+?I=?$+?===:$?~:::::::::::::::::::::?:7~:~~=7I7=?IIIIII=.++I..
			<br/>7,=~=~?~I77Z=??==??Z=Z77++=++~?:~::::::::::::::::::::~~~::::+=?=$?I?III$+~.,?? .
			<br/>I:~==~+?7II+?I=??Z$=O7$+=+=++==,:::::::::::::::::::::~~~~::+???7=???I??7?~..??..
			<br/>=:=~+~+??$?7+IZZZ7?Z77O?=++=++?,:::::::::=::::::::::::~~~~7?=?+~7I+??+?++:..7I..
			<br/>~=?..+I++?++7IIZ7ZZZ777?I=+==+8~,:::::::=~+::::::::::::::???=~+I?++?+?7I+,..II..
			<br/>~+..~+?===?7?+7$$77O77I$7==+~=$7=,:::::=:~=:::::::::::::=+?=+=I??++++=+?+,..:I..
			<br/>=,::?+?++I7?II+77$7Z?$$I7====~?777:~~~+::::?+::~:::::::~++==I?I+===+=ZI.+~. .I .
			<br/>..,,?=+?$7I++??I$$$+?7?==7++?~=7I77?:=::::::~==~=::::~I=++?==??=====?=,.:+ ..? .
			<br/>I.,~+~$8==+..,:O$$.??+=+:.+++?=$II$,:7::::::::~~:~~O+===+==+++=====,,+...=...? .
			<br/>$..??7===O==Z?,:,~=$+~.+,Z+?+I=+II$::~:::::::::?++??I+====?.======I.=.. ..=..+ .
			<br/>7..?=+=I,,,,,~7,+,==7ZZZZZO+III=II$::,:::::::::I+++?++=+=?.=+====+ +:       :.  
			<br/>,.++=~?.,,,,,,:7,?,+=+$ZZZZZ+III?I7,::::::::::~++++.+++=?.====I=+..?...    .:...
			<br/>,.???7.....,,,,:?,?,????7ZZZ8+7IIII,::::::::::$++, .?++?.===?I+?=.+.       ,.   "
        ];
        Deck {
            thrustees: thrustees.iter().map(ToString::to_string).collect(),
            thrusters: thrusters.iter().map(ToString::to_string).collect(),
        }
    }
}

impl Deck {
    pub fn add_thruster(&mut self, thruster: &str) {
        self.thrusters.push(thruster.to_string());
    }

    pub fn add_thrustee(&mut self, thrustee: &str) {
        self.thrustees.push(thrustee.to_string());
    }

    pub fn clear(&mut self) {
        self.thrusters.clear();
        self.thrustees.clear();
    }

    pub fn count_underscores(thrustee: &str) -> i32 {
        lazy_static! {
            static ref REGEX_UNDERSCORE: Regex = Regex::new("_+").unwrap();
        }
        let mut count = 0;
        for _ in REGEX_UNDERSCORE.find_iter(thrustee) {
            count += 1;
        }
        return count;
    }

    pub fn find_thrusts(thrust: &str) -> Vec<String> {
        lazy_static! {
            static ref REGEX_THRUST: Regex = Regex::new("\"(.*?)\"").unwrap();
        }
        let mut thrusts = Vec::new();
        for capture in REGEX_THRUST.captures_iter(thrust) {
            thrusts.push(String::from(&capture[1]));
        }
        thrusts
    }

    pub fn new() -> Deck {
        Deck {
            thrusters: Vec::new(),
            thrustees: Vec::new(),
        }
    }

    pub fn sort(&mut self) {
        self.thrusters.sort();
        self.thrustees.sort();
    }

    pub fn shuffle_deck(&mut self) {
        self.thrusters.shuffle(&mut thread_rng());
        self.thrustees.shuffle(&mut thread_rng());
    }

    pub fn shuffle_thrusters(&mut self) {
        self.thrusters.shuffle(&mut thread_rng());
    }

    pub fn shuffle_thrustees(&mut self) {
        self.thrustees.shuffle(&mut thread_rng());
    }

    pub fn thrust(thruster: &String, thrustee: &String) -> String {
        lazy_static! {
            static ref RE: Regex = Regex::new("[_]+").unwrap();
        }
        let result = RE.replace(&thrustee, &(thruster)[..]);
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_underscore() {
        assert_eq!(1, Deck::count_underscores("_"));
    }

    #[test]
    fn multiple_underscores() {
        assert_eq!(3, Deck::count_underscores("Hey what is up my _ __ ___"));
    }

    #[test]
    fn find_empty_thrust() {
        assert_eq!(vec![""], Deck::find_thrusts(".t \"\""));
    }

    #[test]
    fn find_single_thrust() {
        assert_eq!(vec!["YOL0 SW@G"], Deck::find_thrusts(".t \"YOL0 SW@G\""));
    }

    #[test]
    fn find_multiple_thrusts() {
        assert_eq!(
            vec!["", "Swag", "Now __ is it fam"],
            Deck::find_thrusts(".t \"\" \"Swag\" \"Now __ is it fam\"")
        );
    }
}
