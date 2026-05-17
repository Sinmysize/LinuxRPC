use std::{env::home_dir, fs, io::Write};

use dialoguer::{Input, MultiSelect, Select};

use crate::config::{CONFIG_PATH, Config};

fn create_selection(prompt: &str, items: &Vec<&str>) -> usize {
    Select::new()
    .with_prompt(prompt)
    .default(0)
    .items(items)// &Vec<&str>
    .interact()
    .unwrap()
}

fn create_multiselection(prompt: &str, items: &Vec<&str>) -> Vec<usize> {
    MultiSelect::new()
    .with_prompt(prompt)
    .items(items)
    .interact()
    .unwrap()
}

fn create_input(prompt: &str) -> String {
    Input::<String>::new()
    .with_prompt(prompt)
    .interact_text()
    .unwrap()
}

pub fn config_prompt(config: &mut Config) {  
    let prompt = create_selection("Configuration", &vec!["Swap Config", "Edit Config", "Create Config", "Remove Config", "Exit"]);
    
    // Prompt
    match prompt {
        // Swap Config
        0 => {
            // Get all user made configs
            let options = config.get_configs();

            let index = create_selection("Swap Config", &config.get_configs().iter().map(|option| option.as_str()).collect::<Vec<&str>>());
            let new_config = options[index].clone();

            // Creating a temporary Config instance to make changes to config.rpc
            let mut temp_config = Config::new();
            temp_config.read_config();

            let current_value = temp_config.data.get_key_value("active").unwrap().1;
            temp_config.remove_from_config("active".to_string(), current_value.to_vec());
            temp_config.add_to_config("active".to_string(), new_config);

            match temp_config.data.get("active") {
                Some(d) => {
                    config.file = fs::File::options()
                    .write(true)
                    .read(true)
                    .create(true)
                    .open(format!("{}/{}/{}", home_dir().unwrap().display(), CONFIG_PATH, d[0]))
                    .unwrap();
                },

                None => panic!("Unable to get active value.")
            }

            return config_prompt(config);
        },

        // Edit Config
        1 => {
            let edit_selection = create_selection("Edit Config", &vec!["Add to Config", "Remove from Config", "Back"]);

            match edit_selection {
                // Adding to config
                0 => {
                    config.read_config();

                    // FOR SOME REASON THIS BECOMES EMPTY THE SECOND TIME WHEN USED??
                    // Read a second time if empty to ensure it reads it... I will figure out why this happens someday...
                    if config.data.is_empty() {
                        config.read_config();
                    }

                    let data = config.data.clone();
                    let keys = data.keys().map(|d| d.as_str()).collect::<Vec<&str>>();

                    let key = create_selection("Select key to edit", &keys);
                    let value = create_input("Enter value to add");

                    // Replaces the value instead of adding another value
                    if keys[key] == "clientId" || keys[key] == "player" || keys[key].contains("default") {
                        config.remove_from_config(keys[key].to_string(), vec![value.clone()]);
                    }

                    if keys[key] == "active" {
                        println!("[LinuxRPC]: It is not recommended to edit this field directly. This is done to prevent errors.");
                        return config_prompt(config);
                    }

                    config.add_to_config(keys[key].to_string(), value);
                },

                // Removing from configconfig
                1 => {
                    config.read_config();

                    let keys = config.data.keys().map(|d| d.as_str()).collect::<Vec<&str>>();
                    let key = create_selection("Select key to edit", &keys);

                    let x = config.data.get_key_value(&keys[key].to_string()).map(|d| d.1.iter().map(|f| f.as_str()).collect::<Vec<&str>>()).unwrap();
                    let value = create_multiselection("Select entries to remove (Press Space to select)", &x);

                    config.remove_from_config(keys[key].to_string(), value.into_iter().map(|f| x[f].to_string()).collect::<Vec<String>>());
                },

                2 => {
                    // Hello there
                },

                _ => {}
            }
            return config_prompt(config);
        },

        // Create Config
        2 => {
            let config_template = b"[clientId]\n\n[icons]\n\n[messages]\n\n[default_icon]\n\n[default_small_icon]\n\n[default_icon_text]\n\n[default_small_text]\n\n[player]\n";
            let file_name = create_input("Name your config file");

            let mut new_config = fs::File::create_new(format!("{}/{}/{}.rpc", home_dir().unwrap().display(), CONFIG_PATH, file_name)).unwrap();
            new_config.write_all(config_template).unwrap();
            
            return config_prompt(config);
        },

        // Remove Config
        3 => {
            let options = config.get_configs();

            let index = create_selection("Swap Config", &config.get_configs().iter().map(|option| option.as_str()).collect::<Vec<&str>>());
            let old_config = options[index].clone();

            let confirm_selection = create_selection("Are you sure you want to delete this config?", &vec!["Yes", "No"]);

            match confirm_selection {
                0 => {
                    fs::remove_file(format!("{}/{}/{}", home_dir().unwrap().display(), CONFIG_PATH, old_config)).unwrap();
                },

                1 => {
                    println!("[LinuxRPC]: Aborted deletion.");
                },

                _ => {}
            }
            
            return config_prompt(config);
        },

        _ => {}
    }
}