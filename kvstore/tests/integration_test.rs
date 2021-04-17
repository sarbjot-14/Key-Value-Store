use std::process;
use kv::KVStore;
// use crate::kv::KVStore;
use kv::Operations;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[test]
fn insert_with_empty_path() {

    let owned_string = "".to_string();
    match KVStore::new(&owned_string) {
        Ok(_) => assert_eq!(false, false),
        Err(_e) => assert_eq!(true, true),
    }
}

#[test]
fn check_insert_size_update() {

    let owned_string = "./test-KV/data1".to_string();
    let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|_err| {

        process::exit(1);
    });
    kv_store.insert(String::from("Pizza"), 21 as i32).unwrap();
    assert_eq!( kv_store.size(), 1);
    kv_store.insert(String::from("Coffee"), 33 as i32).unwrap();
    assert_eq!( kv_store.size(), 2);
    kv_store.insert(String::from("Candy"), 54 as i32).unwrap();
    assert_eq!( kv_store.size(), 3);
}

#[test]
fn inserting_already_existing_key() {

    let owned_string = "./test-KV/data2".to_string();
    let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|_err| {

        process::exit(1);
    });

    kv_store.insert(String::from("Hello World"), 2 as i32).unwrap();
    match  kv_store.insert(String::from("Hello World"), 2 as i32) {
        Ok(_) => assert_eq!(false, false),
        Err(_e) => assert_eq!(true, true),
    }
}

#[test]
fn lookup_existing_key() {

    let owned_string = "./test-KV/data3".to_string();
    let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|_err| {

        process::exit(1);
    });
    kv_store.insert(String::from("Future"), 90 as i32).unwrap();
    assert_eq!( kv_store.lookup::<String, i32>(String::from("Future")).unwrap(), 90 as i32);
}

#[test]
fn lookup_non_existing_key() {

    let owned_string = "./test-KV/data4".to_string();
    let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|_err| {

        process::exit(1);
    });
    kv_store.insert(String::from("Past"), 20 as i32).unwrap();
    match  kv_store.lookup::<String, i32>(String::from("Present")) {
        Ok(_) => assert_eq!(false, false),
        Err(_e) => assert_eq!(true, true),
    }
}

#[test]
fn lookup_empty_key() {

    let owned_string = "./test-KV/data5".to_string();
    let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|_err| {

        process::exit(1);
    });
    kv_store.insert(String::from("Past"), 20 as i32).unwrap();
    match  kv_store.lookup::<String, i32>(String::from("")) {
        Ok(_) => assert_eq!(false, false),
        Err(_e) => assert_eq!(true, true),
    }
}

#[test]
fn remove_existing_key() {

    let owned_string = "./test-KV/data6".to_string();
    let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|_err| {

        process::exit(1);
    });
    kv_store.insert(String::from("Cold"), 86 as i32).unwrap();
    kv_store.insert(String::from("Water"), 90 as i32).unwrap();
    assert_eq!( kv_store.remove::<String, i32>(String::from("Water")).unwrap(), 90 as i32);
}

#[test]
fn remove_non_existing_key() {

    let owned_string = "./test-KV/data7".to_string();
    let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|_err| {

        process::exit(1);
    });
    kv_store.insert(String::from("Infinite"), 20 as i32).unwrap();
    kv_store.insert(String::from("Time"), 20 as i32).unwrap();
    match  kv_store.remove::<String, i32>(String::from("This key does not exist")) {
        Ok(_) => assert_eq!(false, false),
        Err(_e) => assert_eq!(true, true),
    }
}

#[test]
fn check_size_when_remove_existing_key() {

    let owned_string = "./test-KV/data8".to_string();
    let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|_err| {

        process::exit(1);
    });
    kv_store.insert(String::from("Sine"), 360 as i32).unwrap();
    assert_eq!( kv_store.size(), 1);
    kv_store.insert(String::from("Wave"), 180 as i32).unwrap();
    assert_eq!( kv_store.size(), 2);
    assert_eq!( kv_store.remove::<String, i32>(String::from("Sine")).unwrap(), 360 as i32);
    assert_eq!( kv_store.size(), 1);
}

#[test]
fn remove_existing_key2() {

    let owned_string = "./test-KV/data9".to_string();
    let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|_err| {

        process::exit(1);
    });
    kv_store.insert(String::from("Earth"), 77 as i32).unwrap();
    assert_eq!( kv_store.remove::<String, i32>(String::from("Earth")).unwrap(), 77 as i32);
}

#[test]
fn insert_i32() {

    let owned_string = "./test-KV/data".to_string();
    let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|_err| {

        process::exit(1);
    });

    kv_store.insert(String::from("key"), 2 as i32).unwrap();

    assert_eq!( kv_store.lookup::<String, i32>(String::from("key")).unwrap(), 2 as i32);

}

//Default
    #[derive(Serialize, Deserialize,Default,Debug)]
    struct Address {
        street: String,
        city: String,
    }

    #[test]
    fn insert_obj() {


        let address = Address {
            street: "10 Downing Street".to_owned(),
            city: "London".to_owned(),
        };

        // Serialize it to a JSON string.
        //let j = serde_json::to_string(&address).unwrap();


        let owned_string = "./test-KV/test1".to_string();
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|_err| {

            process::exit(1);
        });

        kv_store.insert(String::from("key"), address as Address).unwrap();

        let an_add:Address = kv_store.lookup::<String, Address>(String::from("key")).unwrap();

        assert_eq!( an_add.street, "10 Downing Street".to_owned());
        assert_eq!( an_add.city, "London".to_owned());
        // assert_eq!( kv_store.lookup::<String, Address>(String::from("key")).unwrap(), String::from("London") as String);

    }

/*
#[test]
fn insert_object() {
    let owned_string = "./".to_string();
    let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|_err| {

        process::exit(1);
    });

    // #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        test_id: usize,
        test_input: String,
    }

    impl<T> Default for Test {
        fn default() -> Self {
            Self { field: Default::default() }
        }
    }

    let test_obj = Test {
        test_id: 84,
        test_input: String::from("the value"),
    };

    kv_store.insert(String::from("key"), test_obj as Test).unwrap();

    let test_var:Test = kv_store.lookup::<String, Test>(String::from("key")).unwrap();
    assert_eq!( test_var.test_id, 84 as usize);
    assert_eq!( test_var.test_input, String::from("the value"));

}
*/

#[test]
fn insert_bool_true() {

    let owned_string = "./test-KV/test2".to_string();
    let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|_err| {

        process::exit(1);
    });

    let t_bool:bool = true;
    kv_store.insert(String::from("key"), t_bool as bool).unwrap();

    assert_eq!( kv_store.lookup::<String, bool>(String::from("key")).unwrap(), true);

}

#[test]
fn insert_bool_false() {

    let owned_string = "./test-KV/test3".to_string();
    let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|_err| {

        process::exit(1);
    });

    let f_bool:bool = false;
    kv_store.insert(String::from("key"), f_bool as bool).unwrap();

    assert_eq!( kv_store.lookup::<String, bool>(String::from("key")).unwrap(), false);

}

#[test]
fn insert_array() {

    let owned_string = "./test-KV/test4".to_string();
    let mut kv_store = KVStore::new(&owned_string).unwrap_or_else(|_err| {
        process::exit(1)
    });

    let _v: Vec<i32> = Vec::new();
    let _v = vec![1, 2, 3];

    kv_store.insert(String::from("key"), _v as Vec<i32>).unwrap();

    assert_eq!(kv_store.lookup::<String, Vec<i32>>(String::from("key")).unwrap(), [1, 2, 3]);
}

#[test]
fn insert_hashmap() {

    let owned_string = "./test-KV/test5".to_string();
    let mut kv_store = KVStore::new(&owned_string).unwrap_or_else(|_err| {
        process::exit(1)
    });

    let mut scores: HashMap<String, isize> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    kv_store.insert(String::from("key"), scores as HashMap<String, isize>).unwrap();

    let score_test:HashMap<String, isize> = kv_store.lookup::<String, HashMap<String, isize>>(String::from("key")).unwrap();

    assert_eq!( score_test["Blue"], 10);
    assert_eq!( score_test["Yellow"], 50);

}

#[test]
fn invalid_path_lookup() {

    let owned_string = "./test-KV/invalidfolder2".to_string();
    let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|_err| {

        process::exit(1);
    });

    kv_store.insert(String::from("key"), 3 as i32).expect("Insert Failed");

    match  kv_store.lookup::<String, i32>(String::from("key")) {
        Ok(_) => assert_eq!(false, false),
        Err(_e) => assert_eq!(true, true),
    }

}
#[test]
fn invalid_path_insert() {

    let owned_string = "./test-KV/invalidfolder".to_string();
    let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|_err| {

        process::exit(1);
    });

    match  kv_store.insert(String::from("key"), 3 as i32) {
        Ok(_) => assert_eq!(false, false),
        Err(_e) => assert_eq!(true, true),
    }


}
