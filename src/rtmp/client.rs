use rtmp::chunk::{C0};
use std::{io::{self, Read}, net::{SocketAddr, TcpStream}};

enum HandshakeError {
    InvalidInput(String),
    ResourceNotFound,
    PermissionDenied,
}

pub struct Client {
	stream: TcpStream,

	ip: String,
	port: u16
}

impl Client {
	pub fn new(stream: TcpStream) -> io::Result<Self> {
		let peer_addr: SocketAddr = stream.peer_addr()?;
		let ip: String = peer_addr.ip().to_string();
		let port: u16 = peer_addr.port();
		Ok(Client {stream, ip, port})
	}

	pub fn ip(&self) -> &str {
		&self.ip
	}

	pub fn port(&self) -> &u16 {
		&self.port
	}

	pub fn ip_addr(&self) -> String {
		return format!("{}:{}", &self.ip(), &self.port()).to_string();
	}

	pub fn read(&mut self, buffer: &mut [u8]) {
		self.stream.read_exact(buffer);
	}
}