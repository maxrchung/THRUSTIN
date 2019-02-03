#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate ws;

use ws::{listen, Handler, Sender, Result, Message, CloseCode};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::{env, io, thread, time};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use rocket::response::NamedFile;

struct Server {
  out: Sender,
  commands: Arc:<Mutex<VecDeque<String, ws::util::Token));
}

impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Echo the message back
        self.out.send(msg)
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

  thread::spawn(|| {
    rocket::ignite().mount("/", routes![index, file]).launch();
  });

  let commands = Arc::new(Mutex::new(VecDeque::new()));
  // let mut connections = HashMap::new();

  let commands = Arc::clone(&commands);
  thread::spawn(move || {
    listen("0.0.0.0:3012", |out| {
      // connections.insert(out.token(), out.clone());
      // println!("connections: {:#?}", connections);

      move |msg| {
        let mut commandsHandle = commands.lock().unwrap();
        (*commandsHandle).push_back((msg,
                                     out.token()));
        println!("commands: {:#?}", commandsHandle);
        // out.send(msg)
        Ok(())
      }
    }).unwrap();
  });

  loop {
    println!("commands: {:#?}", "asdf");
    thread::sleep(time::Duration::from_millis(5000));
  }
} 