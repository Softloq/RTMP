use std::error::Error;
use std::{fmt, io};

use rtmp::info::{SPEC_VERSION};
use rtmp::server::{Server};
use rtmp::client::{Client};
use rtmp::chunk::{C0_SIZE, C0, S0, C1_SIZE, C1, S1, C2, S2};

use crate::rtmp;

#[derive(Debug)]
pub enum HandshakeError {
    C0InvalidVersionField(u8),
	C1InvalidZeroField(u32)
}

impl fmt::Display for HandshakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HandshakeError::C0InvalidVersionField(version) => write!(f, "Invalid RTMP Specification Version Requested:\n\tExpected: Version {}; Received: Version {}.", SPEC_VERSION, version),
			HandshakeError::C1InvalidZeroField(value) => write!(f, "Chunk Index 1 zero field MUST be all 0s:\n\tExpected: {:#04x}; Received: {:#04x}.", 0x00, value)
		}
    }
}
impl Error for HandshakeError {}

pub fn client_handshake(client: &mut Client) -> Result<(), HandshakeError> {
	println!("[Client] {} beginning the RTMP Handshake", client.ip_addr());

	// Step 1: Client Chunk 0 (C0)
	let mut c0_data = [0u8; C0_SIZE];
	client.read(&mut c0_data); 
	
	let c0 = C0::new(c0_data);
	if c0.version() != SPEC_VERSION { return Err(HandshakeError::C0InvalidVersionField(c0.version())); }
	println!("{}", format!("[Client] RTMP Specification Version {} Detected.", SPEC_VERSION));

	// Step 2: Client Chunk 1 (C1)
	let mut c1_data = [0u8; C1_SIZE];
	client.read(&mut c1_data);
	
	let c1 = C1::new(c1_data);
	client.set_epoch(c1.time());
	if c1.zero() != 0x00 { return Err(HandshakeError::C1InvalidZeroField(c1.zero())); }
	println!("{}", format!("[Client] RTMP Epoch {:#06x}, Zero {:#04x}.", c1.time(), c1.zero()));
	
	Ok(())
}