use crate::file::ini::Ini;
use crate::file::key::Key;
use crate::file::section::Section;
use std::fs;
use std::io::BufRead;

pub struct IniReader {
    pub file_path: String,
    pub ini_file: Ini
}

impl IniReader {
    pub fn new(file_path: String) -> IniReader {
        IniReader {
            file_path,
            ini_file: Ini::new()
        }
    }

    pub fn read(&mut self) -> Result<Ini, Box<dyn std::error::Error>> {
        let file = fs::File::open(self.file_path.clone()).unwrap();
        let reader = std::io::BufReader::new(file);

        let mut iter = reader.lines();
        let mut retrieve: Option<String> = None;
        let mut data_tmp: Option<String> = None;

        while let Some(l) = iter.next() {
            if let Ok(l) = l {
                let l = l.trim().trim_end_matches('\0').to_string();
                if l.is_empty() || l.starts_with(";") {
                    continue;
                }
                if retrieve.is_some() {
                    data_tmp = Some(l.clone());
                }
                if l.starts_with("[") {
                    let mut section = Section::new(l.trim_start_matches('[').trim_end_matches(']').to_string());
                    'reader: while let Some(sub) = iter.next() {
                        if let Ok(mut sub) = sub {
                            dbg!(&sub);
                            sub = sub.trim().trim_end_matches('\0').to_string();
                            if sub.starts_with(';') || sub.is_empty() {
                                continue 'reader;
                            }
                            if sub.starts_with('[') {
                                retrieve = Some(sub.clone());
                                break 'reader;
                            }
                            if sub.contains('=') {
                                let mut split = sub.split('=');
                                let key = split.next().unwrap().trim().trim_end_matches('\0');
                                let value = split.next().unwrap().trim().trim_end_matches('\0');
                                section = section.add_key(
                                   Key::new(key.parse()?, value.parse()?)
                                )
                            }
                        }
                    }
                    self.ini_file.add_section(section);
                } else if let Some(ref mut retr) = retrieve {
                    let mut section = Section::new(retr.trim_start_matches('[').trim_end_matches(']').to_string());
                    if let Some(ref mut tmp) = data_tmp {
                        section = section.add_key(
                            Key::new(tmp.trim().trim_end_matches('\0').parse()?,
                                     l.trim().trim_end_matches('\0').parse()?
                            )
                        );
                    }
                    'reader: while let Some(sub) = iter.next() {
                        if let Ok(mut sub) = sub {
                            sub = sub.trim().trim_end_matches('\0').to_string();
                            dbg!(&sub);
                            if sub.starts_with(';') || sub.is_empty() {
                                continue 'reader;
                            }
                            if sub.starts_with('[') {
                                retrieve = Some(sub);
                                break 'reader;
                            }
                            if sub.contains('=') {
                                let mut split = sub.split('=');
                                let key = split.next().unwrap().trim().trim_end_matches('\0');
                                let value = split.next().unwrap().trim().trim_end_matches('\0');
                                section = section.add_key(
                                    Key::new(key.parse()?, value.parse()?)
                                )
                            }
                        }
                    }
                    self.ini_file.add_section(section);
                } else if l.contains('=') {
                    dbg!(&l);
                    let mut split = l.split('=');
                    let key = split.next().unwrap().trim().trim_end_matches('\0');
                    let value = split.next().unwrap().trim().trim_end_matches('\0');
                    self.ini_file.add_key(
                        Key::new(key.parse()?,
                                 value.parse()?
                        )
                    );
                }
            }
        }

        self.ini_file.clone()
    }
}