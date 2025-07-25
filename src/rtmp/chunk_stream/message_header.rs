use bitvec::prelude::*;

pub enum ChunkMessageType {
	UndefinedType = 0,

	SetChunkSize = 1,
	AbortMessage = 2,
	Acknowledgement = 3,
	WindowAcknowledgementSize = 5,
	SetPeerBandwidth = 6,

	UserControlMessage = 4,

	AudioMessage = 8,
	VideoMessage = 9,

	AMF3DataMessage = 15,
	AMF3SharedObjectMessage = 16,
	AMF3CommandMessage = 17,

	AMF0DataMessage = 18,
	AMF0SharedObjectMessage = 19,
	AMF0CommandMessage = 20,

	AggregateMessage = 22,

	ReservedMessage = 7 // For Future use
}

pub enum ChunkMessageHeader {
	Type0 { type0: ChunkMessageHeaderType0 },
	Type1 { type1: ChunkMessageHeaderType1 },
	Type2 { type2: ChunkMessageHeaderType2 },
	Type3 { type3: ChunkMessageHeaderType3 }
}
impl ChunkMessageHeader {
	pub fn timestamp(&self) -> u32 {
		match self {
			ChunkMessageHeader::Type0{type0} => {
				let bits = type0.data.view_bits::<Msb0>();
				bits[0..24].load_be::<u32>()
			},
			ChunkMessageHeader::Type1{type1} => {
				let bits = type1.data.view_bits::<Msb0>();
				bits[0..24].load_be::<u32>()
			},
			ChunkMessageHeader::Type2{type2} => {
				let bits = type2.data.view_bits::<Msb0>();
				bits[0..24].load_be::<u32>()
			},
			ChunkMessageHeader::Type3{type3: _type3} => { 0 }
		}
	}
	pub fn message_length(&self) -> u32 {
		match self {
			ChunkMessageHeader::Type0{type0} => {
				let bits = type0.data.view_bits::<Msb0>();
				bits[24..48].load_be::<u32>()
			},
			ChunkMessageHeader::Type1{type1} => {
				let bits = type1.data.view_bits::<Msb0>();
				bits[24..48].load_be::<u32>()
			},
			ChunkMessageHeader::Type2{type2: _type2} => { 0 },
			ChunkMessageHeader::Type3{type3: _type3} => { 0 }
		}
	}
	pub fn message_type_id(&self) -> ChunkMessageType {
		let type_value: u32; 
		match self {
			ChunkMessageHeader::Type0{type0} => {
				let bits = type0.data.view_bits::<Msb0>();
				type_value = bits[48..56].load_be::<u32>();
			},
			ChunkMessageHeader::Type1{type1} => {
				let bits = type1.data.view_bits::<Msb0>();
				type_value = bits[48..56].load_be::<u32>();
			},
			ChunkMessageHeader::Type2{type2: _type2} => { type_value = 0; },
			ChunkMessageHeader::Type3{type3: _type3} => { type_value = 0; }
		}
		match type_value {
			1 => ChunkMessageType::SetChunkSize,
			2 => ChunkMessageType::AbortMessage,
			3 => ChunkMessageType::Acknowledgement,
			5 => ChunkMessageType::WindowAcknowledgementSize,
			6 => ChunkMessageType::SetPeerBandwidth,

			4 => ChunkMessageType::UserControlMessage,

			8 => ChunkMessageType::AudioMessage,
			9 => ChunkMessageType::VideoMessage,

			15 => ChunkMessageType::AMF3DataMessage,
			16 => ChunkMessageType::AMF3SharedObjectMessage,
			17 => ChunkMessageType::AMF3CommandMessage,

			18 => ChunkMessageType::AMF0DataMessage,
			19 => ChunkMessageType::AMF0SharedObjectMessage,
			20 => ChunkMessageType::AMF0CommandMessage,

			22 => ChunkMessageType::AggregateMessage,

			7 => ChunkMessageType::ReservedMessage,

			_ => ChunkMessageType::UndefinedType
		}
	}
	pub fn message_stream_id(&self) -> u32 {
		match self {
			ChunkMessageHeader::Type0{type0} => {
				let bits = type0.data.view_bits::<Msb0>();
				bits[56..88].load_le::<u32>()
			},
			ChunkMessageHeader::Type1{type1: _type1} => { 0 },
			ChunkMessageHeader::Type2{type2: _type2} => { 0 },
			ChunkMessageHeader::Type3{type3: _type3} => { 0 }
		}
	}
}

pub struct  ChunkMessageHeaderType0 { data: [u8; 11] }
impl ChunkMessageHeaderType0  {
	pub fn from_bytes(data: [u8; 11]) -> ChunkMessageHeaderType0  { ChunkMessageHeaderType0  { data } }
}

pub struct  ChunkMessageHeaderType1 { data: [u8; 7] }
impl ChunkMessageHeaderType1  {
	pub fn from_bytes(data: [u8; 7]) -> ChunkMessageHeaderType1  { ChunkMessageHeaderType1  { data } }
}

pub struct  ChunkMessageHeaderType2 { data: [u8; 3] }
impl ChunkMessageHeaderType2  {
	pub fn from_bytes(data: [u8; 3]) -> ChunkMessageHeaderType2  { ChunkMessageHeaderType2  { data } }
}

pub struct ChunkMessageHeaderType3 {}
