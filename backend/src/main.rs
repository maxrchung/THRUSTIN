extern crate dotenv;
use dotenv::dotenv;
use thrustin;

fn main() {
    dotenv().ok();
    thrustin::run_ws_server();
}
