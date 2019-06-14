use std::string::*;
use std::option::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct KvStore {
    store : HashMap<String, String>
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore { store : HashMap::new() }
    }

    pub fn set(&mut self, a : String, b : String) {
        self.store.insert(a, b);
    }

    pub fn get(&mut self, a : String) -> Option<String> {
        if self.store.contains_key(&a) {
            let value = self.store.get(&a);
            match value {
                Some(x) => { return Some(x.to_string()) }
                _ => { return None }
            }
        }
        None
    }

    pub fn remove(&mut self, a : String) {
        self.store.remove_entry(&a);
    }
}