// use reqwest::blocking::{Client, Response};
// use std::result;
// use std::collections::HashMap;

mod error;
mod request;
mod basicauth;
mod counting;
pub mod macros;

pub use basicauth::BasicAuth;
pub use counting::PeopleCounting;

pub use error::{Error, Result};
pub use request::NotificationElement;
pub use request::EventType;
pub use request::EventMode;
pub use request::HttpAuthenticationMethod;
pub use request::ParameterFormatType;
pub use request::ProtocolType;
pub use request::AddressingFormatType;
pub use request::UploadImagesDataType;



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn first_header_auth() {
        let res = match basicauth::get_initial_www_auth(basicauth::INITIAL_URI, "admin", "Alpha.2.Beta") {
            Ok(v) => v,
            Err(err) => {
                panic!("error: {:?}", err);
            }
        };
        println!("result: {:?}", res);
    }
    
}



