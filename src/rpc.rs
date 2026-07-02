use std::{env::home_dir, fs, process::Command, thread, time::Duration};

use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity::{self, Assets, Button, Timestamps}};
use rand::seq::IndexedRandom;

use crate::{config::{CONFIG_PATH, Config}};

#[derive(Default)]
pub struct RPCState {
    timestamp: i64,
    icon: String,
    icon_text: String,
    small_icon: String,
    small_text: String,
    message: String,
    buttons: Vec<(String, String)>
}

impl RPCState {
    pub fn new(config: &Config) -> Self {
        let default_icon = match config.data.get("default_icon") {
            Some(data) => data.clone().first().unwrap_or(&"Empty".to_string()).clone(),
            None => "Empty".to_string()
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

        let elapsed_time = Timestamps::new().start(self.timestamp);
        let mut client = DiscordIpcClient::new(&config.data.get("clientId").expect("Failed to get [clientId] from config. Please check if [clientId] exists and has a valid ID")[0]);

        match client.connect() {
            Ok(_) => println!("Connected!"),
            Err(_) => {
                thread::sleep(Duration::from_millis(1_000));
                println!("Trying to connect to RPC...");
                self.run_rpc(config).unwrap();  
            }
        }

        set_activity(
            &mut client,
            Some(&self.message),
            Some(&self.icon),
            Some(&self.icon_text),
            None,
            Some(&self.small_text),
            Some(&self.small_icon),
            self.buttons.clone(),
            elapsed_time.clone()
        )
        .expect("Something went wrong.");

        loop {
            thread::sleep(Duration::from_millis(10_000));
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

            let data = get_playerctl(config.data.get("player").cloned());
            let music_format = format!("♪ {} - {}", data[0], if data.len() == 1 { "ᓚᘏᗢ ᶻ z Z" } else { &data[2] });

            self.message = messages.choose(&mut rand::rng()).map(|selected| selected.to_string()).unwrap();
            self.icon = icons.choose(&mut rand::rng()).map(|selected| selected.to_string()).unwrap();
            let music: Option<&str> = Some(&music_format);
            
            match set_activity(
                &mut client,
            Some(&self.message),
            Some(&self.icon),
            Some(&self.icon_text),
            music,
            Some(&self.small_text),
            Some(&self.small_icon),
            self.buttons.clone(),
            elapsed_time.clone()
            )
            {
                Ok(_) => {},
                Err(_) => {
                    println!("Something went wrong. Trying to reconnect...");
                    self.run_rpc(config).unwrap();
                }
            }
        }
    }

    pub fn stop_rpc(self, config: &Config) {
        let mut client = DiscordIpcClient::new(&config.data.get("clientId").expect("Failed to get [clientId] from config. Please check if [clientId] exists and has a valid ID")[0]);
        client.close().unwrap();
    }
}

fn get_playerctl(player: Option<Vec<String>>) -> Vec<String> {
    let player = match &player {
        Some(data) => data.first().map_or("Empty", |str| str),
        None => "Empty"
    };

    let metadata = Command::new("playerctl")
    .args(["-p", player, "metadata", "-f", "{{artist}}*{{album}}*{{title}}*{{length}}"])
    .output()
    .expect("Something went wrong in getting metadata.");

    let output = String::from_utf8_lossy(&metadata.stdout).into_owned();
    return output.split("*").map(|s| s.to_string()).collect::<Vec<String>>();
}

fn set_activity(
    client: &mut DiscordIpcClient, 
    text: Option<&str>, 
    icon: Option<&str>, 
    icon_text: Option<&str>,
    music: Option<&str>,
    small_text: Option<&str>,
    small_icon: Option<&str>,
    buttons: Vec<(String, String)>,
    elapsed_time: Timestamps
) -> Result<(), discord_rich_presence::error::Error> {
    let mut cached_buttons = vec![];

    for button in buttons {
        if button.0.is_empty() || button.1.is_empty() {
            continue;
        }

        let new_button = Button::new(button.0, button.1);
        cached_buttons.push(new_button);
    }

    client.set_activity(
        activity::Activity::new()
        .details(text.unwrap_or("A Simple RPC Client."))
        .state(music.unwrap_or("Loading LinuxRPC..."))
        .assets(
            Assets::new()
            // Will default to discord placeholder icon! (Can change if manually building)
            .large_image(icon.unwrap_or("Empty")) 
            .large_text(icon_text.unwrap_or("Empty"))
            .small_image(small_icon.unwrap_or("Empty"))
            .small_text(small_text.unwrap_or("Empty"))
        )
        .buttons(cached_buttons)
        .timestamps(
            elapsed_time.clone()
        )
    )
}  