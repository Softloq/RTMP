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

#[derive(Copy, Clone)]
struct C0Info {
	pub version: u8
}
union UnsafeC0 {
	pub buff: [u8; C0_SIZE],
	pub info: C0Info
}
pub struct C0 { data: UnsafeC0 }
impl C0 {
	pub fn new(buffer: [u8; C0_SIZE]) -> C0 { C0 { data: UnsafeC0 { buff: buffer } } }
	pub fn version(&self) -> u8 { unsafe { self.data.info.version } }
}

pub struct S0 {

}
#[derive(Copy, Clone)]
struct C1Info {
	pub time: u32,
	pub zero: u32,
	pub random: [u8; C1_RAND_SIZE]
}
union UnsafeC1 {
	pub buff: [u8; C1_SIZE],
	pub info: C1Info
}
pub struct C1 { data: UnsafeC1 }
impl C1 {
	pub fn new(buffer: [u8; C1_SIZE]) -> C1 { C1 { data: UnsafeC1 { buff: buffer } } }
	pub fn time(&self) -> u32 { unsafe { self.data.info.time } }
	pub fn zero(&self) -> u32 { unsafe { self.data.info.zero } }
	pub fn random(&self) -> [u8; C1_RAND_SIZE] { unsafe { self.data.info.random } }
}

pub struct S1 {

}

pub struct C2 {

}

pub struct S2 {

}