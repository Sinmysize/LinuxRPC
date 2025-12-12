use std::{env, process::Command, thread::{self, sleep}, time::Duration};

use console::Style;
use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity::{self, Assets, Timestamps}};
use rand::seq::IndexedRandom;

use crate::{cli::{config_prompt, create_selection}, config::Config};

mod config;
mod cli;

fn get_playerctl_spotify() -> Vec<String> {
    let metadata = Command::new("playerctl")
    .args(["-p", "spotify", "metadata", "-f", "{{artist}}*{{album}}*{{title}}*{{length}}"])
    .output()
    .expect("Something went wrong in getting metadata for spotify.");

    let output = String::from_utf8_lossy(&metadata.stdout).into_owned();
    
    return output.split("*").map(|s| s.to_string()).collect::<Vec<String>>()
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

fn run_rpc(mut client: &mut DiscordIpcClient, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let unix_timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
    let elapsed_time = Timestamps::new().start(unix_timestamp);

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

    match client.connect() {
        Ok(_) => println!("Connected!"),
        Err(_) => {
            println!("Trying to connect to RPC...");
            run_rpc(client, config).unwrap();
            thread::sleep(Duration::from_millis(1_000));
        }
    };

    set_activity(
        &mut client,
        None,
        Some(default_icon),
        Some(default_icon_text),
        None,
        Some(default_small_text),
        Some(default_small_icon),
        elapsed_time.clone()
    )
    .expect("Something went wrong.");

    loop {
        sleep(Duration::from_millis(10_000));
        let data = get_playerctl_spotify();
        let music_format = format!("♪ {} - {}", data[0], if data.len() == 1 { "ᓚᘏᗢ ᶻ z Z" } else { &data[2] });

        let text = messages.choose(&mut rand::rng()).map(|v| &**v);
        let icon = icons.choose(&mut rand::rng()).map(|v| &**v);
        let music: Option<&str> = Some(&music_format);
        
        match set_activity(
            &mut client,           
            text, 
            icon,
            Some(default_icon_text),      
            music,   
            Some(default_small_text),
            Some(default_small_icon),
            elapsed_time.clone()
        )
        {
            Ok(_) => {},
            Err(_) => {
                println!("Something went wrong. Trying to reconnect...");
                run_rpc(client, config).unwrap();
                thread::sleep(Duration::from_millis(1_000));
            }
        }
    }
}

fn main() {
    let mut config = Config::new();
    config.read_config();

    let mut client = DiscordIpcClient::new(
        match config.data.get("clientId") {
            Some(d) =>  {
                if d.len() == 0 {
                    let red = Style::new().red();
                    println!("{}", red.apply_to("NO CLIENT ID CURRENTLY PRESENT!!\nEdit Config > Add to config > clientId"));
                    ""
                } else {
                    d[0].as_str()
                }
            }, 
            None => {
                let red = Style::new().red();
                println!("{}", red.apply_to("NO CLIENT ID CURRENTLY PRESENT!!\nEdit Config > Add to config > clientId"));
                ""
            }
        }
    );

    let selections = [
        "Edit Config",
        "Refresh RPC",
        "Start RPC",
        "Stop RPC"
    ];

    let args = env::args().collect::<Vec<String>>();

    if args.len() <= 1 || args.len() < 2 {
        println!("Usage: linuxrpc [cmd]\n run: Runs the RPC Client (Best to use 'start')\n config: Runs the config CLI");
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
        "run" => run_rpc(&mut client, &config).unwrap(),
        "config" => {
            let main_prompt = create_selection("Welcome to LinuxRPC CLI!", &selections.to_vec()).unwrap();

            match main_prompt {
                0 => config_prompt(),
                1 => {Command::new("systemctl").args(["--user", "restart", "linuxrpc.service"]).output().unwrap();},
                2 => {Command::new("systemctl").args(["--user", "start", "linuxrpc.service", "--now"]).output().unwrap();},
                3 => {Command::new("systemctl").args(["--user", "stop", "linuxrpc.service"]).output().unwrap();},
                _ => panic!("How did we get here?")
            }
        },
        "help" => println!("Usage: linuxrpc [cmd]\n run: Runs the RPC Client (Best to use 'start')\n config: Runs the config CLI"),
        _ => println!("Usage: linuxrpc [cmd]\n run: Runs the RPC Client (Best to use 'start')\n config: Runs the config CLI")
    }
}   