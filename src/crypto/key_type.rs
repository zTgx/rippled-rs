use serde_json::{Value};

#[derive(Debug)]
pub enum KeyType {
    Secp256k1 = 0,
    Ed25519   = 1,
}

pub fn key_type_from_value (s: &Value) -> Option<KeyType> {
    let mut ret = None;
    if let Some(x) = s.as_str() {
        match x {
            "Secp256k1" => {
                ret = Some( KeyType::Secp256k1 );
            },

            "Ed25519" => {
                ret = Some( KeyType::Ed25519 );
            },

            &_ => {
                
            }
        }
    }

    ret
}
