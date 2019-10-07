use std::net::SocketAddr;

use std::net::Ipv4Addr;
use std::net::SocketAddrV4;

use std::net::Ipv6Addr;
use std::net::SocketAddrV6;

use std::net::TcpStream;
use std::net::TcpListener;

use std::io::Write;

use rul::interface::address::Enum as IPAddress;

use crate::http::Result;

use crate::http::Status;
use crate::http::Method;

const DEFAULT_ADDRESS: u32 = 0x7F_00_00_01u32;
const DEFAULT_PORT: u16 = 80u16;

pub struct Server {
    address: IPAddress,
    port: u16,
}

impl Server {
    pub fn new() -> Self {
        return Self {
            address: IPAddress::Ipv4(DEFAULT_ADDRESS),
            port: DEFAULT_PORT,
        };
    }

    pub fn start(&self) -> Result<()> {
        let address: SocketAddr = match self.address {
            IPAddress::Ipv4(address) => SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::from(address), self.port)),
            IPAddress::Ipv6(address) => SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::from(address), self.port, 0u32, 0u32)),
        };

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
}

rul_implement_address_get!(Server, address);
rul_implement_address_set!(Server, address);

rul_implement_port_get!(Server, port);
rul_implement_port_set!(Server, port);
