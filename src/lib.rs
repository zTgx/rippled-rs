#[macro_use]
extern crate lazy_static;
extern crate ws;

use serde_json::{Value};
use serde_json::json;
use crate::protocol::error_codes::*;

lazy_static! {
    // pub static ref DEFAUT_VALUE: Value = {
    //     let mut json = json!({
    //         ERROR: "1",
    //         ERROR_CODE: "000",
    //         ERROR_MESSAGE: "tesSUCCESS",
    //     });
    //
    //     json
    // };

    static ref DEFAUT_VALUE: Value = one();
}

pub fn one() -> Value {
    let json = json!({
        ERROR: "1",
        ERROR_CODE: "000",
        ERROR_MESSAGE: "tesSUCCESS",
    });

    json
}


pub mod rpc;
pub mod crypto;
pub mod net;
pub mod protocol;
pub mod server;
pub mod nodestore;
pub mod core;
pub mod app;
