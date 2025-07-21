use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
	let listener = TcpListener::bind("127.0.0.1:1935")?;

	for stream in listener.incoming() {
		println!("Works!");
	}
	Ok(())
}