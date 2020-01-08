pub mod method;
pub mod status;

pub mod server;
pub mod client;

const MIN_LENGTH_METHOD: usize = 3usize; // "GET", "PUT", ...
const MIN_LENGTH_TARGET: usize = 1usize; // "/"

const LENGTH_PROTOCOL: usize = 8usize; // "HTTP/1.1"
const LENGTH_SPACE: usize = 1usize; // " "
const LENGTH_EOL: usize = 2usize; // "\r\n"

pub use self::method::Method;
pub use self::status::Status;

pub use self::client::Client;
pub use self::server::Server;

use std::fmt;

pub enum Error {
    InternalError(Box<dyn std::error::Error>),

    InvalidPort,
    InvalidAddress,

    InvalidPath,

    UnsupportedProtocolVersion,

    BadRequest,
    BadResponse,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        return Error::InternalError(Box::new(error));
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        return Error::InternalError(Box::new(error));
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Error::InternalError(source) => fmt::Debug::fmt(&**source, formatter),
            Error::InvalidPort => formatter.write_str("Invalid Port"),
            Error::InvalidAddress => formatter.write_str("Invalid Address"),
            Error::InvalidPath => formatter.write_str("Invalid Path"),
            Error::UnsupportedProtocolVersion => formatter.write_str("Unsupported Protocol Version"),
            Error::BadRequest => formatter.write_str("Bad Request"),
            Error::BadResponse => formatter.write_str("Bad Response"),
        };
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Error::InternalError(source) => fmt::Display::fmt(&**source, formatter),
            Error::InvalidPort => formatter.write_str("Invalid Port"),
            Error::InvalidAddress => formatter.write_str("Invalid Address"),
            Error::InvalidPath => formatter.write_str("Invalid Path"),
            Error::UnsupportedProtocolVersion => formatter.write_str("Unsupported Protocol Version"),
            Error::BadRequest => formatter.write_str("Bad Request"),
            Error::BadResponse => formatter.write_str("Bad Response"),
        };
    }
}

impl std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        return match self {
            Error::InternalError(source) => Some(&**source),
            _ => None,
        };
    }
}

pub type Result<T> = std::result::Result<T, Error>;
