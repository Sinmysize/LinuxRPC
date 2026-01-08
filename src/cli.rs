use dialoguer::{Input, MultiSelect, Select};

use crate::{config::{Config}};

pub fn create_selection(prompt: &str, items: &Vec<&str>) -> Result<usize, dialoguer::Error> {
    Select::new()
    .with_prompt(prompt)
    .default(0)
    .items(items)
    .interact()
}

pub fn create_multiselection(prompt: &str, items: &Vec<&str>) -> Vec<usize> {
    MultiSelect::new()
    .with_prompt(prompt)
    .items(items)
    .interact()
    .unwrap()
}

pub fn create_input(prompt: &str) -> String {
    Input::<String>::new()
    .with_prompt(prompt)
    .interact_text()
    .unwrap()
}

pub fn config_prompt() {
    let e = create_selection("Edit Config", &vec!["Add to config", "Remove from config", "Exit"]).unwrap();

    match e {
        0 => {
            let mut config = Config::new();
            config.read_config();

            let data = config.data.clone();
            let keys = data.keys().map(|d| d.as_str()).collect::<Vec<&str>>();

            let key = create_selection("Select key to edit", &keys).unwrap();
            let value = create_input("Enter value to add");

            // Replaces it instead of adding another value
            if keys[key] == "clientId" || keys[key] == "player" || keys[key].contains("default") {
               config.remove_from_config(keys[key].to_string(), vec![value.clone()]);
            }

            config.add_to_config(keys[key].to_string(), value);
            config_prompt();
        },

        1 => {
            let mut config = Config::new();
            config.read_config();

            let keys = config.data.keys().map(|d| d.as_str()).collect::<Vec<&str>>();
            let key = create_selection("Select key to edit", &keys).unwrap();

            let x = config.data.get_key_value(&keys[key].to_string()).map(|d| d.1.iter().map(|f| f.as_str()).collect::<Vec<&str>>()).unwrap();
            let value = create_multiselection("Select entries to remove (Press Space to select)", &x);

            config.remove_from_config(keys[key].to_string(), value.into_iter().map(|f| x[f].to_string()).collect::<Vec<String>>());
            config_prompt();
        },

        2 => {
            println!("Have a nice day!");
            return;
        }

        _ => {}
    }
}