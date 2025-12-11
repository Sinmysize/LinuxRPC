use dialoguer::{Input, MultiSelect, Select};

use crate::{config::{Config}, main};

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
    let e = create_selection("Edit Config", &vec!["Add to config", "Remove from config", "Return"]).unwrap();

    match e {
        0 => {
            let mut config = Config::new();
            config.read_config();

            let e = config.data.keys().map(|d| d.as_str()).collect::<Vec<&str>>();

            let key = create_selection("Select key to edit", &e).unwrap();
            let value = create_input("Enter value to add");

            config.add_to_config(e[key].to_string(), value);
        },

        1 => {
            let mut config = Config::new();
            config.read_config();

            let e = config.data.keys().map(|d| d.as_str()).collect::<Vec<&str>>();

            let key = create_selection("Select key to edit", &e).unwrap();

            let x = config.data.get_key_value(&e[key].to_string()).map(|d| d.1.iter().map(|f| f.as_str()).collect::<Vec<&str>>()).unwrap();

            let value = create_multiselection("Select entries to remove (Press Space to select)", &x);

            config.remove_from_config(e[key].to_string(), value.into_iter().map(|f| x[f].to_string()).collect::<Vec<String>>());
        },

        2 => {
            main();
        }

        _ => {}
    }
}