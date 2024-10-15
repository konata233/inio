use std::fs;
use std::io::Write;
use crate::file::ini::Ini;

pub struct IniWriter {
    pub file_path: String,
    pub ini_file: Ini
}

impl IniWriter {
    pub fn new(file_path: String, ini_file: Ini) -> IniWriter {
        IniWriter {
            file_path,
            ini_file
        }
    }

    pub fn write(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = fs::File::create(self.file_path.clone())?;

        for key in self.ini_file.keys.iter() {
            file.write_all(format!("{} = {}\n", key.name, key.value).as_bytes())?;
        }
        file.write_all("\n".as_bytes())?;

        for section in self.ini_file.sections.iter() {
            file.write_all(format!("[{}]\n", section.name).as_bytes())?;
            for key in section.keys.iter() {
                file.write_all(format!("{} = {}\n", key.1.name, key.1.value).as_bytes())?;
            }
            file.write_all("\n".as_bytes())?;
        }
        Ok(())
    }
}