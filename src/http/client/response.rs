use std::io;
use std::fmt;

use crate::http::Result;
use crate::http::Error;

use crate::http::Status;

const MAX_HEADER_LENGTH: usize = 4096usize;

pub struct Response {
    buffer: [u8; MAX_HEADER_LENGTH],
    buffer_length: usize,
    buffer_finger: usize,

    status: Status,
}

impl Response {
    fn new() -> Self {
        return Self {
            buffer: [0u8; MAX_HEADER_LENGTH],
            buffer_length: 0usize,
            buffer_finger: 0usize,

            status: Status::OK,
        };
    }

    pub fn parse(reader: &mut dyn io::Read) -> Result<Self> {
        let mut response: Self = Self::new();
        response.buffer_length = reader.read(&mut response.buffer)?;

        if response.buffer_finger + 8usize + 1usize >= response.buffer_length {
            return Err(Error::BadResponse);
        }

        if !response.buffer.starts_with(b"HTTP/1.1 ") {
            return Err(Error::BadResponse);
        }

        response.buffer_finger += 8usize + 1usize;

        if response.buffer_finger + 3usize >= response.buffer_length {
            return Err(Error::BadResponse);
        }

        response.status = match Status::from_code_text(&response.buffer[response.buffer_finger..response.buffer_finger + 3usize]) {
            Some(status) => status,
            None => return Err(Error::BadResponse),
        };

        response.buffer_finger += 3usize;

        return Ok(response);
    }

    pub fn get_status(&self) -> &Status {
        return &self.status;
    }
}

impl fmt::Display for Response {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return formatter.write_fmt(format_args!("HTTP/1.1 {}\r\n\r\n", self.status));
    }
}
