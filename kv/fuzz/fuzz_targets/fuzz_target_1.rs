#![no_main]
use libfuzzer_sys::fuzz_target;

use std::process;
use kv::KVStore;
use kv::Operations;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let owned_string = "data".to_string(); 
    let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
        //eprintln!("Problem : {}", err);
        process::exit(1);
    });

    match std::str::from_utf8(data) {
        Ok(s) => s,
        Err(error) => return,
    };
    let key:String = std::str::from_utf8(data).unwrap().to_string();
    let val:String = std::str::from_utf8(data).unwrap().to_string();
    let key_look:String = std::str::from_utf8(data).unwrap().to_string();
    let val_look:String = std::str::from_utf8(data).unwrap().to_string();
    let key_rem:String = std::str::from_utf8(data).unwrap().to_string();

    // let key:String = match std::str::from_utf8(data).unwrap() {
    //     Ok(str) => str,
    //     Err(error) => "Adventures of Huckleberry Finn".to_string(),
    // };
    //println!("Problem : {}", key);
    
    kv_store.insert(key, val).unwrap();

    assert_eq!( kv_store.lookup::<String, String>(key_look).unwrap(), val_look);  

    kv_store.remove::<String, String>(key_rem).unwrap();

});

