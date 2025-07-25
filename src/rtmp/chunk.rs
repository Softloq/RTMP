use bitvec::prelude::*;

// Size Constraints ------------------------------------------------//

const _: () = assert!(size_of::<ChunkBasicHeaderOneByte>()    == 1);
const _: () = assert!(size_of::<ChunkBasicHeaderTwoBytes>()   == 2);
const _: () = assert!(size_of::<ChunkBasicHeaderThreeBytes>() == 3);

const _: () = assert!(size_of::<ChunkMessageHeaderType0>() == 11);
const _: () = assert!(size_of::<ChunkMessageHeaderType1>() == 7);
const _: () = assert!(size_of::<ChunkMessageHeaderType2>() == 3);
const _: () = assert!(size_of::<ChunkMessageHeaderType3>() == 0);

const _: () = assert!(size_of::<ExtendedTimestamp>() == 4);

//------------------------------------------------------------------//

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

pub struct ChunkData { data: Vec<u8> }
impl ChunkData {
	pub fn from_bytes(data: Vec<u8>) -> ChunkData { ChunkData { data } }
	pub fn into_bytes(&self) -> &Vec<u8> { &self.data }
}
