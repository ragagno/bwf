use std::net::SocketAddr;

use std::net::Ipv4Addr;
use std::net::SocketAddrV4;

use std::net::Ipv6Addr;
use std::net::SocketAddrV6;

use std::net::TcpStream;

use std::io::Write;

use rul::interface::address::Enum as IPAddress;

use crate::http::Result;

use super::Request;
use super::Response;

const DEFAULT_ADDRESS: u32 = 0x7F_00_00_01u32;
const DEFAULT_PORT: u16 = 80u16;

pub struct Client {
    address: IPAddress,
    port: u16,
}

impl Client {
    pub fn new() -> Self {
        return Self {
            address: IPAddress::Ipv4(DEFAULT_ADDRESS),
            port: DEFAULT_PORT,
        };
    }

    pub fn send(&self, request: &Request) -> Result<Response> {
        let address: SocketAddr = match self.address {
            IPAddress::Ipv4(address) => SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::from(address), self.port)),
            IPAddress::Ipv6(address) => SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::from(address), self.port, 0u32, 0u32)),
        };

        let mut stream: TcpStream = TcpStream::connect(address)?;

        stream.write_all(request.to_string().as_bytes())?;

        return Response::parse(&mut stream);
    }
}

rul_implement_address_get!(Client, address);
rul_implement_address_set!(Client, address);

rul_implement_port_get!(Client, port);
rul_implement_port_set!(Client, port);
