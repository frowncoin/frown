#![no_main]
extern crate frown_core;
#[macro_use]
extern crate libfuzzer_sys;

use frown_core::core::transaction;
use frown_core::ser;

fuzz_target!(|data: &[u8]| {
	let mut d = data.clone();
	let _t: Result<transaction::Transaction, ser::Error> = ser::deserialize(&mut d);
});
