use std::fmt::Debug;
use std::fs::File;
use std::fs;


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
    /// If there is **no** key-value mapping stored already with the same key, it should return a
    /// Result that contains the value being asked to be inserted.
    ///
    /// If there **is** a key-value mapping stored already with the same key, it should first read
    /// the existing value, overwrite the existing value with the new value, and return a Result
    /// that contains the **existing** value.
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
    

  
}

impl Operations for KVStore {
    fn new(path: &str) -> std::io::Result<KVStore> {
        Ok(KVStore {
            size: 0,
            path: String::from(path),
        })
    }

    fn size(&self) -> usize {
        0
    }
    fn insert<K, V>(self: &mut Self, key: K, value: V) -> std::io::Result<()>
        where
            K: serde::Serialize + Default + Debug,
            V: serde::Serialize + Default + Debug
    {
        println!("{:?}, {:?}", key, value);
        let serialized_value = serde_json::to_string(&value).unwrap();
        let serialized_key = serde_json::to_string(&key).unwrap();
        // is there better way to read and write to file?
        // should be hashing then creating/overwriting correct file in sub files

        fs::write("src/foo_value.txt", serialized_value).expect("Unable to write file");
        fs::write("src/foo_key.txt", serialized_key).expect("Unable to write file");
        
        Ok(())
        
    }
}

#[cfg(test)]
mod tests {
use std::process;
use super::KVStore;
use super::Operations;
use std::fs;

    #[test]
    fn insert_string() {
        let owned_string = "/random/path".to_string(); 
        let mut kv_store =  KVStore::new(&owned_string).unwrap_or_else(|err| {
            eprintln!("Problem : {}", err);
            process::exit(1);
        });


        // Review some books.
        kv_store.insert(
            "Adventures of Huckleberry Finn".to_string(),
            "My favorite book.".to_string(),
        ).unwrap();
        // use lookup function eventually to see if it is correct instead of reading from file?
        let value = fs::read_to_string("src/foo_value.txt")
        .expect("Something went wrong reading the file");

        let deserialized_value:String = serde_json::from_str(&value).unwrap();

        assert_eq!(deserialized_value , "My favorite book.".to_string());  
        // key
        let key = fs::read_to_string("src/foo_key.txt")
        .expect("Something went wrong reading the file");

        let deserialized_key:String = serde_json::from_str(&key).unwrap();

        assert_eq!(deserialized_key , "Adventures of Huckleberry Finn".to_string());  
    }
}