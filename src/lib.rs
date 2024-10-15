pub mod file;
pub mod io;

#[cfg(test)]
mod tests {
    use crate::file::ini::Ini;
    use crate::file::key::Key;
    use crate::file::section::Section;
    use crate::io::reader::IniReader;
    use crate::io::writer::IniWriter;

    #[test]
    fn it_works() {
        let mut ini = Ini::new();
        ini
            .add_key(Key::from("key_str".to_string(), "test".to_string()))
            .add_key(Key::from("key_int".to_string(), "1".to_string()))
            .add_section(
                Section::new("test".to_string())
                    .add_key(Key::from("key_str".to_string(), "test".to_string()))
                    .add_key(Key::from("key_int".to_string(), "1".to_string()))
                    .add_key(Key::from("key_bool".to_string(), "true".to_string()))
                    .add_key(Key::from("key_float".to_string(), "1.1".to_string()))
            ).add_section(
                Section::new("test2".to_string())
                    .add_key(Key::from("key_str".to_string(), "test2".to_string()))
                    .add_key(Key::from("key_int".to_string(), "2".to_string()))
                    .add_key(Key::from("key_bool".to_string(), "false".to_string()))
                    .add_key(Key::from("key_float".to_string(), "2.2".to_string()))
            );
        let writer = IniWriter::new("test.ini".to_string(), ini);
        writer.write().unwrap();
        let mut reader = IniReader::new("test.ini".to_string());
        let ini = reader.read().unwrap();
        dbg!(&ini);
        assert_eq!(ini.len_sections(), 2);
        assert_eq!(ini.len_keys(), 2);
        assert_eq!(ini.sections[0].name, "test");
        assert_eq!(ini.sections[0].keys.len(), 4);
        assert_eq!(ini.sections[1].name, "test2");
        assert_eq!(ini.sections[1].keys.len(), 4);
        assert_eq!(ini.keys[0].name, "key_str");
        assert_eq!(ini.keys[0].value, "test");
        assert_eq!(ini.keys[1].name, "key_int");
        assert_eq!(ini.keys[1].value, "1");
        assert_eq!(ini.sections[1].get_key("key_float").unwrap().as_float().unwrap(), 2.2f32);
    }
}
