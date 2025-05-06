use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection { stream }
    }

    pub fn connect(addr: &str) -> Result<Self, std::io::Error> {
        let stream = TcpStream::connect(addr)?;
        stream.set_read_timeout(Some(Duration::from_secs(30)))?;
        Ok(Connection { stream })
    }

    pub fn send(&mut self, message: &[u8]) -> Result<(), std::io::Error> {
        self.stream.write_all(message)?;
        Ok(())
    }

    pub fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, std::io::Error> {
        self.stream.read(buffer)
    }
}
