#![no_main]
use libfuzzer_sys::fuzz_target;

use std::process;
use kv::KVStore;
use kv::Operations;

use libfuzzer_sys::arbitrary::{Arbitrary, Result, Unstructured};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl<'a> Arbitrary<'a> for Rgb {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let r = u8::arbitrary(u)?;
        let g = u8::arbitrary(u)?;
        let b = u8::arbitrary(u)?;
        Ok(Rgb { r, g, b })
    }
}

// The fuzz target takes a well-formed `Rgb` as input!
libfuzzer_sys::fuzz_target!(|rgb: Rgb| {
    println!("{}", rgb.r);
});