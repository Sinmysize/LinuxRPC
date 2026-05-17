use std::{env::{self, home_dir}, fs, process::Command};

use console::Style;

use crate::{cli::config_prompt, config::{CONFIG_PATH, Config}, rpc::RPCState};

mod config;
mod cli;
mod rpc;

fn main() {
    let mut config = Config::new();
    config.read_config();

    match config.data.get("active") {
        Some(d) => {
            if d.len() != 0 {
                if !fs::exists(format!("{}/{}/{}", home_dir().unwrap().display(), CONFIG_PATH, d[0])).unwrap() {
                    let red = Style::new().red();
                    println!("[LinuxRPC]: {}", red.apply_to(format!("The config {} does not exist. Please swap to a different config or create a new one.", d[0])));
                } else {
                    config.file = fs::File::options()
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
        },

        None => {}
    };

    config.read_config();

    let args = env::args().collect::<Vec<String>>();
    let help_msg = "Usage: linuxrpc [cmd]\n  run: Runs the RPC client directly (Best to use 'start' to run in background)\n  start: Runs the RPC client in the background\n  stop: Disconnects the RPC client\n  config: Runs the config CLI";

    if args.len() <= 1 || args.len() < 2 {
        println!("{help_msg}");
        return;
    }

    let mut rpc = RPCState::new(&config);
 
    match &*args[1] {
        "start" => {Command::new("/usr/bin/linuxrpc").arg("run").spawn().unwrap();},
        "run" => {rpc.run_rpc(&mut config).unwrap();},
        "config" => config_prompt(&mut config),
        "stop" => {Command::new("pkill").arg("linuxrpc").output().unwrap();},
        "help" => println!("{help_msg}"),
        _ => println!("{help_msg}")
    }
}   