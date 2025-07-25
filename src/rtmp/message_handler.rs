use rtmp::connection::{RtmpConnection};
use rtmp::chunk::{Chunk};
use crate::rtmp;

use bitvec::prelude::*;

pub fn set_chunk_size(chunk: Chunk, rtmp_conn: &mut RtmpConnection) {
	let chunk_size = chunk.chunk_data().into_bytes().view_bits::<Msb0>()[0..32].load_be::<u32>();
	rtmp_conn.set_chunk_size(chunk_size);
}
pub fn abort_message(chunk: Chunk) {
	println!("Test1");
}
pub fn acknowledgement(chunk: Chunk) {
	println!("Test2");
}
pub fn window_acknowledgement_size(chunk: Chunk) {
	println!("Test3");
}
pub fn set_peer_bandwidth(chunk: Chunk) {
	println!("Test4");
}

pub fn user_control_message(chunk: Chunk) {
	println!("Test5");
}

pub fn audio_message(chunk: Chunk) {
	println!("Test6");
}
pub fn video_message(chunk: Chunk) {
	println!("Test7");
}

pub fn amf3_data_message(chunk: Chunk) {
	println!("Test8");
}
pub fn amf3_shared_object_message(chunk: Chunk) {
	println!("Test9");
}
pub fn amf3_command_message(chunk: Chunk) {
	println!("Test10");
}

pub fn amf0_data_message(chunk: Chunk) {
	println!("Test11");
}
pub fn amf0_shared_object_message(chunk: Chunk) {
	println!("Test12");
}
pub fn amf0_command_message(chunk: Chunk) {
	println!("Test13");
}

pub fn aggregate_message(chunk: Chunk) {
	println!("Test14");
}

pub fn reserved_message(chunk: Chunk) {
	println!("Test15");
}