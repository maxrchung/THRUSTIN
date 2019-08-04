use chrono::Local;
use rocket::response::NamedFile;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::{self, Debug, Formatter};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::vec::Vec;
use std::{io, thread};
use ws::{CloseCode, Handler, Handshake, Message, Result};

pub trait Communication {
    // mut required for updating  FileSystemCommunication
    // WebSocketCommunication doesn't have mutability issue since everything is behind Arc Mutex
    fn read_message(&mut self) -> (u32, String);
    // Send message as THRUSTY
    fn send_message(&self, token: &u32, message: &str);
    // Send message from a user
    fn send_message_from(&self, token: &u32, from: &str, message: &str);
    fn send_messages(&self, token: &u32, messages: &Vec<String>);
    fn disconnect(&mut self, token: &u32);

    // Yeah this is the only way I could easily get ip_address, not sure if I want to invest time into some generic route
    fn get_identifier(&self, token: &u32) -> String;
}

impl Debug for dyn Communication {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Debug required for RefCell")
    }
}

pub struct ChannelCommunication {
    send: mpsc::Sender<(u32, String)>,
    read: mpsc::Receiver<(u32, String)>,
    to_send: Option<mpsc::Sender<(u32, String)>>,
    messages: HashMap<u32, Vec<String>>,
    // expected is a counter that is incremented when a message is sent and decremented when a message is read
    // This is to help manage messages that could take a long time to process, e.g. DB inducing commands
    // It isn't perfect, as there are numerous examples of asynchronous messages that can be received, but it helps!!!
    expected: i32,
    can_log: bool,
}

impl ChannelCommunication {
    pub fn new(can_log: bool) -> ChannelCommunication {
        let (send, read) = mpsc::channel();
        ChannelCommunication {
            send,
            read,
            to_send: None,
            messages: HashMap::new(),
            can_log,
            expected: 0,
        }
    }

    pub fn bind(left: &mut ChannelCommunication, right: &mut ChannelCommunication) {
        right.to_send = Some(left.send.clone());
        left.to_send = Some(right.send.clone());
    }

    fn add_message(&mut self, token: u32, msg: String) {
        if !self.messages.contains_key(&token) {
            self.messages.insert(token, Vec::new());
        }
        let message = self.messages.get_mut(&token).unwrap();
        message.push(msg.clone());
    }

    pub fn read_all(&mut self) {
        while self.expected > 0 {
            // Pause for more messages
            thread::sleep(Duration::from_millis(500));

            // Keep on reading while you can and add messages
            while let Ok((token, msg)) = self.read.try_recv() {
                self.add_message(token.clone(), msg.clone());
                if self.can_log {
                    println!("client|{}|{}{}|{}", Local::now(), ">", &token, &msg);
                }
                self.expected -= 1;
            }
        }
    }

    pub fn last(&self, token: u32) -> String {
        let msg = self.messages
            .get(&token)
            .expect("Token does not exist for last")
            .last()
            .expect("Messages does not have last element");
        let json: Value = serde_json::from_str(&*msg)
            .expect("Not valid JSON");
        let msg = json["message"]
            .as_str()
            .expect("Message is not string")
            .to_string();
        msg
    }

    pub fn last_from(&self, token: u32) -> String {
        let msg = self.messages
            .get(&token)
            .expect("Token does not exist for last")
            .last()
            .expect("Messages does not have last element");
        let json: Value = serde_json::from_str(&*msg)
            .expect("Not valid JSON");
        let from = json["from"]
            .as_str()
            .expect("Message is not string")
            .to_string();
        from
    }

    // Since THRUSTS are randomized, we aren't really sure how many THRUSTS we need
    // This will take care of default possibilities...
    pub fn thrust(&mut self, token: u32) {
        self.send(token.clone(), ".t 1");
        self.send(token.clone(), ".t 1 1");
        self.send(token.clone(), ".t 1 1 1");
        self.send(token.clone(), ".t 1 1 1 1");
        self.send(token.clone(), ".t 1 1 1 1 1");
    }

    pub fn send(&mut self, token: u32, msg: &str) {
        self.send_message(&token, msg);
        if self.can_log {
            println!("client|{}|{}{}|{}", Local::now(), &token, ">", &msg);
        }
        // read_all() may be more than send
        // this can occur if messages have been asynchronously sent to the client
        if self.expected < 0 {
            self.expected = 0;
        }
        self.expected += 1;
    }
}

impl Communication for ChannelCommunication {
    fn read_message(&mut self) -> (u32, String) {
        let (token, msg) = self.read.recv().expect("Failed to send message.");
        let json: Value = serde_json::from_str(&*msg)
            .expect("Not valid JSON");
        let msg = json["message"]
            .as_str()
            .expect("Received message is not string")
            .to_string();
        (token, msg)
    }

    fn send_message(&self, token: &u32, message: &str) {
        let msg = json!({
            "from": "THRUSTY",
            "message": message,
        }).to_string();
        self.to_send
            .as_ref()
            .expect("to_send not set")
            .send((token.clone(), msg))
            .expect("Failed to send message.");
    }

    fn send_message_from(&self, token: &u32, from: &str, message: &str) {
        let msg = json!({
            "from": from,
            "message": message,
        }).to_string();
        self.to_send
            .as_ref()
            .expect("to_send not set")
            .send((token.clone(), msg))
            .expect("Failed to send message.");
    }

    fn send_messages(&self, token: &u32, messages: &Vec<String>) {
        let message = messages.join("<br/>");
        self.send_message(token, &message);
    }

    fn get_identifier(&self, token: &u32) -> String {
        token.to_string()
    }

    fn disconnect(&mut self, _token: &u32) {
        self.to_send = None;
    }
}

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
struct WebSocketListener {
    out: ws::Sender,
    connections: Arc<Mutex<HashMap<u32, (String, ws::Sender)>>>,
    send: mpsc::Sender<(u32, String)>,
    uuid: u32,
}

impl Handler for WebSocketListener {
    // Adds new connection to global connections
    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        let mut ip_addr = String::new();
        if let Ok(remote_addr) = handshake.remote_addr() {
            if let Some(remote_addr) = remote_addr {
                ip_addr = remote_addr
            }
        }

        let mut connections_lock = self.connections.lock().unwrap();
        connections_lock.insert(self.uuid, (ip_addr, self.out.clone()));
        Ok(())
    }

    // Adds message to queue for processing
    fn on_message(&mut self, msg: Message) -> Result<()> {
        self.send
            .send((self.uuid, msg.to_string()))
            .expect("Unable to send on message");
        Ok(())
    }

    // Notifies of disconnected client
    fn on_close(&mut self, code: CloseCode, reason: &str) {
        let mut connections_lock = self.connections.lock().unwrap();
        connections_lock.remove(&self.uuid).unwrap();

        match code {
            CloseCode::Normal => self
                .send
                .send((
                    self.uuid,
                    format!(".disconnect CloseCode::Normal {}", reason),
                ))
                .expect("Unable to sent disconnect Normal"),

            CloseCode::Away => self
                .send
                .send((self.uuid, format!(".disconnect CloseCode::Away {}", reason)))
                .expect("Unable to send disconnect Away"),
            _ => self
                .send
                .send((self.uuid, format!(".disconnect Error {}", reason)))
                .expect("Unable to send disconnect Error"),
        };
    }
}

// Main Networking component that public can use
#[derive(Debug)]
pub struct WebSocketCommunication {
    commands: Arc<Mutex<VecDeque<(u32, String)>>>,
    connections: Arc<Mutex<HashMap<u32, (String, ws::Sender)>>>,
    send: mpsc::Sender<(u32, String)>,
    recv: mpsc::Receiver<(u32, String)>,
    uuid: Arc<Mutex<u32>>,
}

impl WebSocketCommunication {
    pub fn new() -> WebSocketCommunication {
        let (sender, receiver) = std::sync::mpsc::channel();
        let communication = WebSocketCommunication {
            commands: Arc::new(Mutex::new(VecDeque::new())),
            connections: Arc::new(Mutex::new(HashMap::new())),
            send: sender,
            recv: receiver,
            // Start at 1 so endless can be 0
            uuid: Arc::new(Mutex::new(1)),
        };
        communication.spawn();
        communication
    }

    // Spawn threads for web server use
    fn spawn(&self) {
        // Only ` rocket on development build
        // Production will have NGINX return static files rather than rocket
        if cfg!(debug_assertions) {
            // Serve static files for client website
            thread::spawn(|| {
                rocket::ignite().mount("/", routes![index, file]).launch();
            });
        }

        // Websockets
        let connections_clone = Arc::clone(&self.connections);
        let send_clone = self.send.clone();
        let uuid_clone = Arc::clone(&self.uuid);
        thread::spawn(move || {
            ws::listen("0.0.0.0:3012", |out| WebSocketListener {
                out: out,
                connections: connections_clone.clone(),
                send: send_clone.clone(),
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
}

impl Communication for WebSocketCommunication {
    // Block and read from queue
    fn read_message(&mut self) -> (u32, String) {
        match self.recv.recv() {
            Ok((token, message)) => {
                let connections_lock = self.connections.lock().unwrap();
                // May disconnect ?
                if let Some((ip_addr, _)) = connections_lock.get(&token) {
                    println!(
                        "{}|{}|{}{}|{}",
                        Local::now(),
                        ip_addr,
                        &token,
                        ">",
                        &message
                    );
                }
                // This block will run if connection has already been disconnected
                else {
                    println!("{}|_|{}{}|{}", Local::now(), &token, ">", &message);
                }
                (token, message)
            }
            Err(_) => {
                println!("{}|_|_|{}|_", Local::now(), ">");
                println!("Catastrophic failure if this fails probably.");
                (0, "".to_string())
            }
        }
    }

    // Send message to client with the corresponding token
    fn send_message(&self, token: &u32, message: &str) {
        let msg = json!({
            "from": "THRUSTY",
            "message": message,
        }).to_string();
        let connections_lock = self.connections.lock().unwrap();
        // Handle case for missing connection - This is possible for disconnects
        if let Some((ip_addr, sender)) = connections_lock.get(&token) {
            // Log server response for troubleshooting and FBI-ing
            sender.send(&*msg).unwrap();
            println!("{}|{}|{}{}|{}", Local::now(), ip_addr, ">", token, msg);
        }
    }

    // Send message with from
    fn send_message_from(&self, token: &u32, from: &str, message: &str) {
        let msg = json!({
            "from": from,
            "message": message,
        }).to_string();
        let connections_lock = self.connections.lock().unwrap();
        // Handle case for missing connection - This is possible for disconnects
        if let Some((ip_addr, sender)) = connections_lock.get(&token) {
            // Log server response for troubleshooting and FBI-ing
            sender.send(&*msg).unwrap();
            println!("{}|{}|{}{}|{}", Local::now(), ip_addr, ">", token, msg);
        }
    }

    fn send_messages(&self, token: &u32, messages: &Vec<String>) {
        let message = messages.join("<br/>");
        self.send_message(token, &message);
    }

    fn get_identifier(&self, token: &u32) -> String {
        let connections_lock = self.connections.lock().unwrap();
        if let Some((ip_addr, _)) = connections_lock.get(&token) {
            ip_addr.clone()
        } else {
            String::new()
        }
    }

    fn disconnect(&mut self, token: &u32) {
        let connections_lock = self.connections.lock().unwrap();
        if let Some((_ip_addr, sender)) = connections_lock.get(&token) {
            // Don't do anything if close succeeds or fails
            match sender.close(CloseCode::Normal) {
                _ => {}
            };
        }
    }
}
