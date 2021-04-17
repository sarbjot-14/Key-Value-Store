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
    // The number of key-value mappings currently stored.
    size: usize,
    // The location of the file system where key-value mappings are stored.
    path: String,
}

/// A trait that defines the operations that need to be supported.
pub trait Operations {
    // A function that initializes a KVStore instance.
    fn new(path: &str) -> std::io::Result<Self>
    where
        Self: Sized;

    // A function that returns the number of key-value mappings currently stored.
    fn size(self: &Self) -> usize;

    // A function that inserts a new key-value mapping.
    fn insert<K, V>(self: &mut Self, key: K, value: V) -> std::io::Result<()>
    where
        K: serde::Serialize + Default + Debug,
        V: serde::Serialize + Default + Debug;

    // A function that returns a previously-inserted value.
    fn lookup<K, V>(self: &Self, key: K) -> std::io::Result<V>
    where
        K: serde::Serialize + Default + Debug,
        V: serde::de::DeserializeOwned + Default + Debug;

    // A function that removes a previously-inserted key-value mapping.
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
            Err(_e) => return Err(Error::new(ErrorKind::Other, "Something went wrong creating the sub directory!")),
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
            Err(_e) => return Err(Error::new(ErrorKind::Other, "Something went wrong creating the sub directory!")),
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
            Err(_e) => return Err(Error::new(ErrorKind::Other, "Something went wrong writing to the key file!")),
            _ => (),
        };
        match fs::write(value_file_path, serialized_value) {
            Err(_e) => return Err(Error::new(ErrorKind::Other, "Something went wronng writing to the value file!")),
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
            Err(_e) => return Err(Error::new(ErrorKind::Other, "Something went wrong creating the sub directory!")),
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
            Err(_e) => return Err(Error::new(ErrorKind::Other, "Something went wrong creating the sub directory!")),
            _ => value = fs::read_to_string(value_file_path)?,
        };

        match fs::remove_file(key_file_path) {
            Err(_e) => return Err(Error::new(ErrorKind::Other, "Something went wrong removing the key file!")),
            _ => (),
        };
        match fs::remove_file(value_file_path) {
            Err(_e) => return Err(Error::new(ErrorKind::Other, "Something went wrong removing the value file!")),
            _ => (),
        };
        self.size = self.size - 1;

        if sub_dir_path.read_dir()?.next().is_none() {
            match fs::remove_dir_all(&sub_dir_path) {
                Err(_e) => return Err(Error::new(ErrorKind::Other, "Something went wrong removing the sub directory!")),
                _ => (),
            };
        }

        Ok(serde_json::from_str(&value).unwrap())
    }
}
