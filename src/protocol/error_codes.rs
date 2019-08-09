use serde_json::{Value};
use serde_json::json;

pub const ERROR: &'static str = "error";
pub const ERROR_CODE: &'static str = "error_code";
pub const ERROR_MESSAGE: &'static str = "error_message";

pub enum ErrorCodeI {
    // Malformed command
    RpcInvalidParams        = 31,
    RpcUnknownCommand       = 32,
    RpcNoPfRequest          = 33,
}


// pub struct ErrorInfo {
//     pub code    : error_code_i,
//     pub token   : String,
//     pub message : String,
// }
// impl ErrorInfo {
//     pub fn new(code: error_code, token: String, message: String) -> Self {
//         ErrorInfo {
//             code: code,
//             token: token,
//             message: message,
//         }
//     }
// }

pub struct Error {}
impl Error {
    pub fn expected_field_error(name: &'static str, ftype: &'static str) -> Result<Value, &'static str> {
        Error::make_param_error (&Error::expected_field_message (name, ftype))
    }

    pub fn expected_field_message (name: &'static str, ftype: &'static str) -> String {
        "Invalid field '".to_owned() + name + "', not " + ftype + "."
    }

    pub fn make_param_error(message: &String) -> Result<Value, &'static str> {
        Error::make_error (ErrorCodeI::RpcInvalidParams, message)
    }

    pub fn make_error (code: ErrorCodeI, message: &String) -> Result<Value, &'static str> {
        Error::inject_error (code, message)
    }

    pub fn inject_error (code: ErrorCodeI, message: &String) -> Result<Value, &'static str> {
        // let info: ErrorInfo = ErrorInfo::new (get_error_info (code));

        let json = json!({
            ERROR: "1",
            ERROR_CODE: "000",
            ERROR_MESSAGE: "tesSUCCESS",
        });

        // if let Some(json) = json.as_object_mut() {
        //     json [ERROR] = info.token;
        //     json [ERROR_CODE] = info.code;
        //     json [ERROR_MESSAGE] = serde_json::from_str(message)?;
        // } else {
        //     panic!("inject_error ppppppppppppppppppppppanic!");
        // }

        Ok( json )
    }

    // pub fn get_error_info(code: error_code_i) -> (error_code, String, String) {
    //     if (code <= rpcSUCCESS || code > rpcLAST)
    //         return detail::unknownError;
    //
    //     return detail::sortedErrorInfos.infos[code - 1];
    // }
}
