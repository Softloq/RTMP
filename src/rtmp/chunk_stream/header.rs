use rtmp::chunk_stream::basic_header::{ChunkBasicHeader};
use rtmp::chunk_stream::message_header::{ChunkMessageHeader};
use crate::rtmp;

pub struct ExtendedTimestamp { data: [u8; 4] }
impl ExtendedTimestamp {
	pub fn from_bytes(data: [u8; 4]) -> ExtendedTimestamp { ExtendedTimestamp { data } }
	pub fn timestamp(&self) -> u32 { u32::from_be_bytes(self.data.try_into().unwrap()) }
}

pub struct ChunkHeader {
	basic_header: ChunkBasicHeader,
	message_header: ChunkMessageHeader,
	extended_timestamp: Option<ExtendedTimestamp>
}
impl ChunkHeader {
	pub fn new(basic_header: ChunkBasicHeader, message_header: ChunkMessageHeader, extended_timestamp: Option<ExtendedTimestamp>) -> ChunkHeader {
		ChunkHeader{ basic_header, message_header, extended_timestamp }
	}

	pub fn basic_header(&self) -> &ChunkBasicHeader { &self.basic_header }
	pub fn message_header(&self) -> &ChunkMessageHeader { &self.message_header }
	pub fn has_extended_timestamp(&self) -> bool { self.extended_timestamp.is_some() }
	pub fn extended_timestamp(&self) -> u32 {
		match &self.extended_timestamp {
			Some(e) => e.timestamp(),
			None => 0
		}
	}
}
