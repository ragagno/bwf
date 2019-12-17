use std::fmt;
use std::io;

use super::Result;
use super::Error;

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

            request.target = if request.buffer_finger + 1 == space_index {
                String::from("/")
            } else if request.buffer[space_index - 1] == b'/' {
                String::from(std::str::from_utf8(&request.buffer[request.buffer_finger..space_index - 1])?)
            } else {
                String::from(std::str::from_utf8(&request.buffer[request.buffer_finger..space_index])?)
            };

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

#[cfg(test)]
mod tests {
    use super::*;

    const REQUEST_GET_HOMEPAGE: &str = "GET / HTTP/1.1\r\n\r\n";
    const REQUEST_DELETE_HOMEPAGE: &str = "DELETE / HTTP/1.1\r\n\r\n";
    const REQUEST_GET_PATH_1: &str = "GET /lorem HTTP/1.1\r\n\r\n";
    const REQUEST_GET_PATH_1_TRAILING: &str = "GET /lorem/ HTTP/1.1\r\n\r\n";
    const REQUEST_GET_PATH_2: &str = "GET /lorem/ipsum HTTP/1.1\r\n\r\n";

    struct StringRead<'a> {
        iter: std::slice::Iter<'a, u8>,
    }

    impl<'a> StringRead<'a> {
        pub fn new(data: &'a str) -> Self {
            Self {
                iter: data.as_bytes().iter(),
            }
        }
    }

    impl<'a> std::io::Read for StringRead<'a> {
        fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
            for i in 0..buffer.len() {
                if let Some(&x) = self.iter.next() {
                    buffer[i] = x;
                } else {
                    return Ok(i);
                }
            }

            return Ok(buffer.len());
        }
    }

    mod assertions {
        #[test]
        fn parse_reader() {
            use super::StringRead;
            use super::Request;

            let mut reader_request_get_homepage = StringRead::new(super::REQUEST_GET_HOMEPAGE);
            let mut reader_request_delete_homepage = StringRead::new(super::REQUEST_DELETE_HOMEPAGE);
            let mut reader_request_get_path_1 = StringRead::new(super::REQUEST_GET_PATH_1);
            let mut reader_request_get_path_1_trailing = StringRead::new(super::REQUEST_GET_PATH_1_TRAILING);
            let mut reader_request_get_path_2 = StringRead::new(super::REQUEST_GET_PATH_2);

            let request_get_homepage = Request::parse(&mut reader_request_get_homepage);
            assert!(request_get_homepage.is_ok());
            if request_get_homepage.is_ok() {
                let request = request_get_homepage.unwrap();

                assert_eq!(super::Method::GET, request.method);
                assert_eq!(String::from("/"), request.target);
            }

            let request_delete_homepage = Request::parse(&mut reader_request_delete_homepage);
            assert!(request_delete_homepage.is_ok());
            if request_delete_homepage.is_ok() {
                let request = request_delete_homepage.unwrap();

                assert_eq!(super::Method::DELETE, request.method);
                assert_eq!(String::from("/"), request.target);
            }

            let request_get_path_1 = Request::parse(&mut reader_request_get_path_1);
            assert!(request_get_path_1.is_ok());
            if request_get_path_1.is_ok() {
                let request = request_get_path_1.unwrap();

                assert_eq!(super::Method::GET, request.method);
                assert_eq!("/lorem", request.target);
            }

            let request_get_path_1_trailing = Request::parse(&mut reader_request_get_path_1_trailing);
            assert!(request_get_path_1_trailing.is_ok());
            if request_get_path_1_trailing.is_ok() {
                let request = request_get_path_1_trailing.unwrap();

                assert_eq!(super::Method::GET, request.method);
                assert_eq!("/lorem", request.target);
            }

            let request_get_path_2 = Request::parse(&mut reader_request_get_path_2);
            assert!(request_get_path_2.is_ok());
            if request_get_path_2.is_ok() {
                let request = request_get_path_2.unwrap();

                assert_eq!(super::Method::GET, request.method);
                assert_eq!("/lorem/ipsum", request.target);
            }
        }
    }

    mod benchmarks {}
}
