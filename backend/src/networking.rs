use rocket::response::NamedFile;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use std::{io, thread};
use ws::{listen, CloseCode, Handler, Handshake, Message, Result, Sender};

// Returns main site file
#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("../frontend/build/index.html")
}

// Allows access to static folder for grabbing CSS/JavaScript files
#[get("/<file..>")]
fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("../frontend/build/").join(file)).ok()
}

// Specifies handler for processing an incoming websocket connection
struct Connection {
    out: Sender,
    commands: Arc<Mutex<VecDeque<(u32, String)>>>,
    connections: Arc<Mutex<HashMap<u32, Sender>>>,
    uuid: u32,
}

impl Handler for Connection {
    // Adds new connection to global connections
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        let mut connections_lock = self.connections.lock().unwrap();
        connections_lock.insert(self.uuid, self.out.clone());
        Ok(())
    }

    // Adds message to queue for processing
    fn on_message(&mut self, msg: Message) -> Result<()> {
        let mut commands_lock = self.commands.lock().unwrap();
        commands_lock.push_back((self.uuid, msg.to_string()));
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
#[derive(Debug)]
pub struct Networking {
    commands: Arc<Mutex<VecDeque<(u32, String)>>>,
    connections: Arc<Mutex<HashMap<u32, Sender>>>,
    uuid: Arc<Mutex<u32>>,
}

impl Networking {
    // Initialize Networking components
    pub fn init() -> Networking {
        let mut communication = Networking {
            commands: Arc::new(Mutex::new(VecDeque::new())),
            connections: Arc::new(Mutex::new(HashMap::new())),
            // Start at 1 so endless can be 0
            uuid: Arc::new(Mutex::new(1)),
        };
        communication.spawn();
        communication
    }

    // Spawn threads for web server use
    fn spawn(&mut self) {
        // Only run rocket on development build
        // Production will have NGINX return static files rather than rocket
        if cfg!(debug_assertions) {
            // Serve static files for client website
            thread::spawn(|| {
                rocket::ignite().mount("/", routes![index, file]).launch();
            });
        }

        // Websockets
        let commands_clone = Arc::clone(&self.commands);
        let connections_clone = Arc::clone(&self.connections);
        let uuid_clone = Arc::clone(&self.uuid);
        thread::spawn(move || {
            listen("0.0.0.0:3012", |out| Connection {
                out: out,
                commands: commands_clone.clone(),
                connections: connections_clone.clone(),
                uuid: {
                    let mut uuid_lock = uuid_clone.lock().unwrap();
                    let uuid = uuid_lock.clone();
                    // Increment uuid
                    *uuid_lock += 1;
                    uuid
                },
            })
            .unwrap()
        });
    }

    // Block and read from queue
    pub fn read_message(&mut self) -> (u32, String) {
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
    pub fn send_message(&self, token: &u32, message: &str) {
        let connections_lock = self.connections.lock().unwrap();
        let sender = connections_lock.get(&token).unwrap();
        sender.send(message).unwrap();
    }

    pub fn send_messages(&self, token: &u32, messages: Vec<String>) {
        let connections_lock = self.connections.lock().unwrap();
        let sender = connections_lock.get(&token).unwrap();
        let message = messages.join("<br/>");
        sender.send(message).unwrap();
    }
}
