# Key-Value-Store

Implement a Key-Value store (Hashmap) in Rust. Create unit tests and fuzzing tests to test the project. 
* function that returns the number of key-value mappings currently stored. (fn size)
* function that inserts a new key-value mapping. (fn insert)
* function that returns a previously-inserted value. (fn lookup)
* function that removes a previously-inserted key-value mapping.


## Tech Stack
* **Rust** : Traits, Error handling, ect
* **Fuzzing Library** : create fuzz tests
* **SHA256 Library** : hash the keys


## Reflection

### Learning Outcomes
1. Test Driven Development
2. Learned about Rust (Traits, Error handling, ect)
3. Fuzz testing :  software testing technique that involves providing invalid, unexpected, or random data as inputs

### Unexpected Obstacles
* Needed to create scripts to clean up after fuzz testing which creates a lot of outputs

## Available Scripts
Unit Tests:
1. Go to kv folder
2. Run: `cargo test`

Fuzz Tests:
1. Go to kv folder
2. Run: `cargo fuzz run fuzz_target_1 -- -runs=100000` 



