extern crate rippled_rs;
use rippled_rs::rpc::handlers::walletpropose::wallet_propose;
use serde_json::{Value};
use rippled_rs::server::server::*;

fn main() {
    let data = r#"
    {
        "command": "wallet_propose",
        "seed": "snoPBrXtMeMyMHUVTgbuqAfg1SUTb",
        "key_type": "secp256k1"
    }"#;

    let v: Value = serde_json::from_str(data).unwrap();
    wallet_propose(&v).unwrap();
    println!("rippled's signing algorithms in Rust.");

    ServerHandler::start();
}
