// Macro stuff to make rocket work
#![feature(proc_macro_hygiene, decl_macro)]
// Macro stuff to make rocket work
#[macro_use] extern crate rocket;

// Get networking module
mod networking;

// Optional to lessen module usage
use networking::Networking;

fn main() {
    // Sets up Networking component
    // You probably don't want to try calling this twice
    let mut communication = Networking::init();

    loop {
        // How to read message
        let (token, message) = communication.read_message();
        println!("Received: {:#?}", (&token, &message));

        // How to send message to a particular client
        communication.send_message(&token, &message);
        println!("Sent echo");
    }
}