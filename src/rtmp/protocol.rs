use rtmp::connection::{RtmpConnection};
use rtmp::handshake::{rtmp_handshake_policy};
use rtmp::chunk::*;
use crate::rtmp;

use bitvec::prelude::*;

pub fn rtmp_protocol(mut rtmp_conn: RtmpConnection) {
	// Handshake Policy
	let handshake_attempt = rtmp_handshake_policy(&mut rtmp_conn);
	if let Err(e) = handshake_attempt {
		eprintln!("[RTMP Handshake Error] {}", e); 
		return;
	}
	
	// Chunking
	loop {
		let chunk_basic_header = read_chunk_basic_header(&mut rtmp_conn);
		let chunk_message_header = read_chunk_message_header(&mut rtmp_conn, &chunk_basic_header);
		let extended_timestamp: Option<ExtendedTimestamp>;
		if chunk_message_header.timestamp() == 0xFFFFFF {
			let mut extended_timestamp_data = [0u8; 4];
			rtmp_conn.read_from_client(&mut extended_timestamp_data);
			extended_timestamp = Some(ExtendedTimestamp::from_bytes(extended_timestamp_data));
		}
		else { extended_timestamp = None; }

		let chunk_header: ChunkHeader = ChunkHeader::new(chunk_basic_header, chunk_message_header, extended_timestamp);
		let chunk_data: ChunkData = read_chunk_data(&mut rtmp_conn, chunk_header.message_header());

		match chunk_header.message_header().message_type_id() {
			ChunkMessageType::SetChunkSize => {
				let chunk_size = chunk_data.into_bytes().view_bits::<Msb0>()[0..32].load_be::<u32>();
				println!("New Chunk Size: {}", chunk_size);
				rtmp_conn.set_chunk_size(chunk_size);
			}
			_ => {}
		}
	}
}

fn read_chunk_basic_header(rtmp_conn: &mut RtmpConnection) -> ChunkBasicHeader {
	let mut first_byte_data = [0u8; 1];
	rtmp_conn.read_from_client(&mut first_byte_data);

	let chunk_basic_header = ChunkBasicHeader::OneByte{ one_byte: ChunkBasicHeaderOneByte::from_bytes(first_byte_data) };		
	match chunk_basic_header.chunk_stream_id() {
		0 => {
			// 2 Bytes
			let mut final_byte_data = [0u8; 1];
			rtmp_conn.read_from_client(&mut final_byte_data);

			let mut full_data = [0u8; 2];
			full_data[0..8].copy_from_slice(&mut first_byte_data);
			full_data[8..16].copy_from_slice(&mut final_byte_data);

			ChunkBasicHeader::TwoBytes{ two_bytes: ChunkBasicHeaderTwoBytes::from_bytes(full_data) }
		},
		1 => {
			// 3 Bytes
			let mut final_byte_data = [0u8; 2];
			rtmp_conn.read_from_client(&mut final_byte_data);

			let mut full_data = [0u8; 3];
			full_data[0..8].copy_from_slice(&mut first_byte_data);
			full_data[8..24].copy_from_slice(&mut final_byte_data);

			ChunkBasicHeader::ThreeBytes{ three_bytes: ChunkBasicHeaderThreeBytes::from_bytes(full_data) }
		},
		_ => { chunk_basic_header }
	}
}

fn read_chunk_message_header(rtmp_conn: &mut RtmpConnection, basic_header: &ChunkBasicHeader) -> ChunkMessageHeader {
	match basic_header.chunk_format() {
		ChunkFormat::Type0 => {
			// 11 Bytes
			let mut type0_data = [0u8; 11];
			rtmp_conn.read_from_client(&mut type0_data);

			ChunkMessageHeader::Type0 { type0: ChunkMessageHeaderType0::from_bytes(type0_data) }
		},
		ChunkFormat::Type1 => {
			// 7 Bytes
			let mut type1_data = [0u8; 7];
			rtmp_conn.read_from_client(&mut type1_data);
			
			ChunkMessageHeader::Type1 { type1: ChunkMessageHeaderType1::from_bytes(type1_data) }
		},
		ChunkFormat::Type2 => {
			// 3 Bytes
			let mut type2_data = [0u8; 3];
			rtmp_conn.read_from_client(&mut type2_data);
			
			ChunkMessageHeader::Type2 { type2: ChunkMessageHeaderType2::from_bytes(type2_data) }
		},
		ChunkFormat::Type3 => {
			// 0 Bytes
			ChunkMessageHeader::Type3 { type3: ChunkMessageHeaderType3 {} }
		}
	}
}

fn read_chunk_data(rtmp_conn: &mut RtmpConnection, message_header: &ChunkMessageHeader) -> ChunkData {
	let message_length: usize = message_header.message_length() as usize;
	let max_chunk_size: usize = rtmp_conn.chunk_size() as usize;
	
	let mut data: Vec<u8>;
	if message_length <= max_chunk_size { data = vec![0u8; message_length]; }
	else { data = vec![0u8; max_chunk_size]; }

	rtmp_conn.read_from_client(&mut data);
	return ChunkData::from_bytes(data);
}