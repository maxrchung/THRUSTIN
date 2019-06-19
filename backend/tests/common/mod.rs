use std::thread;
use thrustin;
use thrustin::communication::ChannelCommunication;

pub fn setup() -> ChannelCommunication {
    let mut server = ChannelCommunication::new();
    let mut client = ChannelCommunication::new();
    ChannelCommunication::bind(&mut server, &mut client);
    thread::spawn(move || {
        thrustin::run_channel(server);
    });
    client
}
