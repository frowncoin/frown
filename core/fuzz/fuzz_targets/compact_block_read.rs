#![no_main]
extern crate frown_core;
#[macro_use]
extern crate libfuzzer_sys;

use frown_core::core::block;
use frown_core::ser;

fuzz_target!(|data: &[u8]| {
	let mut d = data.clone();
	let _t: Result<block::CompactBlock, ser::Error> = ser::deserialize(&mut d);
});
