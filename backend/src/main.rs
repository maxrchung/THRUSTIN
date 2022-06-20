#[macro_use]
extern crate log;

extern crate dotenv;
use dotenv::dotenv;
use thrustin;

fn main() {
    dotenv().ok();
    env_logger::init();
    thrustin::run_ws_server();
}
