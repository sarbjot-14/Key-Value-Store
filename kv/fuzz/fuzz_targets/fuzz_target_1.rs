#![no_main]
use libfuzzer_sys::fuzz_target;

use std::process;
use kv::KVStore;
use kv::Operations;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let owned_string = "./".to_string(); 
    let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
        //eprintln!("Problem : {}", err);
        process::exit(1);
    });

    kv_store.insert(String::from("key"), 2 as i32).unwrap();
});
