use rtmp::chunk_stream::basic_header::{ChunkBasicHeaderOneByte, ChunkBasicHeaderTwoBytes, ChunkBasicHeaderThreeBytes};
use rtmp::chunk_stream::message_header::{ChunkMessageHeaderType0, ChunkMessageHeaderType1, ChunkMessageHeaderType2, ChunkMessageHeaderType3};
use crate::rtmp;

const _: () = assert!(size_of::<ChunkBasicHeaderOneByte>() == 1);
const _: () = assert!(size_of::<ChunkBasicHeaderTwoBytes>() == 2);
const _: () = assert!(size_of::<ChunkBasicHeaderThreeBytes>() == 3);

const _: () = assert!(size_of::<ChunkMessageHeaderType0>() == 11);
const _: () = assert!(size_of::<ChunkMessageHeaderType1>() == 7);
const _: () = assert!(size_of::<ChunkMessageHeaderType2>() == 3);
const _: () = assert!(size_of::<ChunkMessageHeaderType3>() == 0);