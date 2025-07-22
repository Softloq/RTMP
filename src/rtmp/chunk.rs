
struct BasicHeader {

}

struct MessageHeader {

}

struct ExtendedTimestamp {

}

struct Header {
	basic_header: BasicHeader,
	message_header: ChunkMessageHeader,
	extended_timestamp: ExtendedTimestamp
}

struct Data {

}

struct Chunk {
	header: Header,
	data: Data
}

struct C0 {

}

struct S0 {

}

struct C1 {

}

struct S1 {

}

struct C2 {

}

struct S2 {

}