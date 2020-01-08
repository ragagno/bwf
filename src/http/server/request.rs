use std::fmt;
use std::io;

use super::Result;
use super::Error;

use super::super::{MIN_LENGTH_METHOD, LENGTH_SPACE, MIN_LENGTH_TARGET, LENGTH_PROTOCOL, LENGTH_EOL};

use crate::http::Method;

const MAX_LENGTH_HEADER: usize = 4096usize;

const MIN_LENGTH_REQUEST_LINE_FULL: usize = MIN_LENGTH_METHOD + LENGTH_SPACE + MIN_LENGTH_TARGET + LENGTH_SPACE + LENGTH_PROTOCOL + LENGTH_EOL + LENGTH_EOL;
// "GET / HTTP/1.1\r\n\r\n"
const MIN_LENGTH_REQUEST_LINE_SLICED_1: usize = MIN_LENGTH_TARGET + LENGTH_SPACE + LENGTH_PROTOCOL + LENGTH_EOL + LENGTH_EOL;
// "/ HTTP/1.1\r\n\r\n"
const MIN_LENGTH_REQUEST_LINE_SLICED_2: usize = LENGTH_PROTOCOL + LENGTH_EOL + LENGTH_EOL;
// "HTTP/1.1\r\n\r\n"
const MIN_LENGTH_REQUEST_LINE_SLICED_3: usize = LENGTH_EOL; // "\r\n"

pub struct Request {
    buffer: [u8; MAX_LENGTH_HEADER],
    buffer_length: usize,
    buffer_finger: usize,

    method: Method,

    target: String,
}

impl Request {
    pub fn parse(reader: &mut dyn io::Read) -> Result<Self> {
        let mut request = Self {
            buffer: [0u8; MAX_LENGTH_HEADER],
            buffer_length: 0usize,
            buffer_finger: 0usize,

            method: Method::GET,

            target: String::from("/"),
        };

        request.buffer_length = reader.read(&mut request.buffer)?;

        if MIN_LENGTH_REQUEST_LINE_FULL > request.buffer_length {
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

            request.buffer_finger = space_index + LENGTH_SPACE;
        }

        if request.buffer_finger + MIN_LENGTH_REQUEST_LINE_SLICED_1 > request.buffer_length {
            return Err(Error::BadRequest);
        }

        {
            let space_index = request.buffer_finger + match request.buffer[request.buffer_finger..].iter().position(|&byte| byte == b' ') {
                Some(index) => index,
                None => return Err(Error::BadRequest),
            };

            request.target = if request.buffer[request.buffer_finger] == b'/' {
                if request.buffer_finger + 1usize == space_index {
                    String::from("/")
                } else if request.buffer[space_index - 1usize] == b'/' {
                    String::from(std::str::from_utf8(&request.buffer[request.buffer_finger..space_index - 1usize])?)
                } else {
                    String::from(std::str::from_utf8(&request.buffer[request.buffer_finger..space_index])?)
                }
            } else {
                return Err(Error::BadRequest);
            };

            request.buffer_finger = space_index + LENGTH_SPACE;
        }

        if request.buffer_finger + MIN_LENGTH_REQUEST_LINE_SLICED_2 > request.buffer_length {
            return Err(Error::BadRequest);
        }

        if !request.buffer[request.buffer_finger..].starts_with(b"HTTP/1.1\r\n") {
            return Err(Error::BadRequest);
        }

        request.buffer_finger += LENGTH_PROTOCOL + LENGTH_EOL;

        if request.buffer_finger + MIN_LENGTH_REQUEST_LINE_SLICED_3 > request.buffer_length {
            return Err(Error::BadRequest);
        }

        if !request.buffer[request.buffer_finger..].starts_with(b"\r\n") {
            return Err(Error::BadRequest);
        }

        request.buffer_finger += LENGTH_EOL;

        if request.buffer_finger != request.buffer_length {
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

    mod benchmarks {
        use test::Bencher;

        const REQUEST_LINE: &str = "HEAD /lorem/ipsum/dolor/sit/amet HTTP/1.1\r\n\r\n";

        #[bench]
        fn parse_reader(b: &mut Bencher) {
            use super::Request;
            use super::StringRead;

            let mut reader = StringRead::new(REQUEST_LINE);

            b.iter(|| test::black_box(Request::parse(&mut reader)));
        }
    }
}
