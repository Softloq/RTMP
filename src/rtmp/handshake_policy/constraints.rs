use rtmp::handshake_policy::chunks::{C0, C1, C2, S0, S1, S2};
use crate::rtmp;

const _: () = assert!(size_of::<C0>() == 1);
const _: () = assert!(size_of::<C1>() == 1536);
const _: () = assert!(size_of::<C2>() == 1536);

const _: () = assert!(size_of::<S0>() == 1);
const _: () = assert!(size_of::<S1>() == 1536);
const _: () = assert!(size_of::<S2>() == 1536);
