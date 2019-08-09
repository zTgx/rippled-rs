extern crate ws;

use std::thread;
use std::thread::sleep;
use ws::{connect, listen, CloseCode, Sender, Handler, Message, Result};
use serde_json::{Value};
use serde_json::json;

fn main() {
    let client = thread::spawn(move || {

        connect("ws://127.0.0.1:3112", |out| {

            let json = json!({
                "command": "doInfo"
            });


            out.send(json.to_string()).unwrap();

            move |msg| {
                println!("Client got message '{}'. ", msg);
                out.close(CloseCode::Normal)
            }

        }).unwrap()

    });

    let _ = client.join();

    println!("All done.")

}
