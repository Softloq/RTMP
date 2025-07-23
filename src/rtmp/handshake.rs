use std::error::Error;
use std::{fmt, io};

use rtmp::info::{SPEC_VERSION};
use rtmp::server::{Server};
use rtmp::stream::{Stream};
use rtmp::chunk::{C0_SIZE, C0, S0, C1_SIZE, C1, S1, C2, S2};

use crate::rtmp;

#[derive(Debug)]
pub enum HandshakeError {
    DeprecatedVersionField(u8),
	InvalidZeroField(u32)
}

impl fmt::Display for HandshakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HandshakeError::DeprecatedVersionField(version) => write!(f, "Deprecated RTMP Specification Version Requested:\n\tExpected: Version {}; Received: Version {}.", SPEC_VERSION, version),
			HandshakeError::InvalidZeroField(value) => write!(f, "Client Chunk 1 Index zero field MUST be all 0s:\n\tExpected: {:#04x}; Received: {:#04x}.", 0x00, value)
		}
    }
}
impl Error for HandshakeError {}

pub fn handshake(stream: &mut Stream) -> Result<(), HandshakeError> {
	println!("[Client {}] Starting the RTMP Handshake.", stream.ip_addr());

	// Step 1: Read Client Chunk 0 (C0)
	let mut c0_data = [0u8; C0_SIZE];
	stream.read_from_client(&mut c0_data); 
	
	let c0 = C0::new(c0_data);
	if c0.version() < SPEC_VERSION { return Err(HandshakeError::DeprecatedVersionField(c0.version())); }
	println!("{}", format!("[Client {}] Read C0 | RTMP Specification Version {} Detected.", stream.ip_addr(), SPEC_VERSION));

	// Step 2: Read Client Chunk 1 (C1)
	let mut c1_data = [0u8; C1_SIZE];
	stream.read_from_client(&mut c1_data);
	
	let c1 = C1::new(c1_data);
	stream.set_client_epoch(c1.time());
	if c1.zero() != 0x00 { return Err(HandshakeError::InvalidZeroField(c1.zero())); }
	println!("{}", format!("[Client {}] Read C1 | RTMP Client Epoch {}; Zero Field: {:#04x}.", stream.ip_addr(), c1.time(), c1.zero()));

	// Step 3: Send Server Chunk 0 (S0)
	let mut s0 = S0::new();
	s0.set_version(if c0.version() > SPEC_VERSION { SPEC_VERSION } else { c0.version() }); // Future-proofing
	stream.send_to_client(&s0.buffer());
	println!("{}", format!("[Client {}] Sent S0 | RTMP Specification Version {} Requested.", stream.ip_addr(), s0.version()));

	// Step 4: Send Server Chunk 1 (S1)
	let mut s1 = S1::new();
	stream.set_server_epoch();
	s1.set_time(stream.server_epoch_delta());
	s1.set_zero(0x00);
	s1.randomize();
	stream.send_to_client(&s1.buffer());
	println!("{}", format!("[Client {}] Sent S1 | RTMP Server Epoch {}; Zero Field: {:#04x}.", stream.ip_addr(), s1.time(), s1.zero()));

	Ok(())
}