use crate::lobby::Lobby;
use crate::player::Player;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

///////////
//helpers//
///////////
// Retrieves command from split input
// Lowers input so case is insensitive
fn get_command(input: &Vec<&str>) -> String {
    let com = input[0].to_string().to_lowercase();
    return com;
}

// Helper for making help tables
fn generate_table(commands: Vec<(&str, &str, &str)>) -> String {
    let mut table_html = String::from("<table class=\"table table-sm table-responsive w-auto\">");
    table_html.push_str("<tr>");
    table_html.push_str("<td>Command</td>");
    table_html.push_str("<td>aLiAs</td>");
    table_html.push_str("<td>Help Massage</td>");
    table_html.push_str("</tr>");
    for (command, alias, help) in commands {
        table_html.push_str("<tr>");

        table_html.push_str("<td>");
        table_html.push_str(&command.to_string());
        table_html.push_str("</td>");

        table_html.push_str("<td>");
        table_html.push_str(&alias.to_string());
        table_html.push_str("</td>");

        table_html.push_str("<td>");
        table_html.push_str(&help.to_string());
        table_html.push_str("</td>");

        table_html.push_str("</tr>");
    }
    table_html.push_str("</table>");
    return table_html;
}

fn disconnect(token: u32, players: &mut HashMap<u32, Rc<RefCell<Player>>>) {
    players
        .remove(&token)
        .expect("what the heck how did you disconnect someone who didn't exist bro BIG ASS BUG!!");
}

fn disconnect_from_lobby(
    pl: Rc<RefCell<Player>>,
    players: &mut HashMap<u32, Rc<RefCell<Player>>>,
    lobbies: &mut HashMap<i32, Lobby>,
) {
    disconnect(pl.borrow().token, players);
    Lobby::leave_from_lobby(pl, lobbies);
}

///////////////
//choose name//
///////////////
pub fn choose_name_commands(
    split: Vec<&str>,
    pl: Rc<RefCell<Player>>,
    players: &mut HashMap<u32, Rc<RefCell<Player>>>,
) {
    let com = get_command(&split);
    match &*com {
        ".name" | ".n" => Player::set_name(split, pl, players),

        ".help" | ".h" => list_choose_name_commands(&pl.borrow()),

        ".disconnect" => disconnect(pl.borrow().token, players),

        _ => {
            pl.borrow()
                .send_message("u gotta pick a name bro, try '.name URNAMeHERE'");
        }
    }
}

fn list_choose_name_commands(pl: &Player) {
    pl.send_messages(&vec![
        String::from("Alright so the first phase we've got here is this Choose Name phase. What you're gonna do here is set yourself up with a name that you'll go by. i think this is a great idea because now you have a name and people can call you by your name later when we implement chat. Names give people a sense of identity and belonging. Could you imagine having not a name? What if we reduced you just to some unique number ID, now I think that would be rude, do you not agree? I dont' really remember but I think you can change your name later too so don't worry its just like real life, how we change who we are, the way we speak and walk our gait when we're around other people."),
        generate_table(vec![
            (".help", ".h", "this is it chief"),
            (".name Y0LoSWAG4206669", ".n Y0LoSWAG4206669", "great this will change your name to Y0LoSWAG4206669")
        ])
    ]);
}

////////////////
//out of lobby//
////////////////
pub fn out_of_lobby_commands(
    input: &str,
    split: Vec<&str>,
    pl: Rc<RefCell<Player>>,
    players: &mut HashMap<u32, Rc<RefCell<Player>>>,
    lobby_id: &mut i32,
    lobbies: &mut HashMap<i32, Lobby>,
) {
    let com = get_command(&split);
    match &*com {
        ".help" | ".h" => list_out_commands(&pl.borrow()),

        ".join" | ".j" => Lobby::join_lobby(split, pl, lobbies),

        ".list" | ".l" => Lobby::list_lobby(pl, lobbies),

        ".make" | ".m" => Lobby::make_lobby(pl, lobby_id, lobbies),

        ".name" | ".n" => Player::set_name(split, pl, players),

        ".play" | ".p" => Lobby::join_lobby(vec![".join", "0"], pl, lobbies),

        ".thrust" | ".t" => pl.borrow_mut().handle_thrusteer_commands(&input, &split),

        ".unthrust" | ".u" => pl.borrow_mut().clear_pers_deck(),

        ".who" | ".w" => Lobby::list_all_players(pl, players),

        ".disconnect" => disconnect(pl.borrow().token, players),

        _ => {
            pl.borrow()
                .send_message("Bruh that's an invalid command...!.    try .help");
        }
    }
}

fn list_out_commands(pl: &Player) {
    pl.send_messages(&vec![
        String::from("Alright so now you're in like a waiting zone outside of all the lobbies. Here you can browse lobbies, organize your THRUSTS, and (eventually by milestone 5.3) chat with other people in like general chat. Have fun playing THRUSTIN, brought to you by WAXCHUG&daGWADS."),
        generate_table(vec![
            (".help", ".h", "this is it chief"),
            (".join 1", ".j 1", "Join the lobby with ID 1."),
            (".list", ".l", "Lists info for lobbies that are available"),
            (".make", ".m", "Make a new lobby"),
            (".name xx69SWAGGER911xx", ".n xx69SWAGGER911xx", "If you must, do this to change your name to xx69SWAGGER911xx"),
            (".play", ".p", "Join an endless public lobby with some other big doggos."),
            (".THRUST", ".t", "This will list out your added THRUSTEES and THRUSTERS. (THRUSTERS are THRUSTED into the THRUSTEES's underscores.) Lobbies will combine and use everyone's awesome THRUSTS."),
            (".THRUST \"Some _____ THRUSTEE\" \"Some THRUSTER\"", ".t \"Some _____ THRUSTEE\" \"Some THRUSTER\"", "Add THRUSTS to your wonderful collection. THRUSTS with an underscore will be put into your THRUSTEES otherwise yeah you guessed it they're put into THRUSTERS. Also, remember to encapsulate each THRUST with a quotation."),
            (".UNTHRUST", ".u", "Destroy all your THRUSTS... [*** !!!CAUTION THIS IS IRREVERSIBLE!!! ***]"),
            (".who", ".w", "See who else is swaggin' up in this whack with you"),
        ])
    ]);
}

////////////
//in lobby//
////////////
pub fn in_lobby_commands(
    input: &str,
    split: Vec<&str>,
    pl: Rc<RefCell<Player>>,
    players: &mut HashMap<u32, Rc<RefCell<Player>>>,
    lobbies: &mut HashMap<i32, Lobby>,
) {
    let com = get_command(&split);
    let lobby = { lobbies.get_mut(&pl.borrow().lobby).unwrap() };
    match &*com {
        ".help" | ".h" => list_in_commands(&pl.borrow(), lobby.is_host(pl.borrow().token)),

        ".info" | ".i" => lobby.info(pl),

        ".leave" | ".l" => Lobby::leave_from_lobby(pl, lobbies),

        ".thrust" | ".t" => pl.borrow_mut().handle_thrusteer_commands(&input, &split),

        ".unthrust" | ".u" => pl.borrow_mut().clear_pers_deck(),

        ".who" | ".w" => lobby.list_lobby_players(pl),

        ".chief" | ".c" => lobby.switch_host(split, pl),

        ".house" | ".ho" => lobby.toggle_house(pl),

        ".kick" | ".k" => lobby.kick(split, pl),

        ".password" | ".pa" => lobby.set_password(split, pl),

        ".players" | ".pl" => lobby.player_max(split, pl),

        ".points" | ".po" => lobby.point_max(split, pl),

        ".start" | ".s" => lobby.start_game(pl),

        ".thrustee" | ".tee" => lobby.max_thrustee(split, pl),

        ".thruster" | ".ter" => lobby.max_thruster(split, pl),

        ".disconnect" => disconnect_from_lobby(pl, players, lobbies),

        _ => pl
            .borrow()
            .send_message("Broski that shall be an invalid command. enter .help"),
    }
}

fn list_in_commands(pl: &Player, host: bool) {
    let mut commands = vec![
        (".help", ".h", "this is it chief"),
        (".info", ".i", "I'm pretty sure this will give you some info about the lobby you're in."),
        (".leave", ".l", "We're sorry to see you go..."),
        (".THRUST", ".t", "This will list out your added THRUSTEES and THRUSTERS. (THRUSTERS are THRUSTED into the THRUSTEES's underscores.) Lobbies will combine and use everyone's awesome THRUSTS."),
        (".THRUST \"Some _____ THRUSTEE\" \"Some THRUSTER\"", ".t \"Some _____ THRUSTEE\" \"Some THRUSTER\"", "Add THRUSTS to your wonderful collection. THRUSTS with an underscore will be put into your THRUSTEES otherwise yeah you guessed it they're put into THRUSTERS. Also, remember to encapsulate each THRUST with a quotation."),
        (".UNTHRUST", ".u", "Destroy all your THRUSTS... [*** !!!CAUTION THIS IS IRREVERSIBLE!!! ***]"),
        (".who", ".w", "See who's whacking up this swag lobby with you"),
    ];

    if host {
        commands.append(&mut vec![
            (".chief xxXAzn1994", ".c  xxXAzn1994", "(chief-only) Make xxXAzn1994 the chief of the lobby"),
            (".house", ".ho", "(chief-only) This toggles whether to additionally use our default provided cards - I mean THRUSTS --- Anyways don't worry, your own THRUSTS are always added."),
            (".kick YOLOSWAGGER69", ".k YOLOSWAGGER69", "(chief-only) Someone causing you trouble? Toxicity got you down? Well if you are a chief you can kick YOLOSWAGGER69 out of your lobby using this command."),
            (".password passwordspelledbackwards123420", ".pa passwordspelledbackwards123420", "(chief-only) Sometimes you want to protect your lobby's privacy by setting your lobby's password to passwordspelledbackwards123420"),
            (".players 420", ".pl 420", "(chief-only) Okay, how many players do you want to allow in your lobby? 420?"),
            (".points 1", ".po 1", "(chief-only) Okay, how many points do you want to go to? 1? Don't do 1... cause then the game will end really fast."),
            (".start", ".s", "(chief-only) Yup, naturally as the chief you can start up the game."),
            (".THRUSTEE", ".tee", "(chief-only) Hey there, this command will allow you to configure how many choices a THRUSTEE may choose from."),
            (".THRUSTER", ".ter", "(chief-only) This little command here will allow you to configure how many THRUSTERS one may hold onto at one time."),
        ]);
    }
    let message = &vec![
        String::from("Hey cool so now you're in the lobby and now you've got some more commands. If you're the chief, you've got access to some special options to configure the lobby's game experience. Otherwise, normal non-chiefs, yall can chill out and wait for the game to start."),
        generate_table(commands)
    ];

    pl.send_messages(message);
}

////////////////////
//playing commands//
////////////////////
pub fn playing_commands(
    split: Vec<&str>,
    pl: Rc<RefCell<Player>>,
    players: &mut HashMap<u32, Rc<RefCell<Player>>>,
    lobbies: &mut HashMap<i32, Lobby>,
) {
    let com = get_command(&split);
    let lobby = { lobbies.get_mut(&pl.borrow().lobby).unwrap() };
    match &*com {
        ".help" | ".h" => list_playing_commands(&pl.borrow(), lobby.is_host(pl.borrow().token)),

        ".info" | ".i" => lobby.info(pl),

        ".leave" | ".l" => Lobby::leave_from_lobby(pl, lobbies),

        ".points" | ".p" => lobby.display_points(pl),

        ".thrust" | ".t" => lobby.handle_thrust(split, pl),

        ".kick" | ".k" => lobby.kick(split, pl),

        ".disconnect" => disconnect_from_lobby(pl, players, lobbies),

        _ => pl.borrow().send_message("Bruh that's an invalid command."),
    }
}

fn list_playing_commands(pl: &Player, host: bool) {
    let mut commands = vec![
        (".help", ".h", "this is it chief"),
        (
            ".info",
            ".i",
            "Look at your lobby's settings for some info(rmation).",
        ),
        (".leave", ".l", "Goodbye..."),
        (".points", ".p", "See who's got the points in the lobby."),
        (".THRUST 0", ".t 0", "THRUST your first THRUSTER in baby."),
    ];

    if host {
        commands.append(&mut vec![(
            ".kick BOY_MAN_01",
            ".k BOY_MAN_01",
            "(chief-only) Destroy BOY_MAN_01 from your lobby...",
        )]);
    }

    let message = &vec![
        String::from("Great. Now you're in the phase where you are a THRUSTER. In this state, you can THRUST one of your THRUSTER options into the THRUSTEE. Make sure it's a good one!"),
        generate_table(commands)
    ];

    pl.send_messages(message);
}

////////////
//choosing//
////////////
pub fn choosing_commands(
    split: Vec<&str>,
    pl: Rc<RefCell<Player>>,
    players: &mut HashMap<u32, Rc<RefCell<Player>>>,
    lobbies: &mut HashMap<i32, Lobby>,
) {
    let com = get_command(&split);
    let lobby = { lobbies.get_mut(&pl.borrow().lobby).unwrap() };
    match &*com {
        ".help" | ".h" => list_choosing_commands(&pl.borrow(), lobby.is_host(pl.borrow().token)),

        ".info" | ".i" => lobby.info(pl),

        ".leave" | ".l" => Lobby::leave_from_lobby(pl, lobbies),

        ".points" | ".p" => lobby.display_points(pl),

        ".thrust" | ".t" => lobby.choose(split, pl),

        ".kick" | ".k" => lobby.kick(split, pl),

        ".disconnect" => disconnect_from_lobby(pl, players, lobbies),

        _ => pl
            .borrow()
            .send_message("Brother that is an invalid command."),
    }
}

fn list_choosing_commands(pl: &Player, host: bool) {
    let mut commands = vec![
        (".help", ".h", "this is it chief"),
        (
            ".info",
            ".i",
            "Observe the information data relevant to your lobby's configurations",
        ),
        (".leave", ".l", "This shall be farewell, for now..."),
        (".points", ".p", "See who's got the points in the lobby."),
        (".THRUST 2", ".t 2", "Choose THRUSTEE at index 2 to use."),
    ];

    if host {
        commands.append(&mut vec![(
            ".kick BOY_MAN_01",
            ".k BOY_MAN_01",
            "(chief-only) Destroy BOY_MAN_01 from your lobby...",
        )]);
    }

    let message = &vec![
        String::from("Okay you're a THRUSTEE now. First thing you've gotta do is choose a great THRUSTEE that other THRUSTERS can THRUST into. Make sure it's a juicy one!"),
        generate_table(commands)
    ];

    pl.send_messages(message);
}

////////////
//deciding//
////////////
pub fn deciding_commands(
    split: Vec<&str>,
    pl: Rc<RefCell<Player>>,
    players: &mut HashMap<u32, Rc<RefCell<Player>>>,
    lobbies: &mut HashMap<i32, Lobby>,
) {
    let com = get_command(&split);
    let lobby = { lobbies.get_mut(&pl.borrow().lobby).unwrap() };
    match &*com {
        ".help" | ".h" => list_deciding_commands(&pl.borrow(), lobby.is_host(pl.borrow().token)),

        ".info" | ".i" => lobby.info(pl),

        ".leave" | ".l" => Lobby::leave_from_lobby(pl, lobbies),

        ".points" | ".p" => lobby.display_points(pl),

        ".thrust" | ".t" => lobby.decide(split, pl),

        ".kick" | ".k" => lobby.kick(split, pl),

        ".disconnect" => disconnect_from_lobby(pl, players, lobbies),

        _ => pl.borrow().send_message("Bro! That's an invalid command."),
    }
}

fn list_deciding_commands(pl: &Player, host: bool) {
    let mut commands = vec![
        (".help", ".h", "this is it chief"),
        (".info", ".i", "Browse the inherent settings that have been configured in the presence of this lobby's settings existence."),
        (".leave", ".l", "Farewell friend..."),
        (".points", ".p", "See who's got the points in the lobby."),
        (".THRUST 1", ".t 1", "You've made your decision. THRUSTER at index 1 is the best one."),
    ];

    if host {
        commands.append(&mut vec![(
            ".kick BOY_MAN_01",
            ".k BOY_MAN_01",
            "(chief-only) Destroy BOY_MAN_01 from your lobby...",
        )]);
    }

    let message = &vec![
        String::from("Yeah guy it's time for you to decide on the best THRUSTER. Pick the one that you like the best. Trust your head and your gut. You can do it. I believe in you."),
        generate_table(commands)
    ];

    pl.send_messages(message);
}

///////////
//waiting//
///////////
pub fn waiting_commands(
    split: Vec<&str>,
    pl: Rc<RefCell<Player>>,
    players: &mut HashMap<u32, Rc<RefCell<Player>>>,
    lobbies: &mut HashMap<i32, Lobby>,
) {
    let com = get_command(&split);
    let lobby = { lobbies.get_mut(&pl.borrow().lobby).unwrap() };
    match &*com {
        ".help" | ".h" => list_waiting_commands(&pl.borrow(), lobby.is_host(pl.borrow().token)),

        ".info" | ".i" => lobby.info(pl),

        ".points" | ".p" => lobby.display_points(pl),

        ".leave" | ".l" => Lobby::leave_from_lobby(pl, lobbies),

        ".thrust" | ".t" => pl
            .borrow()
            .send_message("Chill out homeboy... you needa w8 for THRUSTEE to choose..."),

        ".kick" | ".k" => lobby.kick(split, pl),

        ".disconnect" => disconnect_from_lobby(pl, players, lobbies),

        _ => pl
            .borrow()
            .send_message("Bruh... that's an invalid command."),
    }
}

fn list_waiting_commands(pl: &Player, host: bool) {
    let mut commands = vec![
        (".help", ".h", "this is it chief"),
        (".info", ".i", "Wondering... what is the relevancy of the configurations to do with this lobby's present status of being set."),
        (".leave", ".l", "The distance between us shall increase... metaphorically..."),
        (".points", ".p", "See who's got the points in the lobby."),
        (".THRUST", ".t", "This doesn't actually do anything. We're just here to let you know you can't THRUST."),
    ];

    if host {
        commands.append(&mut vec![(
            ".kick SAMPLE_USER_000666",
            ".k SAMPLE_USER_000666",
            "(chief-only) Eliminate SAMPLE_USER_000666 from your lobby...",
        )]);
    }

    let message = &vec![
        String::from("Aite my dude you needa chill and wait for the THRUSTEE to choose a good THRUSTEE to be THRUSTED with."),
        generate_table(commands)
    ];

    pl.send_messages(message);
}
