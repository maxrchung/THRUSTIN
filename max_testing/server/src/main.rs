#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate ws;

use ws::listen;
use std::collections::HashMap;
use std::io;
use std::thread;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
  NamedFile::open("static/index.html")
}

#[get("/static/<file..>")]
fn file(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
  thread::spawn(|| {
    rocket::ignite().mount("/", routes![index, file]).launch();
  });

  let mut communication = HashMap::new();
  listen("127.0.0.1:3012", |out| {
    communication.insert(out.token(), out.clone());
    println!("{:?}", communication);
    move |msg| {
      out.send(msg)
    }
  }).unwrap();
} 