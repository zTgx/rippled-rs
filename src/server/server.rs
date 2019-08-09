extern crate ws;

use std::thread;
use ws::{ listen, CloseCode, Sender, Handler, Message, Result};
// use serde_json::json;
use serde_json::{Value};
use crate::nodestore::rocksdb::{RocksDBFactory};

// Server WebSocket handler
struct Server {
    out: Sender,
}

impl Handler for Server {

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Server got message '{}'. ", msg);
        let x = msg.as_text().unwrap();
        let val:Value = serde_json::from_str(x).unwrap();
        println!("command: {}", val["command"]);
        println!("arg: {}", val["arg"]);

        RocksDBFactory::put(&val["arg"]);

        self.out.send(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("WebSocket closing for ({:?}) {}", code, reason);
        // self.out.shutdown().unwrap();
    }
}

pub struct ServerHandler {}
impl ServerHandler {
    pub fn start() {
        // Server thread
        let server = thread::spawn(move || {
            listen("127.0.0.1:3112", |out| {

                Server { out: out }

            }).unwrap()
        });

        let _ = server.join();

        println!("All done.")
    }
}
