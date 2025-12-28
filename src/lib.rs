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

    // Guard further logic if it's not in the enum of subcommands
    let Some(command) = cli.command else {
        return Ok(())
    };

    run_sub_command(&command)?;

    Ok(())
}

fn run_sub_command(sub_command: &Commands) -> Result<(), String> {
    let cmds_file_path = create_path()?;

    if !cmds_file_path.exists() {
        initialize_file(&cmds_file_path)?;
    }

    let mut json = read_cmd_file_contents(&cmds_file_path)?;
        
    let mut is_dirty = false;

    match sub_command {
        Commands::Save { name, cmd } => {
            let cmd = cmd.join(" ");
            save_command(&mut json, &name, &cmd);
            is_dirty = true;
            println!("Saved command: \n {name}: {cmd}");
        },
        Commands::Run { name } => {
            run_command(&json, &name)?;
        },
        Commands::List { all } => {
            list_commands(&json, *all);
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
        },
        Commands::Info => {
            info_command();
        }
    }

    if is_dirty {
        let contents = serde_json::to_string_pretty(&json)
            .map_err(|err| format!("Failed to serialize commands file: {err}"))?;
        update_file(&cmds_file_path, &contents)?; 
    }

    Ok(())
}
