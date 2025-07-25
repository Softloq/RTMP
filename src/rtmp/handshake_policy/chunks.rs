use rand::Rng;

pub struct C0 { data: [u8; 1] }
impl C0 {
	pub fn from_bytes(data: [u8; 1]) -> C0 { C0 { data } }
	pub fn version(&self) -> u8 { self.data[0] }
}

pub struct S0 { data: [u8; 1] }
impl S0 {
	pub fn new() -> S0 { S0 { data: [0u8; 1] } }
	pub fn set_version(&mut self, version: u8) { self.data[0] = version }
	pub fn version(&self) -> u8 { self.data[0] }
	pub fn into_bytes(&self) -> [u8; 1] { self.data }
}

pub struct C1 { data: [u8; 1536] }
impl C1 {
	pub fn from_bytes(data: [u8; 1536]) -> C1 { C1 { data } }
	pub fn time(&self) -> u32 { u32::from_be_bytes(self.data[0..4].try_into().unwrap()) }
	pub fn zero(&self) -> u32 { u32::from_be_bytes(self.data[4..8].try_into().unwrap()) }
	pub fn random(&self) -> [u8; 1528] { self.data[8..1536].try_into().unwrap() }
}

pub struct S1 { data: [u8; 1536] }
impl S1 {
	pub fn new() -> S1 { S1 { data: [0u8; 1536] } }
	pub fn set_time(&mut self, time: u32) { self.data[0..4].copy_from_slice(&time.to_be_bytes()) }
	pub fn time(&self) -> u32 { u32::from_be_bytes(self.data[0..4].try_into().unwrap()) }
	pub fn set_zero(&mut self, zero: u32) { self.data[4..8].copy_from_slice(&zero.to_be_bytes()) }
	pub fn zero(&self) -> u32 { u32::from_be_bytes(self.data[4..8].try_into().unwrap()) }
	pub fn randomize(&mut self) { rand::rng().fill(&mut self.data[8..1536]) }
	pub fn random(&self) -> [u8; 1528] { self.data[8..1536].try_into().unwrap() }
	pub fn into_bytes(&self) -> [u8; 1536] { self.data }
}

pub struct C2 { data: [u8; 1536] }
impl C2 {
	pub fn from_bytes(data: [u8; 1536]) -> C2 { C2 { data } }
	pub fn time(&self) -> u32 { u32::from_be_bytes(self.data[0..4].try_into().unwrap()) }
	pub fn time2(&self) -> u32 { u32::from_be_bytes(self.data[4..8].try_into().unwrap()) }
	pub fn random_echo(&self) -> [u8; 1528] { self.data[8..1536].try_into().unwrap() }
}

pub struct S2 { data: [u8; 1536] }
impl S2 {
	pub fn new() -> S2 { S2 { data: [0u8; 1536] } }
	pub fn set_time(&mut self, time: u32) { self.data[0..4].copy_from_slice(&time.to_be_bytes()) }
	pub fn time(&self) -> u32 { u32::from_be_bytes(self.data[0..4].try_into().unwrap()) }
	pub fn set_time2(&mut self, time2: u32) { self.data[4..8].copy_from_slice(&time2.to_be_bytes()) }
	pub fn time2(&self) -> u32 { u32::from_be_bytes(self.data[4..8].try_into().unwrap()) }
	pub fn set_random_echo(&mut self, echo: [u8; 1528]) { self.data[8..1536].copy_from_slice(&echo); }
	pub fn into_bytes(&self) -> [u8; 1536] { self.data }
}