use std::net::{TcpListener, TcpStream};
use std::{io};

pub struct Server {
	listener: TcpListener
}

impl Server {
	pub fn new(host: &str, port: u16) -> io::Result<Self> {
		let listener: TcpListener = TcpListener::bind(format!("{}:{}", host, port))?;
		Ok(Server { listener })
	}

	pub fn listen(&self) -> io::Result<()> {
		for stream in self.listener.incoming() {
			println!("Works!");
		}
		Ok(())
	}
}