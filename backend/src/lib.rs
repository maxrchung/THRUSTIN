#![feature(proc_macro_hygiene, decl_macro)] // Macro stuff to make rocket work
#![feature(vec_remove_item)] // for remove item in vector
#[macro_use]
extern crate rocket; // Macro stuff to make rocket work
#[macro_use]
extern crate lazy_static; //alexgarbage
extern crate regex; //alexgarbage

mod commands;
mod communication;
mod lobby;
mod player;
mod server;
mod thrust;

use communication::FileSystemCommunication;
use communication::WebSocketCommunication;
use server::Server;
use std::cell::RefCell;
use std::rc::Rc;

pub fn run_file_system_server(id: String) {
    let fs_comm = Rc::new(RefCell::new(FileSystemCommunication::new(id)));
    Server::run(fs_comm);
}

pub fn run_web_socket_server() {
    let ws_comm = Rc::new(RefCell::new(WebSocketCommunication::new()));
    Server::run(ws_comm);
}