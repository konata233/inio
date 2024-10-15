use std::num::{ParseFloatError, ParseIntError};

#[derive(Clone, Debug)]
pub struct Key {
    pub name: String,
    pub value: String,
}

impl Key {
    pub fn new(name: String, value: String) -> Key {
        Key {
            name,
            value: value.trim().trim_end_matches('\0').parse().unwrap(),
        }
    }

    pub fn from<T>(name: String, value: T) -> Key where T: Into<String> {
        Key {
            name,
            value: value.into(),
        }
    }
    
    pub fn as_type<T>(&self) -> T where T: From<String> {
        self.value.clone().into()
    }
    
    pub fn as_bool(&self) -> bool {
        self.value == "true" || self.value == "1" || self.value.to_lowercase() == "yes"
    }
    
    pub fn as_int(&self) -> Result<i32, ParseIntError> {
        self.value.parse::<i32>()
    }
    
    pub fn as_i64(&self) -> Result<i64, ParseIntError> {
        self.value.parse::<i64>()
    }
    
    pub fn as_string(&self) -> String {
        self.value.clone()
    }
    
    pub fn as_float(&self) -> Result<f32, ParseFloatError> {
        self.value.parse::<f32>()
    }
    
    pub fn as_usize(&self) -> Result<usize, ParseIntError> {
        self.value.parse::<usize>()
    }
    
    pub fn as_f64(&self) -> Result<f64, ParseFloatError> {
        self.value.parse::<f64>()
    }
    
    pub fn as_uint(&self) -> Result<u32, ParseIntError> {
        self.value.parse::<u32>()
    }
    
    pub fn as_u64(&self) -> Result<u64, ParseIntError> {
        self.value.parse::<u64>()
    }
    
    pub fn get(&self) -> String {
        self.value.clone()
    }
}