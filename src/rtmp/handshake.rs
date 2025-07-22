use std::error::Error;
use std::{fmt, io};

use rtmp::info::{SPEC_VERSION};
use rtmp::server::{Server};
use rtmp::client::{Client};
use rtmp::chunk::{C0_SIZE, C0, S0, S1, C2, S2};

#[derive(Debug)]
pub enum HandshakeError {
    InvalidVersion(u8)
}

impl fmt::Display for HandshakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HandshakeError::InvalidVersion(version) => write!(f, "Invalid RTMP Specification Version Requested:\n\tExpected: Version {}; Received: Version {}.", SPEC_VERSION, version)
        }
    }
}
impl Error for HandshakeError {}

pub fn client_handshake(client: &mut Client) -> Result<(), HandshakeError> {
	println!("[Client] {} beginning the RTMP Handshake", client.ip_addr());

	// Step 1: Client Chunk 0 (C0),

	let mut c0_data = [0u8; C0_SIZE];
	client.read(&mut c0_data); 
	let c0: C0 = C0::new(c0_data);
	if c0.version() != SPEC_VERSION { return Err(HandshakeError::InvalidVersion(c0.version())); }
	println!("{}", format!("[Client] RTMP Specification Version {} Detected.", SPEC_VERSION));
	Ok(())
}