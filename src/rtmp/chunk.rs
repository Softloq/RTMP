pub const C0_SIZE: usize = 1;
pub const S0_SIZE: usize = 1;
pub const C1_SIZE: usize = 1536;
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

union UnsafeC0 {
	pub data: [u8; C0_SIZE],
	pub version: u8
}

pub struct C0 {
	value: UnsafeC0
}

impl C0 {
	pub fn new(data: [u8; C0_SIZE]) -> C0 {
		C0 { value: UnsafeC0 { data: data } }
	}
	pub fn version(&self) -> u8 {
		unsafe {
			self.value.version
		}
	}
}

pub struct S0 {

}

pub struct C1 {
	pub data: [u8; 1536]
}

pub struct S1 {

}

pub struct C2 {

}

pub struct S2 {

}