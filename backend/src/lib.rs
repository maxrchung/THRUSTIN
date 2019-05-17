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

use server::Server;

pub fn run() {
    Server::run();
}