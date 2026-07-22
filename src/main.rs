use std::{env::{self}, process::Command};
use crate::{cli::config_prompt, config::Config, rpc::RPCState};

mod config;
mod cli;
mod rpc;

fn main() {
    let mut config = Config::new();
    config.read_config();
    config.read_active_config();

    let args = env::args().collect::<Vec<String>>();
    let help_msg = "Usage: linuxrpc [cmd]\n  run: Runs the RPC client directly (Best to use 'start' to run in background)\n  start: Runs the RPC client in the background\n  stop: Disconnects the RPC client\n  config: Runs the config CLI";

    if args.len() <= 1 || args.len() < 2 {
        println!("{help_msg}");
        return;
    }

    let mut rpc = RPCState::new(&config);
 
    match &*args[1] {
        "start" => {
            let executable_path = Command::new("which")
            .arg("linuxrpc")
            .output()
            .expect("Error trying to retrieve path to executable");

            let program = String::from_utf8(executable_path.stdout).unwrap().trim().to_string();
            Command::new(program).arg("run").spawn().unwrap();
        },
        "run" => {rpc.run_rpc(&mut config).unwrap();},
        "config" => config_prompt(&mut config),
        "stop" => {
            Command::new("pkill").arg("linuxrpc").output().unwrap();
            rpc.stop_rpc(&config);
        },
        "help" => println!("{help_msg}"),
        _ => println!("{help_msg}")
    }
}   