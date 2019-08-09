extern crate ws;

use std::thread;
use std::thread::sleep;
use ws::{connect, listen, CloseCode, Sender, Handler, Message, Result};

fn main () {
    // Server WebSocket handler
    struct Server {
        out: Sender,
    }

    impl Handler for Server {

        fn on_message(&mut self, msg: Message) -> Result<()> {
            println!("Server got message '{}'. ", msg);
            self.out.send(msg)
        }

        fn on_close(&mut self, code: CloseCode, reason: &str) {
            println!("WebSocket closing for ({:?}) {}", code, reason);
            // self.out.shutdown().unwrap();
        }
    }

    // Server thread
    let server = thread::spawn(move || {
        listen("127.0.0.1:3112", |out| {

            Server { out: out }

        }).unwrap()
    });

    // Give the server a little time to get going
    // sleep(Duration::from_millis(10));

    // Client thread

    let _ = server.join();

    println!("All done.")
}
