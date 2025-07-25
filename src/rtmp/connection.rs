use std::{io::{self, Read, Write}, net::{SocketAddr, TcpStream}};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct RtmpConnection {
	stream: TcpStream,

	ip: String,
	port: u16,
	server_epoch: u128,
	client_epoch: u32,

	chunk_size: u32
}

impl RtmpConnection {
	pub fn new(stream: TcpStream) -> io::Result<Self> {
		let peer_addr: SocketAddr = stream.peer_addr()?;
		let ip: String = peer_addr.ip().to_string();
		let port: u16 = peer_addr.port();
		Ok(RtmpConnection {stream, ip, port, server_epoch: 0x00, client_epoch: 0x00, chunk_size: 128})
	}

	pub fn client_ip(&self) -> &String { &self.ip }
	pub fn client_port(&self) -> u16 { self.port }
	pub fn client_ip_addr(&self) -> String { format!("{}:{}", self.client_ip(), self.client_port()).to_string() }

	pub fn read_from_client(&mut self, buffer: &mut [u8]) { self.stream.read_exact(buffer); }
	pub fn send_to_client(&mut self, buffer: &mut [u8]) { self.stream.write_all(buffer); }

	pub fn set_server_epoch(&mut self) { self.server_epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time should go forward.").as_millis(); }
	pub fn server_epoch_delta(&self) -> u32 {
		let epoch_delta = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time should go forward.").as_millis() - self.server_epoch;
		epoch_delta as u32
	}

	pub fn set_client_epoch(&mut self, epoch: u32) { self.client_epoch = epoch }

	pub fn set_chunk_size(&mut self, chunk_size: u32) { self.chunk_size = chunk_size }
	pub fn chunk_size(&self) -> u32 { self.chunk_size }
}