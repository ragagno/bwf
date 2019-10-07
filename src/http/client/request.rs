use std::fmt;

use crate::http::Method;

pub struct Request {
    method: Method,

    target: String,
}

impl Request {
    pub fn new() -> Self {
        return Self {
            method: Method::GET,

            target: String::from("/"),
        };
    }

    pub fn set_method(&mut self, method: Method) {
        self.method = method;
    }

    pub fn get_method(&self) -> &Method {
        return &self.method;
    }

    pub fn set_target(&mut self, path: &str) {
        self.target = String::from(path);
    }

    pub fn get_target(&self) -> &str {
        return &self.target;
    }
}

impl fmt::Display for Request {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return formatter.write_fmt(format_args!("{} {} HTTP/1.1\r\n\r\n", self.method, self.target));
    }
}
