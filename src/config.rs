use std::{collections::HashMap, env::home_dir, fs::{self, File}, io::{Read, Seek, SeekFrom, Write}};

use console::Style;

const CONFIG_PATH: &'static str = ".config/LinuxRPC";
const CONFIG_FILE: &'static str = "config.rpc";

pub struct Config {
    file: File,
    pub data: HashMap<String, Vec<String>>
}

impl Config {

    pub fn new() -> Self {
        let config_path: String = format!("{}/{}/{}", home_dir().unwrap().display(), CONFIG_PATH, CONFIG_FILE);

        fs::create_dir_all(format!("{}/{}", home_dir().unwrap().display(), CONFIG_PATH)).unwrap();

        let file = fs::File::options()
        .write(true)
        .read(true)
        .create(true)
        .open(&config_path)
        .unwrap();

        Self { file , data: HashMap::new() }
    }

    pub fn read_config(&mut self) {
        let config_path: String = format!("{}/{}", home_dir().unwrap().display(), CONFIG_PATH);
        fs::create_dir_all(&config_path).unwrap();

        let mut file = &self.file;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let _ = file.seek(SeekFrom::Start(0));

        let data = buffer.split("\n").map(|line| line.to_string()).collect::<Vec<String>>();
        
        if data.len() <= 1 && data[0] == "" {
            let red = Style::new().red();

            println!("[LinuxRPC]: {}", red.apply_to("Your config.rpc is empty! Get the template here: https://github.com/Sinmysize/LinuxRPC?tab=readme-ov-file#configuration"));
        }

        let mut key = String::new();

        for line in data {
            if line.is_empty() {
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


    pub fn write_config(&mut self) {
        let mut contents = String::new();

        let _ = self.file.set_len(0);

        for key in &self.data {
            contents += &*format!("\n[{}]\n", key.0);

            for value in key.1 {
                contents += &*format!("{}\n", value);
            }
        }

        let _ = self.file.write_all(contents.as_bytes());
        println!("Made changes to config!");
    }

    pub fn add_to_config(&mut self, key: String, value: String) {
        if !self.data.contains_key(&key) {
            println!("Som ting wong wit varina");
            return
        }

        // Modify Hashmap
        self.data.get_mut(&key).unwrap().push(value);

        // Write to file
        self.write_config();
    }

    pub fn remove_from_config(&mut self, key: String, values: Vec<String>) {
        if !self.data.contains_key(&key) {
            println!("Som ting wong wit varina");
            return
        }

        for index in values {
            self.data.get_mut(&key).unwrap().retain(|e| e != &index);
        }

        self.write_config();
    }

}
