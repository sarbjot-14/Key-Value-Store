use std::env;
use std::process;
use kv::KVStore;
use kv::Operations;

fn main() {
    println!("Hello, world!");
    let owned_string = "hello world".to_string(); 

   

    let kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
        eprintln!("Problem : {}", err);
        process::exit(1);
    });

    println!("Hello, world!");


}
