use std::net::SocketAddr;

use std::net::Ipv4Addr;
use std::net::SocketAddrV4;

use std::net::TcpStream;
use std::net::TcpListener;

use std::io::Write;

use super::Result;

use crate::http::Status;
use crate::http::Method;

const DEFAULT_ADDRESS: u32 = 0x7F_00_00_01u32;
const DEFAULT_PORT: u16 = 80u16;

pub struct Server {
    address: u32,
    port: u16,
}

impl Server {
    pub fn new() -> Self {
        return Self {
            address: DEFAULT_ADDRESS,
            port: DEFAULT_PORT,
        };
    }

    pub fn start(&self) -> Result<()> {
        let address: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::from(self.address), self.port));

        let listener = TcpListener::bind(address)?;

        for stream in listener.incoming() {
            let stream = stream;

            match stream {
                Ok(stream) => match self.handle(stream) {
                    Ok(_) => {}
                    Err(err) => {
                        println!("{}", err);
                    }
                }
                Err(err) => {
                    println!("{}", err);
                }
            }
        }

        return Ok(());
    }

    fn handle(&self, mut stream: TcpStream) -> Result<()> {
        let request = super::Request::parse(&mut stream)?;

        let mut response = super::Response::new();

        match request.get_method() {
            Method::GET => {
                response.set_status(Status::OK);
            }
            _ => {
                response.set_status(Status::NotFound);
            }
        }

        print!("Request:\n{}", request);
        print!("Response:\n{}", response);
        println!("----");

        stream.write(response.to_string().as_bytes())?;

        return Ok(());
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
