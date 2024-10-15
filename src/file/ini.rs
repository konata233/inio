use std::collections::HashMap;
use std::error::Error;
use crate::file::key::Key;
use crate::file::section::Section;

#[derive(Debug)]
pub struct Ini {
    pub sections: Vec<Section>,
    pub keys: Vec<Key>
}

impl Ini {
    pub fn clone(&self) -> Result<Ini, Box<dyn Error>> {
        Ok(Ini {
            sections: self.sections.clone(),
            keys: self.keys.clone()
        })
    }
}

impl Ini {
    pub fn new() -> Ini {
        Ini {
            sections: Vec::new(),
            keys: Vec::new()
        }
    }

    pub fn add_section(&mut self, section: Section) -> &mut Self {
        self.sections.push(section);
        self
    }

    pub fn add_section_named(&mut self, name: String) {
        self.sections.push(Section::new(name));
    }

    pub fn add_key(&mut self, key: Key) -> &mut Self {
        self.keys.push(key);
        self
    }

    pub fn get_section(&self, name: &str) -> Option<&Section> {
        self.sections.iter().find(|s| s.name == name)
    }

    pub fn get_section_mut(&mut self, name: &str) -> Option<&mut Section> {
        self.sections.iter_mut().find(|s| s.name == name)
    }

    pub fn remove_section(&mut self, name: &str) -> Option<Section> {
        Some(self.sections.remove(self.sections.iter().position(|s| s.name == name).unwrap()))
    }

    pub fn get_keys(&self) -> Vec<&Key> {
        self.keys.iter().collect()
    }

    pub fn get_keys_mut(&mut self) -> Vec<&mut Key> {
        self.keys.iter_mut().collect()
    }

    pub fn get_key(&self, name: &str) -> Option<&Key> {
        self.keys.iter().find(|k| k.name == name)
    }

    pub fn get_key_mut(&mut self, name: &str) -> Option<&mut Key> {
        self.keys.iter_mut().find(|k| k.name == name)
    }

    pub fn remove_key(&mut self, name: &str) -> Option<Key> {
        Some(self.keys.remove(self.keys.iter().position(|k| k.name == name).unwrap()))
    }

    pub fn len_sections(&self) -> usize {
        self.sections.len()
    }

    pub fn len_keys(&self) -> usize {
        self.keys.len()
    }
}

impl Ini {
    pub fn convert_as_hashmap(&self) -> (HashMap<String, String>, HashMap<String, HashMap<String, String>>) {
        let mut map = HashMap::new();

        for section in self.sections.iter() {
            let mut section_map = HashMap::new();
            for key in section.keys.values() {
                section_map.insert(key.name.clone(), key.value.clone());
            }
            map.insert(section.name.clone(), section_map);
        }

        let mut root_map = HashMap::new();
        for key in self.keys.iter() {
            root_map.insert(key.name.clone(), key.value.clone());
        }

        (root_map, map)
    }
}