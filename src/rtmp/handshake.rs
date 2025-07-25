use rtmp::info::{RTMP_VERSION};
use rtmp::connection::{RtmpConnection};
use crate::rtmp;

use rand::Rng;

use std::error::Error;
use std::{fmt};

// Size Constraints ---------------------------//

const _: () = assert!(size_of::<C0>() == 1);
const _: () = assert!(size_of::<C1>() == 1536);
const _: () = assert!(size_of::<C2>() == 1536);

const _: () = assert!(size_of::<S0>() == 1);
const _: () = assert!(size_of::<S1>() == 1536);
const _: () = assert!(size_of::<S2>() == 1536);

//---------------------------------------------//

pub struct C0 { data: [u8; 1] }
impl C0 {
	pub fn from_bytes(data: [u8; 1]) -> C0 { C0 { data } }
	pub fn version(&self) -> u8 { self.data[0] }
}

pub struct S0 { data: [u8; 1] }
impl S0 {
	pub fn new() -> S0 { S0 { data: [0u8; 1] } }
	pub fn set_version(&mut self, version: u8) { self.data[0] = version }
	pub fn version(&self) -> u8 { self.data[0] }
	pub fn into_bytes(&self) -> [u8; 1] { self.data }
}

pub struct C1 { data: [u8; 1536] }
impl C1 {
	pub fn from_bytes(data: [u8; 1536]) -> C1 { C1 { data } }
	pub fn time(&self) -> u32 { u32::from_be_bytes(self.data[0..4].try_into().unwrap()) }
	pub fn zero(&self) -> u32 { u32::from_be_bytes(self.data[4..8].try_into().unwrap()) }
	pub fn random(&self) -> [u8; 1528] { self.data[8..1536].try_into().unwrap() }
}

pub struct S1 { data: [u8; 1536] }
impl S1 {
	pub fn new() -> S1 { S1 { data: [0u8; 1536] } }
	pub fn set_time(&mut self, time: u32) { self.data[0..4].copy_from_slice(&time.to_be_bytes()) }
	pub fn time(&self) -> u32 { u32::from_be_bytes(self.data[0..4].try_into().unwrap()) }
	pub fn set_zero(&mut self, zero: u32) { self.data[4..8].copy_from_slice(&zero.to_be_bytes()) }
	pub fn zero(&self) -> u32 { u32::from_be_bytes(self.data[4..8].try_into().unwrap()) }
	pub fn randomize(&mut self) { rand::rng().fill(&mut self.data[8..1536]) }
	pub fn random(&self) -> [u8; 1528] { self.data[8..1536].try_into().unwrap() }
	pub fn into_bytes(&self) -> [u8; 1536] { self.data }
}

pub struct C2 { data: [u8; 1536] }
impl C2 {
	pub fn from_bytes(data: [u8; 1536]) -> C2 { C2 { data } }
	pub fn time(&self) -> u32 { u32::from_be_bytes(self.data[0..4].try_into().unwrap()) }
	pub fn time2(&self) -> u32 { u32::from_be_bytes(self.data[4..8].try_into().unwrap()) }
	pub fn random_echo(&self) -> [u8; 1528] { self.data[8..1536].try_into().unwrap() }
}

pub struct S2 { data: [u8; 1536] }
impl S2 {
	pub fn new() -> S2 { S2 { data: [0u8; 1536] } }
	pub fn set_time(&mut self, time: u32) { self.data[0..4].copy_from_slice(&time.to_be_bytes()) }
	pub fn time(&self) -> u32 { u32::from_be_bytes(self.data[0..4].try_into().unwrap()) }
	pub fn set_time2(&mut self, time2: u32) { self.data[4..8].copy_from_slice(&time2.to_be_bytes()) }
	pub fn time2(&self) -> u32 { u32::from_be_bytes(self.data[4..8].try_into().unwrap()) }
	pub fn set_random_echo(&mut self, echo: [u8; 1528]) { self.data[8..1536].copy_from_slice(&echo); }
	pub fn into_bytes(&self) -> [u8; 1536] { self.data }
}

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
            HandshakeError::DeprecatedVersionField(version) => write!(f, "Deprecated RTMP Specification Version Requested:\n\tExpected: Version {}; Received: Version {}.", RTMP_VERSION, version),
			HandshakeError::InvalidZeroField(value) => write!(f, "Client Chunk 1 Index zero field MUST be all 0s:\n\tExpected: {:#04x}; Received: {:#04x}.", 0x00, value),
			HandshakeError::MismatchTimeField(expected, received) => write!(f, "Client Chunk 2 Index time field MUST match the Server Chunk 1 Index time field:\n\tExpected: {}; Received: {}.", expected, received),
			HandshakeError::MismatchTime2Field(expected, received) => write!(f, "Client Chunk 2 Index time2 field MUST match the Server Chunk 1 Index zero field:\n\tExpected: {:#04x}; Received: {:#04x}.", expected, received),
			HandshakeError::MismatchEchoField() => write!(f, "Client Chunk 2 Index random echo field MUST match the Server Chunk 1 Index random field.")
		}
    }
}
impl Error for HandshakeError {}

pub fn rtmp_handshake_policy(rtmp_conn: &mut RtmpConnection) -> Result<(), HandshakeError> {
	println!("[RTMP Server] Client '{}' | Starting Handshake.", rtmp_conn.client_ip_addr());

	// Step 1: Read Client Chunk 0 (C0)
	let mut c0_data = [0u8; 1];
	rtmp_conn.read_from_client(&mut c0_data); 
	
	let c0 = C0::from_bytes(c0_data);
	if c0.version() < RTMP_VERSION { return Err(HandshakeError::DeprecatedVersionField(c0.version())); }
	println!("{}", format!("[RTMP Server] Client '{}' | Read C0 | RTMP Specification Version {} Detected.", rtmp_conn.client_ip_addr(), RTMP_VERSION));
	
	// Step 2: Send Server Chunk 0 (S0)
	let mut s0 = S0::new();
	s0.set_version(if c0.version() > RTMP_VERSION { RTMP_VERSION } else { c0.version() });
	rtmp_conn.send_to_client(&mut s0.into_bytes());
	println!("{}", format!("[RTMP Server] Client '{}' | Sent S0 | RTMP Specification Version {} Requested.", rtmp_conn.client_ip_addr(), s0.version()));

	// Step 3: Send Server Chunk 1 (S1)
	let mut s1 = S1::new();
	rtmp_conn.set_server_epoch();
	s1.set_time(rtmp_conn.server_epoch_delta());
	s1.set_zero(0x00);
	s1.randomize();
	rtmp_conn.send_to_client(&mut s1.into_bytes());
	println!("{}", format!("[RTMP Server] Client '{}' | Sent S1 | RTMP Server Epoch {}; Zero Field: {:#04x}.", rtmp_conn.client_ip_addr(), s1.time(), s1.zero()));

	// Step 4: Read Client Chunk 1 (C1)
	let mut c1_data = [0u8; 1536];
	rtmp_conn.read_from_client(&mut c1_data);
	
	let c1 = C1::from_bytes(c1_data);
	rtmp_conn.set_client_epoch(c1.time());
	if c1.zero() != 0x00 { return Err(HandshakeError::InvalidZeroField(c1.zero())); }
	println!("{}", format!("[RTMP Server] Client '{}' | Read C1 | RTMP Client Epoch {}; Zero Field: {:#04x}.", rtmp_conn.client_ip_addr(), c1.time(), c1.zero()));

	// Step 5: Send Server Chunk 2 (S2)
	let mut s2 = S2::new();
	s2.set_time(c1.time());
	s2.set_time2(c1.zero());
	s2.set_random_echo(c1.random());
	rtmp_conn.send_to_client(&mut s2.into_bytes());
	println!("{}", format!("[RTMP Server] Client '{}' | Sent S2 | RTMP Server Time {}; Time2 {}.", rtmp_conn.client_ip_addr(), s2.time(), s2.time2()));

	// Step 6: Read Client Chunk 2 (C2)
	let mut c2_data = [0u8; 1536];
	rtmp_conn.read_from_client(&mut c2_data);

	let c2 = C2::from_bytes(c2_data);
	if c2.time() != s1.time() { return Err(HandshakeError::MismatchTimeField(s1.time(), c2.time())); }
	if c2.time2() != s1.zero() { return Err(HandshakeError::MismatchTime2Field(s1.zero(), c2.time2())); }
	if c2.random_echo() != s1.random() { return Err(HandshakeError::MismatchEchoField()); }
	println!("{}", format!("[RTMP Server] Client '{}' | Read C2 | RTMP Client Time {}; Time2 {}.", rtmp_conn.client_ip_addr(), c2.time(), c2.time2()));

	println!("[RTMP Server] Client '{}' | Handshake Done.", rtmp_conn.client_ip_addr());	

	Ok(())
}
