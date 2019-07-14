use std::thread;
use thrustin;
use thrustin::communication::ChannelCommunication;

// By default don't log so console clutter is reduced
pub fn setup() -> ChannelCommunication {
    let mut server = ChannelCommunication::new(false);
    let mut client = ChannelCommunication::new(false);
    ChannelCommunication::bind(&mut server, &mut client);
    thread::spawn(move || {
        thrustin::run_test_server(server);
    });
    client
}

// This can be specifically called to help debug the debugging hehe
pub fn setup_with_logging() -> ChannelCommunication {
    let mut server = ChannelCommunication::new(true);
    let mut client = ChannelCommunication::new(true);
    ChannelCommunication::bind(&mut server, &mut client);
    thread::spawn(move || {
        thrustin::run_test_server(server);
    });
    client
}

pub fn setup_with_db(db_name: &'static str) -> ChannelCommunication {
    let mut server = ChannelCommunication::new(false);
    let mut client = ChannelCommunication::new(false);
    ChannelCommunication::bind(&mut server, &mut client);
    thread::spawn(move || {
        thrustin::run_test_db_server(server, db_name);
    });
    client
}