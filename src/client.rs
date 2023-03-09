use net::TcpStream;
use std::net;
use std::io;
use std::io::Read;

pub struct Client {
    stream: TcpStream,
}

impl Client{
    pub fn connect<A: net::ToSocketAddrs>(addr: A) -> io::Result<Client> {
        Ok(Self {
            stream: TcpStream::connect(addr)?
        })
    }
    pub fn handle_packets(mut self) {
        loop {
            let mut buf = [0 as u8; 6];
            match self.stream.read(&mut buf) {
                Err(_) => { break; }
                Ok(n) => {
                    if n <= 0 { break }
                    println!("{}", String::from_utf8(Vec::from(buf)).unwrap());
                }
            }
        }
    }
}