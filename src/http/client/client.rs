use std::net::SocketAddr;

use std::net::Ipv4Addr;
use std::net::SocketAddrV4;

use std::net::TcpStream;

use std::io::Write;

use crate::http::Result;

use super::Request;
use super::Response;

const DEFAULT_ADDRESS: u32 = 0x7F_00_00_01u32;
const DEFAULT_PORT: u16 = 80u16;

pub struct Client {
    address: u32,
    port: u16,
}

impl Client {
    pub fn new() -> Self {
        return Self {
            address: DEFAULT_ADDRESS,
            port: DEFAULT_PORT,
        };
    }

    pub fn send(&self, request: &Request) -> Result<Response> {
        let address: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::from(self.address), self.port));

        let mut stream: TcpStream = TcpStream::connect(address)?;

        stream.write_all(request.to_string().as_bytes())?;

        return Response::parse(&mut stream);
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn get_port(&mut self) -> u16 {
        self.port
    }

    pub fn set_address(&mut self, address: u32) {
        self.address = address;
    }

    pub fn get_address(&mut self) -> u32 {
        self.address
    }
}
