use bitvec::prelude::*;

pub enum ChunkFormat { Type0, Type1, Type2, Type3 }

pub enum ChunkBasicHeader {
	OneByte { one_byte: ChunkBasicHeaderOneByte },
	TwoBytes { two_bytes: ChunkBasicHeaderTwoBytes },
	ThreeBytes { three_bytes: ChunkBasicHeaderThreeBytes }
}
impl ChunkBasicHeader {
	pub fn chunk_format(&self) -> ChunkFormat {
		let bits: &BitSlice<u8, Msb0>;
		match self {
			ChunkBasicHeader::OneByte{one_byte} => { bits = one_byte.data.view_bits::<Msb0>(); },
			ChunkBasicHeader::TwoBytes{two_bytes} => { bits = two_bytes.data.view_bits::<Msb0>(); },
			ChunkBasicHeader::ThreeBytes{three_bytes} => { bits = three_bytes.data.view_bits::<Msb0>(); }
		}

		match bits[0..2].load_be::<u8>() {
			0 => { ChunkFormat::Type0 },
			1 => { ChunkFormat::Type1 },
			2 => { ChunkFormat::Type2 },
			3 => { ChunkFormat::Type3 },
			_ => { ChunkFormat::Type3 }
		}
	}

	pub fn chunk_stream_id(&self) -> u32 {
		match self {
			ChunkBasicHeader::OneByte{one_byte} => {
				let bits = one_byte.data.view_bits::<Msb0>();
				bits[2..8].load_be::<u32>()
			},
			ChunkBasicHeader::TwoBytes{two_bytes} => {
				let bits = two_bytes.data.view_bits::<Msb0>();
				bits[8..16].load_be::<u32>() + 64
			},
			ChunkBasicHeader::ThreeBytes{three_bytes} => {
				let bits = three_bytes.data.view_bits::<Msb0>();
				(bits[16..24].load_be::<u32>() * 256) + (bits[8..16].load_be::<u32>() + 64)
			}
		}
	}
}

pub struct ChunkBasicHeaderOneByte { data: [u8; 1] }
impl ChunkBasicHeaderOneByte {
	pub fn from_bytes(data: [u8; 1]) -> ChunkBasicHeaderOneByte { ChunkBasicHeaderOneByte { data } }
}

pub struct ChunkBasicHeaderTwoBytes { data: [u8; 2] }
impl ChunkBasicHeaderTwoBytes {
	pub fn from_bytes(data: [u8; 2]) -> ChunkBasicHeaderTwoBytes { ChunkBasicHeaderTwoBytes { data } }
}

pub struct ChunkBasicHeaderThreeBytes { data: [u8; 3] }
impl ChunkBasicHeaderThreeBytes {
	pub fn from_bytes(data: [u8; 3]) -> ChunkBasicHeaderThreeBytes { ChunkBasicHeaderThreeBytes { data } }
}
