use std::collections::HashMap;
use std::option::*;
use std::string::*;

#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            store: HashMap::new(),
        }
    }

    /// Sets a value in store
    pub fn set(&mut self, a: String, b: String) {
        self.store.insert(a, b);
    }

    /// Get a value from a store
    ///
    /// Returns None where value doesn't exist
    pub fn get(&mut self, a: String) -> Option<String> {
        if self.store.contains_key(&a) {
            let value = self.store.get(&a);
            match value {
                Some(x) => return Some(x.to_string()),
                _ => return None,
            }
        }
        None
    }

    /// Remove a value from the store
    /// Always succeeds regardless of existence in the store
    pub fn remove(&mut self, a: String) {
        self.store.remove_entry(&a);
    }
}
