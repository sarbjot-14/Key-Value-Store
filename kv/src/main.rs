use std::process;
use kv::KVStore;
use kv::Operations;
use std::io::{stdin,stdout,Write};

fn main() {
    println!("Hello, world!");

    let owned_string = "/random/path".to_string(); 
    let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
        eprintln!("Problem : {}", err);
        process::exit(1);
    });

    // you dont have to use this, most of the testing should be done in the unit tests
    let mut end_program = true;
    while end_program{
        let mut command=String::new();
        print!("Please enter command (insert, exit): ");
        let _=stdout().flush();
        stdin().read_line(&mut command).expect("Did not enter a correct string");
        if let Some('\n')=command.chars().next_back() {
            command.pop();
        }
        if let Some('\r')=command.chars().next_back() {
            command.pop();
        }
        println!("You typed: {}",command);
        if command.eq(&String::from("exit")){
            end_program = false;
            continue;
        }

        let mut key=String::new();
        print!("Please enter key: ");
        let _=stdout().flush();
        stdin().read_line(&mut key).expect("Did not enter a correct string");
        if let Some('\n')=key.chars().next_back() {
            key.pop();
        }
        if let Some('\r')=key.chars().next_back() {
            key.pop();
        }
        println!("You typed: {}",key);

        let mut value=String::new();

        print!("Please enter value: ");
        let _=stdout().flush();
        stdin().read_line(&mut value).expect("Did not enter a correct string");
        if let Some('\n')=value.chars().next_back() {
            value.pop();
        }
        if let Some('\r')=value.chars().next_back() {
            value.pop();
        }
        println!("You typed: {}",value);

        kv_store.insert(key,value).unwrap_or_else(|err| {
            eprintln!("Problem : {}", err);
            process::exit(1);
        });
        //fn insert<K, V>(self: &Self, key: K, value: V) -> std::io::Result<V>

    }
    
  


}
