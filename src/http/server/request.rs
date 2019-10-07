use std::fmt;
use std::io;

use crate::http::Result;
use crate::http::Error;

use crate::http::Method;

const MAX_HEADER_LENGTH: usize = 4096usize;

pub struct Request {
    buffer: [u8; MAX_HEADER_LENGTH],
    buffer_length: usize,
    buffer_finger: usize,

    method: Method,

    target: String,
}

impl Request {
    fn new() -> Self {
        return Self {
            buffer: [0u8; MAX_HEADER_LENGTH],
            buffer_length: 0usize,
            buffer_finger: 0usize,

            method: Method::GET,

            target: String::from("/"),
        };
    }

    pub fn parse(reader: &mut dyn io::Read) -> Result<Self> {
        let mut request = Self::new();
        request.buffer_length = reader.read(&mut request.buffer)?;

        if request.buffer_finger + 3usize + 1usize >= request.buffer_length {
            return Err(Error::BadRequest);
        }

        {
            let space_index = request.buffer_finger + match request.buffer[request.buffer_finger..].iter().position(|&byte| byte == b' ') {
                Some(index) => index,
                None => return Err(Error::BadRequest),
            };

            request.method = match Method::from_text(&request.buffer[..space_index]) {
                Some(method) => method,
                None => return Err(Error::BadRequest),
            };

            request.buffer_finger = space_index;
        }

        if request.buffer_finger + 1usize >= request.buffer_length {
            return Err(Error::BadRequest);
        }

        request.buffer_finger += 1usize;

        {
            let space_index = request.buffer_finger + request.buffer[request.buffer_finger..].iter().position(|&byte| byte == b' ').unwrap();

            request.target = String::from(std::str::from_utf8(&request.buffer[request.buffer_finger..space_index])?);

            request.buffer_finger = space_index;
        }

        request.buffer_finger += 1usize;

        if request.buffer_finger + 8usize >= request.buffer_length {
            return Err(Error::BadRequest);
        }

        if !request.buffer[request.buffer_finger..].starts_with(b"HTTP/1.1") {
            return Err(Error::BadRequest);
        }

        return Ok(request);
    }

    pub fn get_method(&self) -> &Method {
        return &self.method;
    }
}

impl fmt::Display for Request {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return formatter.write_fmt(format_args!("{} {} HTTP/1.1\r\n\r\n", self.method, self.target));
    }
}
