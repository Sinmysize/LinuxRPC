use std::{env, process::Command, thread::{self, sleep}, time::Duration};

use console::Style;
use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity::{self, Assets, Timestamps}};
use rand::seq::IndexedRandom;

use crate::{cli::{config_prompt}, config::Config};

mod config;
mod cli;

#[derive(Debug, Default)]
struct RPCState {
    timestamp: i64,
    icon: String,
    icon_text: String,
    small_icon: String,
    small_text: String,
    message: String,
}

impl RPCState {
    fn new(config: &Config) -> Self {
        let default_icon = match config.data.get("default_icon") {
            Some(d) => {
                if d.len() == 0 {
                    "Empty"
                } else {
                    d[0].as_str()
                }
            },
            None => "Empty"
        };

        let default_icon_text = match config.data.get("default_icon_text") {
            Some(d) => {
                if d.len() == 0 {
                    "Made by Sinmysize"
                } else {
                    d[0].as_str()
                }
            },
            None => "Made by Sinmysize"
        };

        let default_small_icon = match config.data.get("default_small_icon") {
            Some(d) => {
                if d.len() == 0 {
                    "Empty"
                } else {
                    d[0].as_str()
                }
            },
            None => "Empty"
        };

        let default_small_text = match config.data.get("default_small_text") {
            Some(d) => {
                if d.len() == 0 {
                    "Using Linux"
                } else {
                    d[0].as_str()
                }
            },
            None => "Using Linux"
        };

        Self {
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
            icon: default_icon.to_string(),
            icon_text: default_icon_text.to_string(),
            small_icon: default_small_icon.to_string(),
            small_text: default_small_text.to_string(),
            message: "A Simple RPC Client.".to_string()
        }
    }

    fn run_rpc(&mut self, mut client: &mut DiscordIpcClient, config: &Config) -> Result<(), ()> {
        let elapsed_time = Timestamps::new().start(self.timestamp);

        let messages = match config.data.get("messages") {
            Some(d) => d,
            None => &vec!["Check your config! [messages] is empty!".to_string()]
        };

        let default_icon = match config.data.get("default_icon") {
            Some(d) => {
                if d.len() == 0 {
                    "Empty"
                } else {
                    d[0].as_str()
                }
            },
            None => "Empty"
        };

        let icons = match config.data.get("icons") {
            Some(d) => d,
            None => &vec![format!("{}", default_icon)]
        };

        match client.connect() {
            Ok(_) => println!("Connected!"),
            Err(_) => {
                thread::sleep(Duration::from_millis(1_000));
                println!("Trying to connect to RPC...");
                self.run_rpc(client, config).unwrap();  
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
            elapsed_time.clone()
        )
        .expect("Something went wrong.");

        loop {
            sleep(Duration::from_millis(10_000));
            let data = get_playerctl(config.data.get("player").cloned());
            let music_format = format!("♪ {} - {}", data[0], if data.len() == 1 { "ᓚᘏᗢ ᶻ z Z" } else { &data[2] });

            self.message = messages.choose(&mut rand::rng()).map(|v| &**v).unwrap().to_string();
            self.icon = icons.choose(&mut rand::rng()).map(|v| &**v).unwrap().to_string();
            let music: Option<&str> = Some(&music_format);
            
            match set_activity(
                &mut client,
            Some(&self.message),
            Some(&self.icon),
            Some(&self.icon_text),
            music,
            Some(&self.small_text),
            Some(&self.small_icon),
            elapsed_time.clone()
            )
            {
                Ok(_) => {},
                Err(_) => {
                    println!("Something went wrong. Trying to reconnect...");
                    self.run_rpc(client, config).unwrap();
                }
            }
        }
    }
}

fn get_playerctl(player: Option<Vec<String>>) -> Vec<String> {
    let player = match &player {
        Some(e) => {
            if e.len() == 0 {
                "Empty"
            } else {
                &e[0]
            }
        },
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
    elapsed_time: Timestamps
) -> Result<(), discord_rich_presence::error::Error> {
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
        .timestamps(
            elapsed_time.clone()
        )
    )
}   

fn main() {
    let mut config = Config::new();
    config.read_config();

    let args = env::args().collect::<Vec<String>>();
    let help_msg = "Usage: linuxrpc [cmd]\n  run: Runs the RPC client directly (Best to use 'start' to run in background)\n  start: Runs the RPC client in the background\n  stop: Disconnects the RPC client\n  config: Runs the config CLI";

    if args.len() <= 1 || args.len() < 2 {
        println!("{help_msg}");
        return;
    }

    match &*args[1] {
        "start" => {
            let cmd = Command::new("systemctl").args(["--user", "enable", "linuxrpc.service"]).output().unwrap();
            let err_output = String::from_utf8(cmd.stderr).unwrap();

            if err_output.is_empty() {
                Command::new("systemctl").args(["--user", "start", "linuxrpc.service", "--now"]).output().unwrap();
            } else {
                println!("linuxrpc.service cannot be found.")
            }
        },
        "run" => {
            let mut client = DiscordIpcClient::new(
            match config.data.get("clientId") {
                    Some(d) =>  {
                        if d.len() == 0 {
                            let red = Style::new().red();
                            println!("[LinuxRPC]: {}", red.apply_to("Your client ID is empty!! Edit Config > Add to config > clientId"));
                            return;
                        } else {
                            d[0].as_str()
                        }
                    }, 
                    None => {
                        let red = Style::new().red();
                        println!("[LinuxRPC]: {}", red.apply_to("Your client ID is empty!! Edit Config > Add to config > clientId"));
                        return;
                    }
                }
            );

            let mut rpc = RPCState::new(&config);
            let _ = rpc.run_rpc(&mut client, &config);
        },
        "config" => config_prompt(),
        "stop" => {Command::new("systemctl").args(["--user", "stop", "linuxrpc.service"]).output().unwrap();},
        "refresh" => {Command::new("systemctl").args(["--user", "restart", "linuxrpc.service"]).output().unwrap();},
        "help" => println!("{help_msg}"),
        _ => println!("{help_msg}")
    }
}   