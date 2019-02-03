#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod networking;

fn main() {
    let mut communication = networking::Networking::new();
    communication.init();

    loop {
        let (token, message) = communication.read_queue();
        println!("Received: {:#?}", (&token, &message));

        communication.send_message(&token, &message);
        println!("Sent echo")
    }
}