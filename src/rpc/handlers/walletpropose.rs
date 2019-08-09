/*
https://xrpl.org/wallet_propose.html

Request Format
An example of the request format:
///WebSocket(with key type)
{
    "command": "wallet_propose",
    "seed": "snoPBrXtMeMyMHUVTgbuqAfg1SUTb",
    "key_type": "secp256k1"
}

///WebSocket(no key type)
{
    "command": "wallet_propose",
    "passphrase": "masterpassphrase"
}

The request can contain the following parameters:
key_type	String	Which elliptic curve to use for this key pair. Valid values are ed25519 and secp256k1 (all lower case). Defaults to secp256k1.
passphrase	String	(Optional) Generate a key pair and address from this seed value. This value can be formatted in hexadecimal , the XRP Ledger's base58 format, RFC-1751 , or as an arbitrary string. Cannot be used with seed or seed_hex.
seed	    String	(Optional) Generate the key pair and address from this seed value in the XRP Ledger's base58-encoded format. Cannot be used with passphrase or seed_hex.
seed_hex	String	(Optional) Generate the key pair and address from this seed value in hexadecimal  format. Cannot be used with passphrase or seed.

//You must provide at most one of the following fields: passphrase, seed, or seed_hex. If you omit all three, rippled uses a random seed.
//Generally, running this command with no parameters is a good way to generate a random seed.

Response Format
An example of a successful response:
{
  "id": 2,
  "status": "success",
  "type": "response",
  "result": {
    "account_id": "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
    "key_type": "secp256k1",
    "master_key": "I IRE BOND BOW TRIO LAID SEAT GOAL HEN IBIS IBIS DARE",
    "master_seed": "snoPBrXtMeMyMHUVTgbuqAfg1SUTb",
    "master_seed_hex": "DEDCE9CE67B451D852FD4E846FCDE31C",
    "public_key": "aBQG8RQAzjs1eTKFEAQXr2gS4utcDiEC9wmi7pfUPTi27VCahwgw",
    "public_key_hex": "0330E7FC9D56BB25D6893BA3F317AE5BCF33B3291BD63DB32654A313222F7FD020"
  }
}
master_seed	    String	This is the private key of the key pair. The master seed from which all other information about this account is derived, in the XRP Ledger's base58 encoded string format. Typically, you use the key in this format to sign transactions.
master_seed_hex	String	The master seed, in hex format. A simple, widely-supported way to represent the private key. Can be used to sign transactions.
master_key	    String	The master seed, in RFC 1751  format. An easier to remember, easier-to-write-down version of the private key. Can be used to sign transactions.
account_id	    String	The Address of the account in the XRP Ledger's base58 format. This is not the public key, but a hash-of-a-hash of it. It also has a checksum so a typo almost certainly results in an invalid address rather than a valid, but different address. This is the primary identifier of an account in the XRP Ledger. You tell people this to get paid, and use it in transactions to indicate who you are and who you're paying, trusting, and so forth. Multi-signing lists also use these to identify other signers.
public_key	    String	The public key of the key pair, in the XRP Ledger's base58 encoded string format. Derived from the master_seed.
public_key_hex	String	This is the public key of the key pair, in hexadecimal. Derived from the master_seed. To validate the signature on a transaction, rippled needs this public key. That's why the format for a signed transaction includes the public key in the SigningPubKey field.
warning	        String	(May be omitted) If the request specified a seed value, this field provides a warning that it may be insecure.

In addition to using it as a regular key pair, you can also use it as a member of a multi-signing list (SignerList).
*/

use serde_json::{Value};
use crate::protocol::error_codes::*;
use crate::crypto::key_type::{KeyType, key_type_from_value};
use crate::{one};
use crate::protocol::jss::*;
use crate::net::rpc_error::RPCError;

pub fn do_propose(params: &Value) -> Result<Value, &'static str> {
    // println!("command: {}", v["command"]);
    // println!("seed: {}", v["seed"]);
    // println!("keyType: {}", v["keytype"]);

    // let command = v["command"];
    // let seed = v["seed"];
    // let key_type = v["key_type"];

    let mut key_code = Some( KeyType::Secp256k1 );
    if params[JSS_KEY_TYPE].is_null() == false {
        if ! params[JSS_KEY_TYPE].is_string() {
            return Error::expected_field_error(JSS_KEY_TYPE, "string");
        }

        key_code = key_type_from_value( &params[JSS_KEY_TYPE] );
        if key_code.is_none() {
            return RPCError::rpc_error(ErrorCodeI::RpcInvalidParams);
        }
    }

    // ripple-lib encodes seed used to generate an Ed25519 wallet in a
    // non-standard way. While we never encode seeds that way, we try
    // to detect such keys to avoid user confusion.
    {
        // if ! params[JSS_PASSPHRASE].is_null() {
        //     seed = RPC::parseRippleLibSeed(params[JSS_PASSPHRASE]);
        // } else if ! params[JSS_SEED].is_null() {
        //     seed = RPC::parseRippleLibSeed(params[JSS_SEED]);
        // }
        //
        // if(seed)
        // {
        //     rippleLibSeed = true;
        //
        //     // If the user *explicitly* requests a key type other than
        //     // Ed25519 we return an error.
        //     if (keyType.value_or(KeyType::ed25519) != KeyType::ed25519)
        //         return rpcError(rpcBAD_SEED);
        //
        //     keyType = KeyType::ed25519;
        // }
    }

    // if ! params[JSS_PASSPHRASE].is_null() ||
    //    ! params[JSS_SEED].is_null() ||
    //    ! params[JSS_SEED_HEX].is_null()
    // {
    //     seed = RPC::getSeedFromRPC(params, err);
    //
    //     if (!seed)
    //         return err;
    // }
    // else
    // {
    //     seed = randomSeed();
    // }

    // auto const publicKey = generateKeyPair (*keyType, *seed).first;
    //
    // Json::Value obj (Json::objectValue);
    //
    // auto const seed1751 = seedAs1751 (*seed);
    // auto const seedHex = strHex (*seed);
    // auto const seedBase58 = toBase58 (*seed);
    //
    // obj[jss::master_seed] = seedBase58;
    // obj[jss::master_seed_hex] = seedHex;
    // obj[jss::master_key] = seed1751;
    // obj[jss::account_id] = toBase58(calcAccountID(publicKey));
    // obj[jss::public_key] = toBase58(TokenType::AccountPublic, publicKey);
    // obj[jss::key_type] = to_string (*keyType);
    // obj[jss::public_key_hex] = strHex (publicKey);

    // If a passphrase was specified, and it was hashed and used as a seed
    // run a quick entropy check and add an appropriate warning, because
    // "brain wallets" can be easily attacked.

    // if (!rippleLibSeed && params.isMember (jss::passphrase))
    // {
    //     auto const passphrase = params[jss::passphrase].asString();
    //
    //     if (passphrase != seed1751 &&
    //         passphrase != seedBase58 &&
    //         passphrase != seedHex)
    //     {
    //         // 80 bits of entropy isn't bad, but it's better to
    //         // err on the side of caution and be conservative.
    //         if (estimate_entropy (passphrase) < 80.0)
    //             obj[jss::warning] =
    //                 "This wallet was generated using a user-supplied "
    //                 "passphrase that has low entropy and is vulnerable "
    //                 "to brute-force attacks.";
    //         else
    //             obj[jss::warning] =
    //                 "This wallet was generated using a user-supplied "
    //                 "passphrase. It may be vulnerable to brute-force "
    //                 "attacks.";
    //     }
    // }

    Ok(one())
}

pub fn wallet_propose(value: &Value) -> Result<Value, &'static str> {
    do_propose(&value)
}
