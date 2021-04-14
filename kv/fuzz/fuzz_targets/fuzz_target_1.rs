#![no_main]
use libfuzzer_sys::fuzz_target;

use std::process;
use kv::KVStore;
use kv::Operations;

use libfuzzer_sys::arbitrary::{Arbitrary, Result, Unstructured};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Random {
    pub value: u8,
    pub key: i32,
 
}

impl<'a> Arbitrary<'a> for Random {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let value = u8::arbitrary(u)?;
        let key = i32::arbitrary(u)?;
        Ok(Random {value, key})
    }
}

libfuzzer_sys::fuzz_target!(|random: Random| {

    let owned_string = "./delete".to_string(); 
    let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
        process::exit(1);
    });

    // check size, because store keys could already exist
    let store_size:usize = kv_store.size();
    assert!(store_size >= 0 as usize );

    // if odd insert, else delete 
    let mut insert_count:usize = 0;
    let mut delete_count:usize = 0;
    // keep the number of subdirecteries under 20 because we want to test it trying to delete with empty folder
    let moded_key:i32 = random.key % 20;  
    if random.key != 0{ // ignore 0's because there is too many of them
        if random.key % 2 == 1  {
            //println!("Odd!!! : {}", random.key);
            match kv_store.insert(moded_key, random.value as u8) {
                Err(error) => (),
                Ok(_) => {
                    // count how many successful inserts
                    insert_count = insert_count + 1;
                    // make sure insert was right
                    assert_eq!( kv_store.lookup::<i32, u8>(moded_key).unwrap(), random.value as u8);
                    
                }
            };
        }
        else{
            //println!("Even!!! : {}", random.key);
            match kv_store.remove::<i32, u8>(moded_key) {
                Err(error) => (),
                Ok(_) => {
                    // count delete
                    delete_count = delete_count + 1;
                    
                }
        
            };

        }
        // make sure changes kept size positive
        assert!(store_size+insert_count -delete_count >= 0 as usize );

        // make sure changes made reflect what is in files
        let new_store_size:usize = kv_store.size();
        assert_eq!(new_store_size,store_size+insert_count -delete_count );

    }
    
});