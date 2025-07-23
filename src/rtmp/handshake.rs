use std::error::Error;
use std::{fmt};

use rtmp::stream::{Stream};
use rtmp::chunk::{C0_SIZE, C1_SIZE, C2_SIZE, C0, C1, C2, S0, S1, S2};
use rtmp::info::{SPEC_VERSION};

use crate::rtmp;

#[derive(Debug)]
pub enum HandshakeError {
    DeprecatedVersionField(u8),
	InvalidZeroField(u32),
	MismatchTimeField(u32, u32),
	MismatchTime2Field(u32, u32),
	MismatchEchoField()
}

impl fmt::Display for HandshakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HandshakeError::DeprecatedVersionField(version) => write!(f, "Deprecated RTMP Specification Version Requested:\n\tExpected: Version {}; Received: Version {}.", SPEC_VERSION, version),
			HandshakeError::InvalidZeroField(value) => write!(f, "Client Chunk 1 Index zero field MUST be all 0s:\n\tExpected: {:#04x}; Received: {:#04x}.", 0x00, value),
			HandshakeError::MismatchTimeField(expected, received) => write!(f, "Client Chunk 2 Index time field MUST match the Server Chunk 1 Index time field:\n\tExpected: {}; Received: {}.", expected, received),
			HandshakeError::MismatchTime2Field(expected, received) => write!(f, "Client Chunk 2 Index time2 field MUST match the Server Chunk 1 Index zero field:\n\tExpected: {:#04x}; Received: {:#04x}.", expected, received),
			HandshakeError::MismatchEchoField() => write!(f, "Client Chunk 2 Index random echo field MUST match the Server Chunk 1 Index random field.")
		}
    }
}
impl Error for HandshakeError {}

pub fn handshake(stream: &mut Stream) -> Result<(), HandshakeError> {
	println!("[RTMP Server] Client '{}' | Starting Handshake.", stream.ip_addr());

	// Step 1: Read Client Chunk 0 (C0)
	let mut c0_data = [0u8; C0_SIZE];
	stream.read_from_client(&mut c0_data); 
	
	let c0 = C0::new(c0_data);
	if c0.version() < SPEC_VERSION { return Err(HandshakeError::DeprecatedVersionField(c0.version())); }
	println!("{}", format!("[RTMP Server] Client '{}' | Read C0 | RTMP Specification Version {} Detected.", stream.ip_addr(), SPEC_VERSION));

	// Step 2: Send Server Chunk 0 (S0)
	let mut s0 = S0::new();
	s0.set_version(if c0.version() > SPEC_VERSION { SPEC_VERSION } else { c0.version() }); // Future-proofing
	stream.send_to_client(&s0.buffer());
	println!("{}", format!("[RTMP Server] Client '{}' | Sent S0 | RTMP Specification Version {} Requested.", stream.ip_addr(), s0.version()));

	// Step 3: Send Server Chunk 1 (S1)
	let mut s1 = S1::new();
	stream.set_server_epoch();
	s1.set_time(stream.server_epoch_delta());
	s1.set_zero(0x00);
	s1.randomize();
	stream.send_to_client(&s1.buffer());
	println!("{}", format!("[RTMP Server] Client '{}' | Sent S1 | RTMP Server Epoch {}; Zero Field: {:#04x}.", stream.ip_addr(), s1.time(), s1.zero()));

	// Step 4: Read Client Chunk 1 (C1)
	let mut c1_data = [0u8; C1_SIZE];
	stream.read_from_client(&mut c1_data);
	
	let c1 = C1::new(c1_data);
	stream.set_client_epoch(c1.time());
	if c1.zero() != 0x00 { return Err(HandshakeError::InvalidZeroField(c1.zero())); }
	println!("{}", format!("[RTMP Server] Client '{}' | Read C1 | RTMP Client Epoch {}; Zero Field: {:#04x}.", stream.ip_addr(), c1.time(), c1.zero()));

	// Step 5: Send Server Chunk 2 (S2)
	let mut s2 = S2::new();
	s2.set_time(c1.time());
	s2.set_time2(c1.zero());
	s2.set_random_echo(c1.random());
	stream.send_to_client(&s2.buffer());
	println!("{}", format!("[RTMP Server] Client '{}' | Sent S2 | RTMP Server Time {}; Time2 {}.", stream.ip_addr(), s2.time(), s2.time2()));

	// Step 6: Read Client Chunk 2 (C2)
	let mut c2_data = [0u8; C2_SIZE];
	stream.read_from_client(&mut c2_data);

	let c2 = C2::new(c2_data);
	if c2.time() != s1.time() { return Err(HandshakeError::MismatchTimeField(s1.time(), c2.time())); }
	if c2.time2() != s1.zero() { return Err(HandshakeError::MismatchTime2Field(s1.zero(), c2.time2())); }
	if c2.random_echo() != s1.random() { return Err(HandshakeError::MismatchEchoField()); }
	println!("{}", format!("[RTMP Server] Client '{}' | Read C2 | RTMP Client Time {}; Time2 {}.", stream.ip_addr(), c2.time(), c2.time2()));

	println!("[RTMP Server] Client '{}' | Handshake Done.", stream.ip_addr());	

	Ok(())
}