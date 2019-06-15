// There are known issues with using fs::remove_dir_all on Windows
// This crate fixes these issues and makes delete more consistent
use remove_dir_all::remove_dir_all;
use rocket::response::NamedFile;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::vec::Vec;
use std::{io, thread};
use ws::{CloseCode, Handler, Handshake, Message, Result, Sender};

pub trait Communication {
    fn start(&self);
    fn continue_running(&self) -> bool;
    fn stop(&self);

    // mut required for updating  FileSystemCommunication
    // WebSocketCommunication doesn't have mutability issue since everything is behind Arc Mutex
    fn read_message(&mut self) -> (u32, String);

    fn send_message(&self, token: &u32, message: &str);
    fn send_messages(&self, token: &u32, messages: &Vec<String>);
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
    running: bool,
    messages: HashMap<u32, Vec<String>>
}

impl ChannelCommunication {
    pub fn new() -> ChannelCommunication {
        let (send, read) = mpsc::channel();
        ChannelCommunication {
            send,
            read,
            to_send: None,
            running: true,
            messages: HashMap::new()
        }
    }

    pub fn bind(left: &mut ChannelCommunication, right: &mut ChannelCommunication) {
        right.to_send = Some(left.send.clone());
        left.to_send = Some(right.send.clone());
    }

    pub fn add_message(&mut self, token: u32, msg: String) {
        if !self.messages.contains_key(&token) {
            self.messages.insert(token, Vec::new());
        }
        let message = self.messages.get_mut(&token).unwrap();
        message.push(msg.clone());
    }

    pub fn read_all(&mut self) -> (u32, String) {
        let mut last_message = (0, String::new());
        // Keep on reading while you can and add messages
        while let Ok((token, msg)) = self.read.try_recv() {
            last_message = (token, msg.clone());
            self.add_message(token.clone(), msg.clone());
        }
        last_message
    }
}

impl Communication for ChannelCommunication {
    fn start(&self) {
    }

    fn continue_running(&self) -> bool {
        return self.running;
    }

    fn stop(&self) {
    }

    fn read_message(&mut self) -> (u32, String) {
        let (token, msg) = self.read.recv().expect("Failed to send message.");
        // stop server if empty message is sent
        if msg == "" {
            self.running = false;
        }

        self.add_message(token.clone(), msg.clone());
        (token, msg)
    }

    fn send_message(&self, token: &u32, message: &str) {
        self.send.send((token.clone(), String::from(message))).expect("Failed to send message.");
    }

    fn send_messages(&self, token: &u32, messages: &Vec<String>) {
        let message = messages.join("<br/>");
        self.send_message(token, &message);
    }
}

#[derive(Debug)]
pub struct FileSystemCommunication {
    id: String,
    uuid: u32,
    client_to_token: HashMap<String, u32>,
    token_to_client: HashMap<u32, String>,
    running: bool,
}

impl FileSystemCommunication {
    pub fn new(id: String) -> FileSystemCommunication {
        FileSystemCommunication {
            id,
            uuid: 1,
            client_to_token: HashMap::new(),
            token_to_client: HashMap::new(),
            running: true,
        }
    }
}

impl Communication for FileSystemCommunication {
    fn start(&self) {
        // I think ideally the whole folder should be wiped, but this can cause conflict with
        // clients sending before the folder is wiped
        if !Path::new(&self.id).exists() {
            fs::create_dir(&self.id).expect("Failed to create server directory");
        }

        // Block for new folder
        while !Path::new(&self.id).exists() {}
    }

    fn continue_running(&self) -> bool {
        return self.running;
    }

    fn stop(&self) {
        while remove_dir_all(&self.id).is_err() {}
    }

    fn read_message(&mut self) -> (u32, String) {
        loop {
            let dir;
            loop {
                match fs::read_dir(&self.id) {
                    Ok(read) => {
                        dir = read;
                        break;
                    }
                    _ => (),
                }
            }

            for entry in dir {
                let entry = entry.expect("Failed to make server entry");
                let path = entry.path();
                let os_file_name = path
                    .file_name()
                    .expect("Failed to get file name for server message");
                let file_name = os_file_name
                    .to_os_string()
                    .into_string()
                    .expect("Failed to convert OS String file name to String");
                if file_name == "end" {
                    self.running = false;
                    return (0, String::new());
                }

                let split: Vec<&str> = file_name.split("_____").collect();
                if split.len() == 2 && split[1] == "server" {
                    let client_name = split[0];

                    if !self.client_to_token.contains_key(client_name) {
                        self.client_to_token
                            .insert(String::from(client_name), self.uuid);
                        self.token_to_client
                            .insert(self.uuid, String::from(client_name));
                        self.uuid = self.uuid + 1;
                    }

                    let client_token = self.client_to_token.get(client_name).unwrap();
                    let mut msg = String::new();

                    // Message may not be done reading yet, so keep on checking
                    while msg.is_empty() {
                        match fs::read_to_string(&path) {
                            Ok(contents) => msg = contents,
                            _ => (),
                        }
                    }

                    fs::remove_file(path).expect("Failed to remove server message file");
                    return (*client_token, msg);
                }
            }
        }
    }

    fn send_message(&self, token: &u32, message: &str) {
        if self.running {
            let client_name = self
                .token_to_client
                .get(token)
                .expect("Unable to get token_to_client");
            let file_name = format!("{}/server_____{}", &self.id, client_name);
            let file_path = Path::new(&file_name);
            // Block if path exists already
            while file_path.exists() {}

            fs::write(file_path, message).expect("Failed to write file to client");
        }
    }

    fn send_messages(&self, token: &u32, messages: &Vec<String>) {
        let message = messages.join("<br/>");
        self.send_message(token, &message);
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
    out: Sender,
    connections: Arc<Mutex<HashMap<u32, Sender>>>,
    send: mpsc::Sender<(u32, String)>,
    uuid: u32,
}

impl Handler for WebSocketListener {
    // Adds new connection to global connections
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        let mut connections_lock = self.connections.lock().unwrap();
        connections_lock.insert(self.uuid, self.out.clone());
        Ok(())
    }

    // Adds message to queue for processing
    fn on_message(&mut self, msg: Message) -> Result<()> {
        self.send.send((self.uuid, msg.to_string()));
        Ok(())
    }

    // Notifies of disconnected client
    fn on_close(&mut self, code: CloseCode, reason: &str) {
        let mut connections_lock = self.connections.lock().unwrap();
        connections_lock.remove(&self.uuid).unwrap();

        match code {
            CloseCode::Normal => self.send.send((self.uuid, format!(".disconnect CloseCode::Normal {}", reason))),
            
            CloseCode::Away => self.send.send(
            (self.uuid, format!(".disconnect CloseCode::Away {}", reason))),
            _ => self.send.send((self.uuid, format!(".disconnect Error {}", reason))),
        };
    }
}

// Main Networking component that public can use
#[derive(Debug)]
pub struct WebSocketCommunication {
    commands: Arc<Mutex<VecDeque<(u32, String)>>>,
    connections: Arc<Mutex<HashMap<u32, Sender>>>,
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
        communication
    }

    // Spawn threads for web server use
    fn spawn(&self) {
        // Only run rocket on development build
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
    fn start(&self) {
        self.spawn();
    }

    fn continue_running(&self) -> bool {
        true
    }

    fn stop(&self) {}

    // Block and read from queue
    fn read_message(&mut self) -> (u32, String) {
        match self.recv.recv() {
            Ok(msg) => return msg,
            Err(_) => {
                println!("Catastrophic failure if this fails probably.");
                (0, "".to_string())
            }
        }
    }

    // Send message to client with the corresponding token
    fn send_message(&self, token: &u32, message: &str) {
        let connections_lock = self.connections.lock().unwrap();
        // Handle case for missing connection - This is possible for disconnects
        if let Some(sender) = connections_lock.get(&token) {
            // Log server response for troubleshooting and FBI-ing
            sender.send(message).unwrap();
        }
    }

    fn send_messages(&self, token: &u32, messages: &Vec<String>) {
        let message = messages.join("<br/>");
        self.send_message(token, &message);
    }
}
