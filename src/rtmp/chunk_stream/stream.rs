use rtmp::connection::{RtmpConnection};
use rtmp::chunk_stream::basic_header::*;
use rtmp::chunk_stream::message_header::*;
use rtmp::chunk_stream::header::*;
use crate::rtmp;

use bitvec::prelude::*;

pub struct RtmpChunkStream { rtmp_conn: RtmpConnection }
impl RtmpChunkStream {
	pub fn new(rtmp_conn: RtmpConnection) -> RtmpChunkStream { RtmpChunkStream { rtmp_conn } }
	
	pub fn chunking(&mut self) {
		let chunk_header = self.read_chunk_header();
		self.read_chunk_data(chunk_header);
	}
	
	pub fn read_chunk_data(&mut self, chunk_header: ChunkHeader) {
		match chunk_header.message_header().message_type_id() {
			ChunkMessageType::SetChunkSize => {
				let message_length = chunk_header.message_header().message_length();
				if message_length <= self.rtmp_conn.chunk_size() {
					let mut data: Vec<u8> = vec![0u8; message_length as usize];
					self.rtmp_conn.read_from_client(&mut data);
					let chunk_size = data.view_bits::<Msb0>()[0..32].load_be::<u32>();
					self.rtmp_conn.set_chunk_size(chunk_size);
				}
			},
			_ => {
				
			}
		}
	}

	pub fn read_chunk_header(&mut self) -> ChunkHeader {
		let basic_header = self.read_basic_header();
		let message_header = self.read_message_header(&basic_header);
		if message_header.timestamp() == 0xFFFFFF {
			let mut extended_timestamp_data = [0u8; 4];
			self.rtmp_conn.read_from_client(&mut extended_timestamp_data);
			ChunkHeader::new(basic_header, message_header, Some(ExtendedTimestamp::from_bytes(extended_timestamp_data)))
		}
		else { ChunkHeader::new(basic_header, message_header, None) }
	}

	pub fn read_basic_header(&mut self) -> ChunkBasicHeader {
		let mut first_byte_data = [0u8; 1];
		self.rtmp_conn.read_from_client(&mut first_byte_data);

		let chunk_basic_header = ChunkBasicHeader::OneByte{ one_byte: ChunkBasicHeaderOneByte::from_bytes(first_byte_data) };		
		match chunk_basic_header.chunk_stream_id() {
			0 => {
				// 2 Bytes
				let mut final_byte_data = [0u8; 1];
				self.rtmp_conn.read_from_client(&mut final_byte_data);

				let mut full_data = [0u8; 2];
				full_data[0..8].copy_from_slice(&mut first_byte_data);
				full_data[8..16].copy_from_slice(&mut final_byte_data);

				ChunkBasicHeader::TwoBytes{ two_bytes: ChunkBasicHeaderTwoBytes::from_bytes(full_data) }
			},
			1 => {
				// 3 Bytes
				let mut final_byte_data = [0u8; 2];
				self.rtmp_conn.read_from_client(&mut final_byte_data);

				let mut full_data = [0u8; 3];
				full_data[0..8].copy_from_slice(&mut first_byte_data);
				full_data[8..24].copy_from_slice(&mut final_byte_data);

				ChunkBasicHeader::ThreeBytes{ three_bytes: ChunkBasicHeaderThreeBytes::from_bytes(full_data) }
			},
			_ => { chunk_basic_header }
		}
	}

	pub fn read_message_header(&mut self, basic_header: &ChunkBasicHeader) -> ChunkMessageHeader {
		match basic_header.chunk_format() {
			ChunkFormat::Type0 => {
				// 11 Bytes
				let mut type0_data = [0u8; 11];
				self.rtmp_conn.read_from_client(&mut type0_data);

				ChunkMessageHeader::Type0 { type0: ChunkMessageHeaderType0::from_bytes(type0_data) }
			},
			ChunkFormat::Type1 => {
				// 7 Bytes
				let mut type1_data = [0u8; 7];
				self.rtmp_conn.read_from_client(&mut type1_data);
				
				ChunkMessageHeader::Type1 { type1: ChunkMessageHeaderType1::from_bytes(type1_data) }
			},
			ChunkFormat::Type2 => {
				// 3 Bytes
				let mut type2_data = [0u8; 3];
				self.rtmp_conn.read_from_client(&mut type2_data);
				
				ChunkMessageHeader::Type2 { type2: ChunkMessageHeaderType2::from_bytes(type2_data) }
			},
			ChunkFormat::Type3 => {
				// 0 Bytes
				ChunkMessageHeader::Type3 { type3: ChunkMessageHeaderType3 {} }
			}
		}
	}
}