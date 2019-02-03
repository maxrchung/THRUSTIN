#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate ws;

use ws::{listen, Handler, Sender, Handshake, Result, Message, CloseCode, util::Token};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::{env, io, thread, time};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use rocket::response::NamedFile;

struct Server {
  out: Sender,
  commands: Arc<Mutex<VecDeque<(Token, String)>>>,
  connections: Arc<Mutex<HashMap<Token, Sender>>>
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        let mut connections_handle = self.connections.lock().unwrap();
        (*connections_handle).insert(self.out.token(), self.out.clone());
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        let mut commands_handle = self.commands.lock().unwrap();
        (*commands_handle).push_back((self.out.token(), msg.to_string()));
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

fn main() {
  env::set_var("ROCKET_ENV", "staging");

  // Serve files
  thread::spawn(|| {
    rocket::ignite().mount("/", routes![index, file]).launch();
  });

  // Websockets
  let commands = Arc::new(Mutex::new(VecDeque::new()));
  let connections = Arc::new(Mutex::new(HashMap::new()));
  let commands_clone = Arc::clone(&commands);
  let connections_clone = Arc::clone(&connections);
  thread::spawn(move || {
    listen("0.0.0.0:3012", |out| { 
      Server { 
        out: out, 
        commands: commands_clone.clone(),
        connections: connections_clone.clone()
      }
    }).unwrap()
  });

  loop {
    println!("commands: {:#?}", commands);
    println!("connections: {:#?}", connections);
    thread::sleep(time::Duration::from_millis(5000));
  }
}