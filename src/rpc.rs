use std::{env::home_dir, fs, process::Command, thread, time::Duration};

use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity::{self, Assets, Button, Timestamps}};
use rand::seq::IndexedRandom;

use crate::{cli::{err_message, std_message, success_message}, config::{CONFIG_PATH, Config}};

#[derive(Default, Clone)]
pub struct RPCState {
    timestamp: i64,
    icon: String,
    icon_text: String,
    small_icon: String,
    small_text: String,
    message: String,
    music: String,
    buttons: Vec<(String, String)>
}

impl RPCState {
    pub fn new(config: &Config) -> Self {
        let default_icon = match config.data.get("default_icon") {
            Some(data) => data.first().map_or("Empty", |str| str),
            None => "Empty"
        };

        let default_icon_text = match config.data.get("default_icon_text") {
            Some(data) => data.first().map_or("Made by Sinmysize", |str| str),
            None => "Made by Sinmysize"
        };

        let default_small_icon = match config.data.get("default_small_icon") {
            Some(data) => data.first().map_or("Empty", |str| str),
            None => "Empty"
        };

        let default_small_text = match config.data.get("default_small_text") {
            Some(data) => data.first().map_or("Using Linux", |str| str),
            None => "Using Linux"
        };

        let button1 = {
            let label = match config.data.get("button1_label") {
                Some(data) => data.first().map_or("", |str| str),
                None => ""
            };

            let url = match config.data.get("button1_url") {
                Some(data) => data.first().map_or("", |str| str),
                None => ""
            };

            (label.to_string(), url.to_string())
        };

        let button2 = {
            let label = match config.data.get("button2_label") {
                Some(data) => data.first().map_or("", |str| str),
                None => ""
            };

            let url = match config.data.get("button2_url") {
                Some(data) => data.first().map_or("", |str| str),
                None => ""
            };

            (label.to_string(), url.to_string())
        };

        Self {
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
            icon: default_icon.to_string(),
            icon_text: default_icon_text.to_string(),
            small_icon: default_small_icon.to_string(),
            small_text: default_small_text.to_string(),
            message: "A Simple RPC Client.".to_string(),
            music: "Loading Client...".to_string(),
            buttons: vec![button1, button2]
        }
    }

    pub fn run_rpc(&mut self, config: &mut Config) -> Result<(), ()> {
        config.read_config();

        let mut temp_config = Config::new();
        temp_config.read_config();

        let mut cached_active = match temp_config.data.get("active") { 
            Some(data) => data.first().unwrap().clone(),
            None => "PLACEHOLDER".to_string() // This shouldn't be used at all, but you never know
        };

        let mut client = DiscordIpcClient::new(&config.data.get("clientId").expect("Failed to get [clientId] from config. Please check if [clientId] exists and has a valid ID")[0]);

        match client.connect() {
            Ok(_) => println!("{}", success_message("Connected!")),
            Err(_) => {
                println!("{}", std_message("Trying to reconnect..."));
                self.run_rpc(config).unwrap();
                thread::sleep(Duration::from_millis(1_000));
            }
        }
    
        loop {
            config.read_config();
            
            let mut temp_config = Config::new();
            temp_config.read_config();

            let current_active = temp_config.data.get("active").unwrap()[0].clone();
 
            if current_active != cached_active {
                println!("Swapped Config");

                cached_active = current_active.clone();

                config.file = fs::File::options()
                .write(true)
                .read(true)
                .create(true)
                .open(format!("{}/{}/{}", home_dir().unwrap().display(), CONFIG_PATH, current_active))
                .unwrap();

                config.read_config();

                // Establish new client
                client.client_id = config.data.get("clientId").unwrap()[0].clone();
                client.reconnect().unwrap();
            }

            let mut messages = match config.data.get("messages") {
                Some(data) => data.clone(),
                None => vec!["Check your config! [messages] is empty!".to_string()]
            };

            if messages.is_empty() {
                messages.push("Check your config! [messages] is empty!".to_string());
            }

            let default_icon = match config.data.get("default_icon") {
                Some(data) => data.first().unwrap_or(&"Empty".to_string()).clone(),
                None => "Empty".to_string()
            };

            let mut icons = match config.data.get("icons") {
                Some(data) => data.clone(),
                None => vec![format!("{}", default_icon)]
            };

            if icons.is_empty() {
                icons.push("Empty".to_string());
            }

            self.message = messages.choose(&mut rand::rng()).map(|selected| selected.to_string()).unwrap();
            self.icon = icons.choose(&mut rand::rng()).map(|selected| selected.to_string()).unwrap();
            self.music = get_playerctl(config);
            
            match self.clone().set_activity(&mut client) {
                Ok(_) => {},
                Err(_) => {
                    println!("{}", err_message("Something went wrong. Trying to reconnect..."));
                    self.run_rpc(config).unwrap();
                }
            }

            thread::sleep(Duration::from_millis(10_000));
        }
    }

    pub fn stop_rpc(self, config: &Config) {
        let mut client = DiscordIpcClient::new(&config.data.get("clientId").expect("Failed to get [clientId] from config. Please check if [clientId] exists and has a valid ID")[0]);
        client.close().unwrap();
    }

    pub fn set_activity(self, client: &mut DiscordIpcClient) -> Result<(), discord_rich_presence::error::Error> {
        let mut cached_buttons = vec![];

        for button in self.buttons {
            if button.0.is_empty() || button.1.is_empty() {
                continue;
            }

            let new_button = Button::new(button.0, button.1);
            cached_buttons.push(new_button);
        }

        client.set_activity(
            activity::Activity::new()
            .details(self.message)
            .state(self.music)
            .assets(
                Assets::new()
                .large_image(self.icon) 
                .large_text(self.icon_text)
                .small_image(self.small_icon)
                .small_text(self.small_text)
            )
            .buttons(cached_buttons)
            .timestamps(
                Timestamps::new().start(self.timestamp)
            )
        )
    }  
}

fn get_playerctl(config: &Config) -> String {
    let player = match config.data.get("player") {
        Some(data) => data.first().map_or("Empty", |str| str),
        None => "Empty"
    };

    let metadata = Command::new("playerctl")
    .args(["-p", player, "metadata", "-f", "{{artist}}*{{album}}*{{title}}*{{length}}"])
    .output()
    .expect("Something went wrong in getting metadata.");

    let output = String::from_utf8_lossy(&metadata.stdout).into_owned().split("*").map(|s| s.to_string()).collect::<Vec<String>>();
    return format!("♪ {} - {}", output[0], if output.len() == 1 { "ᓚᘏᗢ ᶻ z Z" } else { &output[2] });
}