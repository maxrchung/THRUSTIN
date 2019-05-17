use std::net::TcpListener;
use std::thread;
use thrustin;
use ws::{connect, Handler, Sender, Handshake, Result, Message, CloseCode};

// This client is only used for testing purposes only and be under the tests module
// Normal websocket clients should be connecting and interacting using the React frontend
pub struct TestClient {
    out: Sender,
    last_msg: String,
}

impl Handler for TestClient {
    // `on_open` will be called only after the WebSocket handshake is successful
    // so at this point we know that the connection is ready to send/receive messages.
    // We ignore the `Handshake` for now, but you could also use this method to setup
    // Handler state or reject the connection based on the details of the Request
    // or Response, such as by checking cookies or Auth headers.
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // Now we don't need to call unwrap since `on_open` returns a `Result<()>`.
        // If this call fails, it will only result in this connection disconnecting.
        self.out.send("Hello WebSocket")
    }

    // `on_message` is roughly equivalent to the Handler closure. It takes a `Message`
    // and returns a `Result<()>`.
    fn on_message(&mut self, msg: Message) -> Result<()> {
        self.last_msg = format!("{}", msg);
        self.out.close(CloseCode::Normal)
    }
}

impl TestClient {
    fn send_message(&mut self, msg: &str) {
        self.out.send(msg);
    }

    fn new() {
        connect("ws://127.0.0.1:3012", 
            |out| TestClient { 
                out: out, 
                last_msg: String::new() 
            } 
        ).unwrap();
    }
}

pub fn run_test_server() {
    // Only run server if port is available
    match TcpListener::bind(("0.0.0.0", 3012)) {
        Ok(_) => {
            thread::spawn(|| {
                thrustin::run();
            });
        }
        Err(_) => (),
    };
}