// use reqwest::Error as reqError;
// use digest_auth::Error as digestError;
// use reqwest::header::ToStrError as toStrError;
// use reqwest::header::InvalidHeaderValue as invalidHeaderValue;
// use xml::reader::Error as xmlReaderError;
// use xml::writer::emitter::EmitterError as xmlEmitterError;
use std::fmt;
use std::string::FromUtf8Error;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum Error {
    Err(String),
    FromUtf8Error(FromUtf8Error),
    // ReqError(reqError),
    // DigestError(digestError),
    // ToStrError(toStrError),
    // InvalidHeaderValue(invalidHeaderValue),
    // XmlReaderError(xmlReaderError),
    // XmlEmitterError(xmlEmitterError),
}


impl Error {
    pub fn new(msg: &str) -> Error {
        Error::Err(msg.to_string())
    }
}

impl std::error::Error for Error {}

/**/
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Err(e) => write!(f,"{}",e),
            _ => write!(f,"{:?}",self),
        }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Error::FromUtf8Error(err)
    }
}
/*
impl From<reqError> for Error {
    fn from(err: reqError) -> Self {
        Error::ReqError(err)
    }
}

impl From<digestError> for Error {
    fn from(err: digestError) -> Self {
        Error::DigestError(err)
    }
}

impl From<toStrError> for Error {
    fn from(err: toStrError) -> Self {
        Error::ToStrError(err)
    }
}

impl From<invalidHeaderValue> for Error {
    fn from(err: invalidHeaderValue) -> Self {
        Error::InvalidHeaderValue(err)
    }
}

impl From<xmlReaderError> for Error {
    fn from(err: xmlReaderError) -> Self {
        Error::XmlReaderError(err)
    }
}

impl From<xmlEmitterError> for Error {
    fn from(err: xmlEmitterError) -> Self {
        Error::XmlEmitterError(err)
    }
}
*/
