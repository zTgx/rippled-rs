use serde_json::{Value};
use crate::protocol::error_codes::*;
use crate::protocol::jss::*;

// VFALCO NOTE Deprecated function
pub struct RPCError {}
impl RPCError {
    pub fn rpc_error (code: ErrorCodeI) -> Result<Value, &'static str>{
        Error::inject_error (code, &"".to_string())
    }

    // VFALCO NOTE Deprecated function
    pub fn is_rpc_error(jv: &Value) -> bool {
        jv.is_object() && ! jv[JSS_ERROR].is_null()
    }
}
