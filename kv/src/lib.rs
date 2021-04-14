extern crate crypto;

use std::fmt::Debug;
use std::fs;

use std::io::{Error, ErrorKind};
use std::path::Path;
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;
use walkdir::WalkDir;


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

        let curr_path = &path;
        let sub_dir_path = Path::new(&path);
        match fs::create_dir_all(&sub_dir_path) {
            Err(e) => return Err(Error::new(ErrorKind::Other, "Something went wrong creating the sub directory!")),
            _ => (),
        };

        let mut count = 0;
        for entry in WalkDir::new(curr_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
                let file_name = entry.file_name().to_string_lossy();
                if file_name.ends_with(".key"){
                    println!("{}", file_name);
                    count = count + 1;
                }
        }
        println!("Key file count: {}", count);
        Ok(KVStore {
            size: count,
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

        let serialized_value = serde_json::to_string(&value).unwrap();
        let serialized_key = serde_json::to_string(&key).unwrap();

        hasher.input_str(&serialized_key);
        let sha_key = hasher.result_str();

        let sha_key_slice = &sha_key[0..10];

        let curr_path = &self.path;
        let key_format = ".key";
        let value_format = ".value";

        let sub_dir_str = format!("{}/{}", curr_path, sha_key_slice);
        let sub_dir_path = Path::new(&sub_dir_str);

        match fs::create_dir_all(&sub_dir_path) {
            Err(e) => return Err(Error::new(ErrorKind::Other, "Something went wrong creating the sub directory!")),
            _ => (),
        };

        let key_file = format!("{}/{}{}", sub_dir_str, sha_key, key_format);
        let value_file = format!("{}/{}{}", sub_dir_str, sha_key, value_format);
        let key_file_path = Path::new(&key_file);
        let value_file_path = Path::new(&value_file);

        if key_file_path.is_file() {
            return Err(Error::new(ErrorKind::Other, "Key file already exists!"));
        }
        if value_file_path.is_file() {
            return Err(Error::new(ErrorKind::Other, "Value file already exists!"));
        }

        match fs::write(key_file_path, serialized_key) {
            Err(e) => return Err(Error::new(ErrorKind::Other, "Something went wrong writing to the key file!")),
            _ => (),
        };
        match fs::write(value_file_path, serialized_value) {
            Err(e) => return Err(Error::new(ErrorKind::Other, "Something went wronng writing to the value file!")),
            _ => (),
        };
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

        let value_file = format!("{}/{}{}", sub_dir, sha_key, value_format);
        let value_file_path = Path::new(&value_file);

        if !(value_file_path.is_file()) {
            return Err(Error::new(ErrorKind::Other, "Value file does not exist!"));
        }

        let value;
        match fs::read_to_string(value_file_path) {
            Err(e) => return Err(Error::new(ErrorKind::Other, "Something went wrong creating the sub directory!")),
            _ => value = fs::read_to_string(value_file_path)?,
        };

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
            return Err(Error::new(ErrorKind::Other, "Sub directory does not exist!"));
        }
        if !(key_file_path.is_file()) {
            return Err(Error::new(ErrorKind::Other, "Key file does not exist!"));
        }
        if !(value_file_path.is_file()) {
            return Err(Error::new(ErrorKind::Other, "Value file does not exist!"));
        }

        let value;
        match fs::read_to_string(value_file_path) {
            Err(e) => return Err(Error::new(ErrorKind::Other, "Something went wrong creating the sub directory!")),
            _ => value = fs::read_to_string(value_file_path)?,
        };

        match fs::remove_file(key_file_path) {
            Err(e) => return Err(Error::new(ErrorKind::Other, "Something went wrong removing the key file!")),
            _ => (),
        };
        match fs::remove_file(value_file_path) {
            Err(e) => return Err(Error::new(ErrorKind::Other, "Something went wrong removing the value file!")),
            _ => (),
        };
        self.size = self.size - 1;

        if sub_dir_path.read_dir()?.next().is_none() {
            match fs::remove_dir_all(&sub_dir_path) {
                Err(e) => return Err(Error::new(ErrorKind::Other, "Something went wrong removing the sub directory!")),
                _ => (),
            };
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
    fn insert_with_empty_path() {
        let owned_string = "".to_string();
        match KVStore::new(&owned_string) {
            Ok(_) => assert_eq!(false, false),
            Err(e) => assert_eq!(true, true),
        }
    }

    #[test]
    fn check_insert_size_update() {
        let owned_string = "data1".to_string();
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            eprintln!("Problem : {}", err);
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
        let owned_string = "data2".to_string();
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            eprintln!("Problem : {}", err);
            process::exit(1);
        });

        kv_store.insert(String::from("Hello World"), 2 as i32).unwrap();
        match  kv_store.insert(String::from("Hello World"), 2 as i32) {
            Ok(_) => assert_eq!(false, false),
            Err(e) => assert_eq!(true, true),
        }
    }

    #[test]
    fn lookup_existing_key() {
        let owned_string = "data3".to_string();
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            eprintln!("Problem : {}", err);
            process::exit(1);
        });
        kv_store.insert(String::from("Future"), 90 as i32).unwrap();
        assert_eq!( kv_store.lookup::<String, i32>(String::from("Future")).unwrap(), 90 as i32);
    }

    #[test]
    fn lookup_non_existing_key() {
        let owned_string = "data4".to_string();
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            eprintln!("Problem : {}", err);
            process::exit(1);
        });
        kv_store.insert(String::from("Past"), 20 as i32).unwrap();
        match  kv_store.lookup::<String, i32>(String::from("Present")) {
            Ok(_) => assert_eq!(false, false),
            Err(e) => assert_eq!(true, true),
        }
    }

    #[test]
    fn lookup_empty_key() {
        let owned_string = "data5".to_string();
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            eprintln!("Problem : {}", err);
            process::exit(1);
        });
        kv_store.insert(String::from("Past"), 20 as i32).unwrap();
        match  kv_store.lookup::<String, i32>(String::from("")) {
            Ok(_) => assert_eq!(false, false),
            Err(e) => assert_eq!(true, true),
        }
    }

    #[test]
    fn remove_existing_key() {
        let owned_string = "data6".to_string();
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            eprintln!("Problem : {}", err);
            process::exit(1);
        });
        kv_store.insert(String::from("Cold"), 86 as i32).unwrap();
        kv_store.insert(String::from("Water"), 90 as i32).unwrap();
        assert_eq!( kv_store.remove::<String, i32>(String::from("Water")).unwrap(), 90 as i32);
    }

    #[test]
    fn remove_non_existing_key() {
        let owned_string = "data7".to_string();
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            eprintln!("Problem : {}", err);
            process::exit(1);
        });
        kv_store.insert(String::from("Infinite"), 20 as i32).unwrap();
        kv_store.insert(String::from("Time"), 20 as i32).unwrap();
        match  kv_store.remove::<String, i32>(String::from("This key does not exist")) {
            Ok(_) => assert_eq!(false, false),
            Err(e) => assert_eq!(true, true),
        }
    }

    #[test]
    fn check_size_when_remove_existing_key() {
        let owned_string = "data8".to_string();
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            eprintln!("Problem : {}", err);
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
        let owned_string = "data9".to_string();
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            eprintln!("Problem : {}", err);
            process::exit(1);
        });
        kv_store.insert(String::from("Earth"), 77 as i32).unwrap();
        assert_eq!( kv_store.remove::<String, i32>(String::from("Earth")).unwrap(), 77 as i32);
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


            let owned_string = "./test1".to_string();
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
    fn insert_bool_true() {
        let owned_string = "./test2".to_string();
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            //eprintln!("Problem : {}", err);
            process::exit(1);
        });

        let t_bool:bool = true;
        kv_store.insert(String::from("key"), t_bool as bool).unwrap();

        assert_eq!( kv_store.lookup::<String, bool>(String::from("key")).unwrap(), true);

    }

    #[test]
    fn insert_bool_false() {
        let owned_string = "./test3".to_string();
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            //eprintln!("Problem : {}", err);
            process::exit(1);
        });

        let f_bool:bool = false;
        kv_store.insert(String::from("key"), f_bool as bool).unwrap();

        assert_eq!( kv_store.lookup::<String, bool>(String::from("key")).unwrap(), false);

    }

    #[test]
    fn insert_array() {
        let owned_string = "./test4".to_string();
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
        let owned_string = "./test5".to_string();
        let mut kv_store = KVStore::new(&owned_string).unwrap_or_else(|err| {
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
