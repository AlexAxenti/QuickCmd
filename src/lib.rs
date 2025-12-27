mod cli;
mod model;
mod commands;
mod file_management;

use cli::{Cli, Commands};
use commands::{*};
use file_management::{*};

use clap::Parser;
use serde_json;

pub fn run() -> Result<(), String> {
    let cli = Cli::parse();

    let Some(command) = cli.command else {
        return Ok(())
    };

    let cmds_path = check_and_create_file()?;

    let mut json = read_cmd_file(&cmds_path)?;

    println!("{:#?}", cmds_path);
        
    let mut is_dirty = false;

    match command {
        Commands::Save { name, cmd } => {
            let cmd = cmd.join(" ");
            save_command(&mut json, &name, &cmd);
            is_dirty = true;
            println!("Saved command: \n {name}: {cmd}");
        },
        Commands::Run { name } => {
            println!("run command called: {name}");
        },
        Commands::List => {
            list_commands(&json);
        },
        Commands::Show { name } => {
            match show_command(&json, &name) {
                Some(cmd_value) => println!("{name}: {cmd_value}"),
                None => eprintln!("Unable to find command: {name}"),
            }
        }, 
        Commands::Remove { name } => {
            match remove_command(&mut json, &name) {
                Some(removed_cmd) => { 
                    is_dirty = true;
                    println!("Removed command: \n{name}: {removed_cmd}") 
                },
                None => eprintln!("Unable to find command: {name}"),
            }
        }
    }

    if is_dirty {
        let contents = serde_json::to_string_pretty(&json)
            .map_err(|err| format!("Failed to serialize commands file: {err}"))?;
        update_file(&cmds_path, &contents)?; 
    }

    Ok(())
}