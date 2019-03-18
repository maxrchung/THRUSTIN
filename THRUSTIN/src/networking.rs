use rocket::response::NamedFile;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use std::{env, io, thread};
use ws::{listen, util::Token, CloseCode, Handler, Handshake, Message, Result, Sender};

// Returns main site file
#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

// Allows access to static folder for grabbing CSS/JavaScript files
#[get("/static/<file..>")]
fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

// Specifies handler for processing an incoming websocket connection
struct Connection {
    out: Sender,
    commands: Arc<Mutex<VecDeque<(Token, String)>>>,
    connections: Arc<Mutex<HashMap<Token, Sender>>>,
}

impl Handler for Connection {
    // Adds new connection to global connections
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        let mut connections_lock = self.connections.lock().unwrap();
        connections_lock.insert(self.out.token(), self.out.clone());
        Ok(())
    }

    // Adds message to queue for processing
    fn on_message(&mut self, msg: Message) -> Result<()> {
        let mut commands_lock = self.commands.lock().unwrap();
        commands_lock.push_back((self.out.token(), msg.to_string()));
        Ok(())
    }

    // Notifies of disconnected client
    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}

// Main Networking component that public can use
pub struct Networking {
    commands: Arc<Mutex<VecDeque<(Token, String)>>>,
    connections: Arc<Mutex<HashMap<Token, Sender>>>,
}

impl Networking {
    // Initialize Networking components
    pub fn init() -> Networking {
        let mut communication = Networking {
            commands: Arc::new(Mutex::new(VecDeque::new())),
            connections: Arc::new(Mutex::new(HashMap::new())),
        };
        communication.spawn();
        communication
    }

    // Spawn threads for web server use
    fn spawn(&mut self) {
        // Staging allows LAN server to be used
        env::set_var("ROCKET_ENV", "staging");

        // Serve static files for client website
        thread::spawn(|| {
            rocket::ignite().mount("/", routes![index, file]).launch();
        });

        // Websockets
        let commands_clone = Arc::clone(&self.commands);
        let connections_clone = Arc::clone(&self.connections);
        thread::spawn(move || {
            listen("0.0.0.0:3012", |out| Connection {
                out: out,
                commands: commands_clone.clone(),
                connections: connections_clone.clone(),
            })
            .unwrap()
        });
    }

    // Block and read from queue
    pub fn read_message(&mut self) -> (Token, String) {
/*
        match self.commands.lock() {
            Ok(mut lock) => {
                while None == lock.pop_front(){}

                return lock.pop_front().unwrap();
            },
            Err(uh_oh) => {
                
            }
        };
*/
        let mut length = 0;
        while length == 0 {
            let commands_lock = self.commands.lock().unwrap();
            length = commands_lock.len();
        }
        let mut commands_lock = self.commands.lock().unwrap();
        commands_lock.pop_front().unwrap()
    }

    // Send message to client with the corresponding token
    pub fn send_message(&self, token: &Token, message: &str) {
        let connections_lock = self.connections.lock().unwrap();
        let sender = connections_lock.get(&token).unwrap();
        sender.send(message).unwrap();
    }

    pub fn send_messages(&self, token: &Token, messages: Vec<String>) {
        let connections_lock = self.connections.lock().unwrap();
        let sender = connections_lock.get(&token).unwrap();
        let message = messages.join("<br/>");
        sender.send(message).unwrap();
    }
}
