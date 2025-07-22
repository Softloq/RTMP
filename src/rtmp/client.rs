use std::{io, net::{SocketAddr, TcpStream}};

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

	pub fn handshake(&self) {
		println!("[Client] {} beginning the RTMP Handshake", self.ip_addr());
	}
}