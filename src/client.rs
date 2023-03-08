use net::TcpStream;
use std::net;
use std::io;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn connect<A: net::ToSocketAddrs>(addr: A) -> io::Result<Client> {
        match TcpStream::connect(addr) {
            Ok(s) => Ok(Self{
                stream: s
            }),
            Err(e) => Err(e)
        }
    }
}