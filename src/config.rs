use std::{collections::HashMap, env::home_dir, fs::{self, File}, io::{Read, Seek, SeekFrom, Write}};

use console::Style;

pub const CONFIG_PATH: &'static str = ".config/LinuxRPC";
pub const CONFIG_FILE: &'static str = "config.rpc";

pub struct Config {
    pub file: File,
    pub data: HashMap<String, Vec<String>>
}

impl Config {
    pub fn new() -> Self {
        let config_path: String = format!("{}/{}/{}", home_dir().unwrap().display(), CONFIG_PATH, CONFIG_FILE);

        fs::create_dir_all(format!("{}/{}", home_dir().unwrap().display(), CONFIG_PATH)).unwrap();

        let mut file = fs::File::options()
        .write(true)
        .read(true)
        .create(true)
        .open(&config_path)
        .unwrap();
        
        if file.metadata().unwrap().len() < 1 {
            file.write_all(b"[active]").unwrap();
        }

        Self { file , data: HashMap::new() }
    }

    pub fn get_configs(&mut self) -> Vec<String> {
        let mut dirs = fs::read_dir(format!("{}/{}", home_dir().unwrap().display(), CONFIG_PATH)).unwrap()
        .map(|dir| {
            dir.unwrap().file_name().to_str().unwrap().to_string()
        })
        .collect::<Vec<_>>();

        for dir in dirs.clone() {
            if dir.ends_with(CONFIG_FILE) {
                let index = dirs.clone().into_iter().position(|e| e == dir).unwrap();
                dirs.remove(index);

                continue;
            }

            if dir.ends_with(".rpc") {
                continue;
            }

            let index = dirs.clone().into_iter().position(|e| e == dir).unwrap();
            dirs.remove(index);
        }

        dirs
    }

    pub fn read_config(&mut self) {
        self.data.clear();

        let config_path: String = format!("{}/{}", home_dir().unwrap().display(), CONFIG_PATH);
        fs::create_dir_all(&config_path).unwrap();

        let mut file = &self.file;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let _ = file.seek(SeekFrom::Start(0));

        let data = buffer.lines().map(|line| line.to_string()).collect::<Vec<String>>();

        if data.len() == 0 {
            return
        }

        if data.len() <= 1 && data[0] == "" {
            let red = Style::new().red();
            println!("[LinuxRPC]: {}", red.apply_to("The config you entered does not exist"));
        }

        let mut key = String::new();

        for line in data {
            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            if line.starts_with("[") && line.ends_with("]") {
                key = line[1..line.len() - 1].to_string();

                self.data.insert(key.clone(), vec![]);
                continue;
            }
            
            if let Some(v) = self.data.get_mut(&key) {
                v.push(line.trim().to_string());
            } else {
                self.data.insert(key.clone(), vec![line.trim().to_string()]);
            }
        }
    }


    fn write_config(&mut self) {
        let mut contents = String::new();

        let _ = self.file.set_len(0);
        let _ = self.file.seek(SeekFrom::Start(0));

        for key in &self.data {
            contents += &format!("\n[{}]\n", key.0);

            for value in key.1 {
                contents += &*format!("{}\n", value.replace("\r", ""));
            }
        }

        let _ = self.file.write_all(contents.as_bytes());
    }

    pub fn add_to_config(&mut self, key: String, value: String) {
        if !self.data.contains_key(&key) {
            println!("Something went wrong getting the key");
            return
        }

        // Modify Hashmap
        self.data.get_mut(&key).unwrap().push(value);

        // Write to file
        self.write_config();
    }

    pub fn remove_from_config(&mut self, key: String, values: Vec<String>) {
        if !self.data.contains_key(&key) {
            println!("Something went wrong getting the key");
            return
        }

        for index in values {
            self.data.get_mut(&key).unwrap().retain(|e| e != &index);
        }

        self.write_config();
    }

    pub fn read_active_config(&mut self) {
        match self.data.get("active") {
            Some(d) => {
                if d.len() != 0 {
                    if !fs::exists(format!("{}/{}/{}", home_dir().unwrap().display(), CONFIG_PATH, d[0])).unwrap() {
                        let red = Style::new().red();
                        println!("[LinuxRPC]: {}", red.apply_to(format!("The config {} does not exist. Please swap to a different config or create a new one.", d[0])));
                    } else {
                        self.file = fs::File::options()
                        .write(true)
                        .read(true)
                        .create(true)
                        .open(format!("{}/{}/{}", home_dir().unwrap().display(), CONFIG_PATH, d[0]))
                        .unwrap();
                    }
                } else {
                    let yellow = Style::new().yellow();
                    println!("[LinuxRPC]: {}", yellow.apply_to("There are no configs active. Please swap to a config or create a new one."));
                }

                self.read_config();
            },

            None => {}
        };
    }
}
