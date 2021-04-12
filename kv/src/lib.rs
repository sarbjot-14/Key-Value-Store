extern crate crypto;

use std::fmt::Debug;
use std::fs;

use std::io::{Error, ErrorKind};
use std::path::Path;
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

#[derive(Debug)]
/// A struct that represents a key-value store.
pub struct KVStore {
    /// The number of key-value mappings currently stored.
    size: usize,
    /// The location of the file system where key-value mappings are stored.
    path: String,
}

/// A trait that defines the operations that need to be supported.
pub trait Operations {
    /// A function that initializes a KVStore instance.
    ///
    /// If there is no directory at the provided path, this should create it. If there is an error
    /// while creating a directory, this should return an [std::io::Error].
    ///
    /// If there are **no** key-value mappings stored already under the directory, this
    /// should simply create a new KVStore instance that can store and retrieve key-value mappings
    /// using the directory. It should also correctly initialize the size to 0.
    ///
    /// If there **are** existing key-value mappings stored already under the directory, this
    /// should initialize a KVStore instance that is able to store and retrieve existing key-value
    /// mappings as well as new key-value mappings. It should also correctly initialize the size to
    /// the number of existing key-value mappings.
    fn new(path: &str) -> std::io::Result<Self>
    where
        Self: Sized;

    /// A function that returns the number of key-value mappings currently stored.
    fn size(self: &Self) -> usize;

    /// A function that inserts a new key-value mapping.
    ///
    /// If there is **no** key-value mapping stored already with the same key, it should return
    /// `Ok(())` if storing is successfully done.
    ///
    /// If there **is** a key-value mapping stored already with the same key, it should return an
    /// [std::io::Error].
    ///
    /// Make sure you read and understand the assignment document regarding how to store key-value
    /// mappings using files as well as how to structure sub-directories.
    ///
    /// Make sure you understand what the trait bounds mean for K and V.
    ///
    /// Refer to [https://docs.serde.rs/serde/](https://docs.serde.rs/serde/)
    /// and [https://serde.rs](https://serde.rs) for serde.
    fn insert<K, V>(self: &mut Self, key: K, value: V) -> std::io::Result<()>
    where
        K: serde::Serialize + Default + Debug,
        V: serde::Serialize + Default + Debug;

    /// A function that returns a previously-inserted value.
    ///
    /// If there **is** a key-value mapping stored already with the same key, it should return
    /// the value.
    ///
    /// If there is **no** key-value mapping stored already with the same key, it should return
    /// an [std::io::Error].
    ///
    /// Make sure you understand what the trait bounds mean for K and V.
    ///
    /// Refer to [https://docs.serde.rs/serde/](https://docs.serde.rs/serde/)
    /// and [https://serde.rs](https://serde.rs) for serde.
    fn lookup<K, V>(self: &Self, key: K) -> std::io::Result<V>
    where
        K: serde::Serialize + Default + Debug,
        V: serde::de::DeserializeOwned + Default + Debug;


    /// A function that removes a previously-inserted key-value mapping.
    ///
    /// If there **is** a key-value mapping stored already with the same key, it should return
    /// the value and delete the key-value mapping from the file system.
    ///
    /// If there is **no** key-value mapping stored already with the same key, it should
    /// return an [std::io::Error].
    ///
    /// If a sub-directory does not contain any key-value files, this should delete the
    /// sub-directory as well.
    ///
    /// Make sure you understand what the trait bounds mean for K and V.
    ///
    /// Refer to [https://docs.serde.rs/serde/](https://docs.serde.rs/serde/)
    /// and [https://serde.rs](https://serde.rs) for serde.
    fn remove<K, V>(self: &mut Self, key: K) -> std::io::Result<V>
    where
        K: serde::Serialize + Default + Debug,
        V: serde::de::DeserializeOwned + Default + Debug;
}

impl Operations for KVStore {

    fn new(path: &str) -> std::io::Result<KVStore> {
        Ok(KVStore {
            size: 0,
            path: String::from(path),
        })
    }

    fn size(&self) -> usize {
        self.size
    }

    fn insert<K, V>(self: &mut Self, key: K, value: V) -> std::io::Result<()>
        where
            K: serde::Serialize + Default + Debug,
            V: serde::Serialize + Default + Debug
    {
        let mut hasher = Sha256::new();

        //println!("{:?}, {:?}", key, value);
        let serialized_value = serde_json::to_string(&value).unwrap(); //might cause error
        let serialized_key = serde_json::to_string(&key).unwrap();

        hasher.input_str(&serialized_key); // might cause error here
        let sha_key = hasher.result_str();

        let sha_key_slice = &sha_key[0..10];

        let curr_path = &self.path;
        let key_format = ".key";
        let value_format = ".value";

        let sub_dir_path = format!("{}/{}", curr_path, sha_key_slice);
        fs::create_dir_all(&sub_dir_path).expect("Something went wrong creating the sub directory!");
        let key_file = format!("{}/{}{}", sub_dir_path, sha_key, key_format);
        let value_file = format!("{}/{}{}", sub_dir_path, sha_key, value_format);
        let key_file_path = Path::new(&key_file);
        let value_file_path = Path::new(&value_file);


        if key_file_path.is_file() {
            Error::new(ErrorKind::Other, "Key file already exists!");
        }
        if value_file_path.is_file() {
            Error::new(ErrorKind::Other, "Value file already exists!");
        }


        fs::write(key_file, serialized_key).expect("Something went wrong writing to the key file!");
        fs::write(value_file, serialized_value).expect("Something went wronng writing to the value file!");
        self.size = self.size + 1;
        
        Ok(())
    }

    fn lookup<K, V>(self: &Self, key: K) -> std::io::Result<V>
    where
        K: serde::Serialize + Default + Debug,
        V: serde::de::DeserializeOwned + Default + Debug
    {

        let mut hasher = Sha256::new();

        let serialized_key = serde_json::to_string(&key).unwrap();

        hasher.input_str(&serialized_key);
        let sha_key = hasher.result_str();
        let sha_key_slice = &sha_key[0..10];

        let curr_path = &self.path;
        let value_format = ".value";

        let sub_dir = format!("{}/{}", curr_path, sha_key_slice);
        fs::create_dir_all(&sub_dir).expect("Something went wrong creating the sub directory!");

        let value_file = format!("{}/{}{}", sub_dir, sha_key, value_format);
        let value_file_path = Path::new(&value_file);

        if !(value_file_path.is_file()) {
            Error::new(ErrorKind::Other, "Value file does not exist!");
        }
        
        let value = fs::read_to_string(value_file)
        .expect("Something went wrong reading the value file!");

        println!("it is {}", value);

        Ok(serde_json::from_str(&value).unwrap())
    }

    fn remove<K, V>(self: &mut Self, key: K) -> std::io::Result<V>
    where
        K: serde::Serialize + Default + Debug,
        V: serde::de::DeserializeOwned + Default + Debug
    {
        let mut hasher = Sha256::new();
        let serialized_key = serde_json::to_string(&key).unwrap();
        hasher.input_str(&serialized_key);

        let sha_key = hasher.result_str();
        let sha_key_slice = &sha_key[0..10];

        let curr_path = &self.path;
        let key_format = ".key";
        let value_format = ".value";

        let sub_dir = format!("{}/{}", curr_path, sha_key_slice);
        let key_file = format!("{}/{}{}", sub_dir, sha_key, key_format);
        let value_file = format!("{}/{}{}", sub_dir, sha_key, value_format);

        let sub_dir_path = Path::new(&sub_dir);
        let key_file_path = Path::new(&key_file);
        let value_file_path = Path::new(&value_file);

        if !(sub_dir_path.is_dir()) {
            Error::new(ErrorKind::Other, "Sub directory does not exist!");
        }
        if !(key_file_path.is_file()) {
            Error::new(ErrorKind::Other, "Key file does not exist!");
        }
        if !(value_file_path.is_file()) {
            Error::new(ErrorKind::Other, "Value file does not exist!");
        }

        let value = fs::read_to_string(value_file_path)
        .expect("Something went wrong reading the value file!");

        fs::remove_file(&key_file).expect("Something went wrong removing the key file!");
        fs::remove_file(&value_file).expect("Something went wrong removing the value file!");
        self.size = self.size - 1;

        if sub_dir_path.read_dir()?.next().is_none() {
            fs::remove_dir_all(&sub_dir_path).expect("Something went wrong removing the sub directory!");
        }

        Ok(serde_json::from_str(&value).unwrap())
    }
}

#[cfg(test)]
mod tests {
use std::process;
use super::KVStore;
use super::Operations;
use std::fs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

    #[test]
    fn insert_i32_1() {
        let owned_string = ".".to_string(); 
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            eprintln!("Problem : {}", err);
            process::exit(1);
        });
        kv_store.insert(String::from("Test String 1 - Key"), 1 as i32).unwrap();
        kv_store.insert(String::from("Test String 2 - Key"), 2 as i32).unwrap();
        kv_store.insert(String::from("Test String 3 - Key"), 3 as i32).unwrap();
        println!("size : {}", kv_store.size());
        assert_eq!( kv_store.size(), 3);
        //kv_store.insert(String::from("Test String 1 - Key"), 2 as i32).unwrap();
        //assert_eq!( kv_store.lookup::<String, i32>(String::from("Test String 1 - Key")).unwrap(), 2 as i32);
    }

    #[test]
    fn insert_i32_101() {
        let owned_string = ".".to_string(); 
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            eprintln!("Problem : {}", err);
            process::exit(1);
        });

        println!("size : {}", kv_store.size());
        //assert_eq!( kv_store.size(), 0);
        assert_eq!( kv_store.lookup::<String, i32>(String::from("Test String 1 - Key")).unwrap(), 1 as i32);
        assert_eq!( kv_store.size(), 0);
        //kv_store.insert(String::from("Test String 1 - Key"), 2 as i32).unwrap();
        //assert_eq!( kv_store.lookup::<String, i32>(String::from("Test String 1 - Key")).unwrap(), 2 as i32);
    }

    #[test]
    fn invalid_path_lookup() {
        let owned_string = "./invalidfolder".to_string(); 
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            //eprintln!("Problem : {}", err);
            process::exit(1);
        });

        kv_store.insert(String::from("key"), 2 as i32).expect("Insert Failed");

        match  kv_store.lookup::<String, i32>(String::from("key")) {
            Ok(_) => assert_eq!(false, false),
            Err(e) => assert_eq!(true, true),
        }
           
    }
    #[test]
    fn invalid_path_insert() {
        let owned_string = "./invalidfolder".to_string(); 
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            //eprintln!("Problem : {}", err);
            process::exit(1);
        });

        match  kv_store.insert(String::from("key"), 2 as i32) {
            Ok(_) => assert_eq!(false, false),
            Err(e) => assert_eq!(true, true),
        }
           
    }

    #[test]
    fn inserting_already_existing_key() {
        let owned_string = ".".to_string(); 
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            //eprintln!("Problem : {}", err);
            process::exit(1);
        });

        kv_store.insert(String::from("Hello World"), 2 as i32).unwrap();
        match  kv_store.insert(String::from("Hello World"), 2 as i32) {
            Ok(_) => assert_eq!(false, false),
            Err(e) => assert_eq!(true, true),
        }  
    }


    #[test]
    fn insert_string() {
        let owned_string = ".".to_string(); 
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            //eprintln!("Problem : {}", err);
            process::exit(1);
        });


        // Review some books.
        kv_store.insert(
            "Adventures of Huckleberry Finn".to_string(),
            "My favorite book.".to_string(),
        ).unwrap();
        // use lookup function eventually to see if it is correct instead of reading from file?
        let value = fs::read_to_string("src/foo_value.txt")
        .expect("Something went wrong reading the value file!");

        let deserialized_value:String = serde_json::from_str(&value).unwrap();

        assert_eq!(deserialized_value , "My favorite book.".to_string());
        // key
        let key = fs::read_to_string("src/foo_key.txt")
        .expect("Something went wrong reading the key file!");

        let deserialized_key:String = serde_json::from_str(&key).unwrap();

        assert_eq!(deserialized_key , "Adventures of Huckleberry Finn".to_string());
    }

    fn print_type_of<T>(_: &T) {

        if std::any::type_name::<T>() == "i32"{
            //println!("it is {}", std::any::type_name::<T>());

        }
    }

    #[test]
    fn insert_i32() {
        let owned_string = "data".to_string(); 
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            //eprintln!("Problem : {}", err);
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


            let owned_string = "./test".to_string();
            let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
                //eprintln!("Problem : {}", err);
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
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            //eprintln!("Problem : {}", err);
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
    fn insert_bool() {
        let owned_string = "./tests".to_string();
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            //eprintln!("Problem : {}", err);
            process::exit(1);
        });

        let t_bool:bool = true;
        kv_store.insert(String::from("key"), t_bool as bool).unwrap();

        assert_eq!( kv_store.lookup::<String, bool>(String::from("key")).unwrap(), true);

        let f_bool:bool = false;
        kv_store.insert(String::from("key"), f_bool as bool).unwrap();

        assert_eq!( kv_store.lookup::<String, bool>(String::from("key")).unwrap(), false);

    }

    #[test]
    fn insert_array() {
        let owned_string = "./t".to_string();
        let mut kv_store = KVStore::new(&owned_string).unwrap_or_else(|err| {
            process::exit(1)
        });

        let v: Vec<i32> = Vec::new();
        let v = vec![1, 2, 3];

        kv_store.insert(String::from("key"), v as Vec<i32>).unwrap();

        assert_eq!(kv_store.lookup::<String, Vec<i32>>(String::from("key")).unwrap(), [1, 2, 3]);
    }

    #[test]
    fn insert_hashmap() {
        let owned_string = "./te".to_string();
        let mut kv_store = KVStore::new(&owned_string).unwrap_or_else(|err| {
            process::exit(1)
        });

        let mut scores: HashMap<String, isize> = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        kv_store.insert(String::from("key"), scores as HashMap<String, isize>).unwrap();

        let score_test:HashMap<String, isize> = kv_store.lookup::<String, HashMap<String, isize>>(String::from("key")).unwrap();

        assert_eq!( score_test["Blue"], 10);

    }

    #[test]
    fn invalid_path_lookup() {
        let owned_string = "./invalidfolder".to_string();
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            //eprintln!("Problem : {}", err);
            process::exit(1);
        });

        kv_store.insert(String::from("key"), 3 as i32).expect("Insert Failed");

        match  kv_store.lookup::<String, i32>(String::from("key")) {
            Ok(_) => assert_eq!(false, false),
            Err(e) => assert_eq!(true, true),
        }

    }
    #[test]
    fn invalid_path_insert() {
        let owned_string = "./invalidfolder".to_string();
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            //eprintln!("Problem : {}", err);
            process::exit(1);
        });

        match  kv_store.insert(String::from("key"), 3 as i32) {
            Ok(_) => assert_eq!(false, false),
            Err(e) => assert_eq!(true, true),
        }


    }
}
