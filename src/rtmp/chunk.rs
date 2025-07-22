pub const C0_SIZE: usize = 1;
pub const S0_SIZE: usize = 1;
pub const C1_SIZE: usize = 1536;
pub const C1_RAND_SIZE: usize = 1528;
pub const S1_SIZE: usize = 1536;

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

pub struct S0 {

}

pub struct C1 { buff: [u8; C1_SIZE] }
impl C1 {
	pub fn new(buffer: [u8; C1_SIZE]) -> C1 { C1 { buff: buffer } }
	pub fn time(&self) -> u32 {
		u32::from_be_bytes(self.buff[0..4].try_into().unwrap())
	}
	pub fn zero(&self) -> u32 {
		u32::from_be_bytes(self.buff[4..8].try_into().unwrap())
	}
	pub fn random(&self) -> [u8; C1_RAND_SIZE] {
		self.buff[8..C1_SIZE].try_into().unwrap()
	}
}

pub struct S1 {

}

pub struct C2 {

}

pub struct S2 {

}