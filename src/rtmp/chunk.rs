use rand::Rng;

pub const C0_SIZE: usize = 1;
pub const C1_SIZE: usize = 1536;
pub const C2_SIZE: usize = 1536;

pub const S0_SIZE: usize = 1;
pub const S1_SIZE: usize = 1536;
pub const S2_SIZE: usize = 1536;

const _: () = assert!(size_of::<C0>() == C0_SIZE);
const _: () = assert!(size_of::<C1>() == C1_SIZE);
const _: () = assert!(size_of::<C2>() == C2_SIZE);

const _: () = assert!(size_of::<S0>() == S0_SIZE);
const _: () = assert!(size_of::<S1>() == S1_SIZE);
const _: () = assert!(size_of::<S2>() == S2_SIZE);

struct BasicHeader {

}

struct MessageHeader {

}

struct ExtendedTimestamp {

}

struct Header {
	basic_header: BasicHeader,
	message_header: MessageHeader,
	extended_timestamp: ExtendedTimestamp
}

struct Data {

}

struct Chunk {
	header: Header,
	data: Data
}

pub struct C0 { buff: [u8; C0_SIZE] }
impl C0 {
	pub fn new(buffer: [u8; C0_SIZE]) -> C0 { C0 { buff: buffer } }
	pub fn version(&self) -> u8 { self.buff[0] }
}

pub struct S0 { buff: [u8; S0_SIZE]}
impl S0 {
	pub fn new() -> S0 { S0 { buff: [0u8; S0_SIZE] } }
	pub fn set_version(&mut self, version: u8) { self.buff[0] = version }
	pub fn version(&self) -> u8 { self.buff[0] }
	pub fn buffer(&self) -> [u8; S0_SIZE] { self.buff }
}

pub struct C1 { buff: [u8; C1_SIZE] }
impl C1 {
	pub fn new(buffer: [u8; C1_SIZE]) -> C1 { C1 { buff: buffer } }
	pub fn time(&self) -> u32 { u32::from_be_bytes(self.buff[0..4].try_into().unwrap()) }
	pub fn zero(&self) -> u32 { u32::from_be_bytes(self.buff[4..8].try_into().unwrap()) }
	pub fn random(&self) -> [u8; 1528] { self.buff[8..C1_SIZE].try_into().unwrap() }
}

pub struct S1 { buff: [u8; S1_SIZE] }
impl S1 {
	pub fn new() -> S1 { S1 { buff: [0u8; S1_SIZE] } }
	pub fn set_time(&mut self, time: u32) { self.buff[0..4].copy_from_slice(&time.to_be_bytes()) }
	pub fn time(&self) -> u32 { u32::from_be_bytes(self.buff[0..4].try_into().unwrap()) }
	pub fn set_zero(&mut self, zero: u32) { self.buff[4..8].copy_from_slice(&zero.to_be_bytes()) }
	pub fn zero(&self) -> u32 { u32::from_be_bytes(self.buff[4..8].try_into().unwrap()) }
	pub fn randomize(&mut self) { rand::rng().fill(&mut self.buff[8..S1_SIZE]) }
	pub fn random(&self) -> [u8; 1528] { self.buff[8..S1_SIZE].try_into().unwrap() }
	pub fn buffer(&self) -> [u8; S1_SIZE] { self.buff }
}

pub struct C2 { buff: [u8; C2_SIZE] }
impl C2 {
	pub fn new(buffer: [u8; C2_SIZE]) -> C2 { C2 { buff: buffer } }
	pub fn time(&self) -> u32 { u32::from_be_bytes(self.buff[0..4].try_into().unwrap()) }
	pub fn time2(&self) -> u32 { u32::from_be_bytes(self.buff[4..8].try_into().unwrap()) }
	pub fn random_echo(&self) -> [u8; 1528] { self.buff[8..C2_SIZE].try_into().unwrap() }
}

pub struct S2 { buff: [u8; S2_SIZE] }
impl S2 {
	pub fn new() -> S2 { S2 { buff: [0u8; S2_SIZE] } }
	pub fn set_time(&mut self, time: u32) { self.buff[0..4].copy_from_slice(&time.to_be_bytes()) }
	pub fn time(&self) -> u32 { u32::from_be_bytes(self.buff[0..4].try_into().unwrap()) }
	pub fn set_time2(&mut self, time2: u32) { self.buff[4..8].copy_from_slice(&time2.to_be_bytes()) }
	pub fn time2(&self) -> u32 { u32::from_be_bytes(self.buff[4..8].try_into().unwrap()) }
	pub fn set_random_echo(&mut self, echo: [u8; 1528]) { self.buff[8..S2_SIZE].copy_from_slice(&echo); }
	pub fn buffer(&self) -> [u8; S2_SIZE] { self.buff }
}