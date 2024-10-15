use crate::file::key::Key;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Section {
    pub name: String,
    pub keys: HashMap<String, Key>
}

impl Section {
    pub fn new(name: String) -> Section {
        Section {
            name,
            keys: HashMap::new()
        }
    }
}

impl Section {
    pub fn add_key(mut self, key: Key) -> Self {
        self.keys.insert(key.name.clone(), key);
        self
    }

    pub fn get_key(&self, name: &str) -> Option<&Key> {
        self.keys.get(name)
    }

    pub fn get_key_mut(&mut self, name: &str) -> Option<&mut Key> {
        self.keys.get_mut(name)
    }

    pub fn remove_key(&mut self, name: &str) -> Option<Key> {
        self.keys.remove(name)
    }

    pub fn get_keys(&self) -> Vec<&Key> {
        self.keys.values().collect()
    }

    pub fn get_keys_mut(&mut self) -> Vec<&mut Key> {
        self.keys.values_mut().collect()
    }

    pub fn as_hashmap(&self) -> HashMap<String, Key> {
        self.keys.clone()
    }

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }
}