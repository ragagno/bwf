use std::fmt;

use crate::http;

pub struct Response {
    status: http::Status,
}

impl Response {
    pub fn new() -> Self {
        return Self {
            status: http::Status::OK,
        };
    }

    pub fn get_status(&self) -> &http::Status {
        return &self.status;
    }

    pub fn set_status(&mut self, status: http::Status) {
        self.status = status;
    }
}

impl fmt::Display for Response {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return formatter.write_fmt(format_args!("HTTP/1.1 {}\r\n\r\n", self.status));
    }
}
