#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate frown_core;
extern crate frown_p2p;

use frown_core::ser;
use frown_p2p::msg::TxHashSetArchive;

fuzz_target!(|data: &[u8]| {
	let mut d = data.clone();
	let _t: Result<TxHashSetArchive, ser::Error> = ser::deserialize(&mut d);
});
