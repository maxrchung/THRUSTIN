// A WebSocket echo server

extern crate ws;

use ws::listen;
use std::collections::HashMap;

fn main() {
  let mut communication = HashMap::new();
  listen("127.0.0.1:3012", |out| {
      communication.insert(out.token(), out.clone());
      println!("{:?}", communication);
      move |msg| {
        out.send(msg)
      }
  }).unwrap()
} 