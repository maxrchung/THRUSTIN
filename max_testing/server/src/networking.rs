use ws::{listen, Handler, Sender, Handshake, Result, Message, CloseCode, util::Token};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::{env, io, thread};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use rocket::response::NamedFile;

struct Connection {
    out: Sender,
    commands: Arc<Mutex<VecDeque<(Token, String)>>>,
    connections: Arc<Mutex<HashMap<Token, Sender>>>
}

impl Handler for Connection {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        let mut connections_lock = self.connections.lock().unwrap();
        connections_lock.insert(self.out.token(), self.out.clone());
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        let mut commands_lock = self.commands.lock().unwrap();
        commands_lock.push_back((self.out.token(), msg.to_string()));
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/static/<file..>")]
fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

pub struct Networking {
    commands: Arc<Mutex<VecDeque<(Token, String)>>>,
    connections: Arc<Mutex<HashMap<Token, Sender>>>
}

impl Networking {
    pub fn new() -> Networking {
        Networking {
            commands: Arc::new(Mutex::new(VecDeque::new())),
            connections: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn init(&mut self) {
        env::set_var("ROCKET_ENV", "staging");

        // Serve files
        thread::spawn(|| {
            rocket::ignite().mount("/", routes![index, file]).launch();
        });

        // Websockets
        let commands_clone = Arc::clone(&self.commands);
        let connections_clone = Arc::clone(&self.connections);
        thread::spawn(move || {
            listen("0.0.0.0:3012", |out| { 
            Connection { 
                out: out, 
                commands: commands_clone.clone(),
                connections: connections_clone.clone()
            }
            }).unwrap()
        });
    }

    // Block and read from queue 
    pub fn read_queue(&mut self) -> (Token, String) {
        let mut length = 0;
        while length == 0 {
            // println!("commands: {:#?}", self.commands);
            // println!("connections: {:#?}", self.connections);
            let commands_lock = self.commands.lock().unwrap();
            length = commands_lock.len();
        }

        let mut commands_lock = self.commands.lock().unwrap();
        commands_lock.pop_front().unwrap()
    }

    // Send message to client with the corresponding token
    pub fn send_message(&mut self, token: &Token , message: &str) {
        let connections_lock = self.connections.lock().unwrap();
        let sender = connections_lock.get(&token).unwrap();
        sender.send(message).unwrap();
    }
}