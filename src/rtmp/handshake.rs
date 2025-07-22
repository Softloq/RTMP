use std::io;

use rtmp::server::{Server};
use rtmp::client::{Client};
use rtmp::chunk::{C0_SIZE, C0, S0, C1, S1, C2, S2};

pub enum HandshakeError {
    InvalidInput(String),
    ResourceNotFound,
    PermissionDenied,
}

pub fn client_handshake(client: &mut Client) -> io::Result<()> {
	println!("[Client] {} beginning the RTMP Handshake", client.ip_addr());

	// Step 1: Client Chunk 0 (C0),
	// Represents RTMP Version and MUST be 3.

	let mut c0_data = [0u8; C0_SIZE];
	client.read(&mut c0_data); 
	let c0: C0 = C0::new(c0_data);
	println!("[Client] {} C0: {:#04x}", client.ip_addr(), c0.version());
	
	Ok(())
}