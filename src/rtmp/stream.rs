use std::{io::{self, Read, Write}, net::{SocketAddr, TcpStream}};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Stream {
	stream: TcpStream,

	ip: String,
	port: u16,
	server_epoch: u128,
	client_epoch: u32
}

impl Stream {
	pub fn new(stream: TcpStream) -> io::Result<Self> {
		let peer_addr: SocketAddr = stream.peer_addr()?;
		let ip: String = peer_addr.ip().to_string();
		let port: u16 = peer_addr.port();
		Ok(Stream {stream, ip, port, server_epoch: 0x00, client_epoch: 0x00})
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

	pub fn read_from_client(&mut self, buffer: &mut [u8]) {
		self.stream.read_exact(buffer);
	}

	pub fn send_to_client(&mut self, buffer: &[u8]) {
		self.stream.write_all(buffer);
	}

	pub fn set_server_epoch(&mut self) {
		self.server_epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time should go forward.").as_millis();
	}

	pub fn server_epoch_delta(&self) -> u32 {
		let epoch_delta = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time should go forward.").as_millis() - self.server_epoch;
		epoch_delta as u32
	}

	pub fn set_client_epoch(&mut self, epoch: u32) {
		self.client_epoch = epoch
	}	
}