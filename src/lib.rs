mod cli;
mod model;

use cli::{Cli, Commands};
use model::FileJson;
use clap::Parser;
use std::path::PathBuf;
use std::fs;
use std::io::{Error, ErrorKind, Write};
use directories_next::ProjectDirs;
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

fn update_file(path: &PathBuf, contents: &str) -> Result<(), String> {
    let tmp_path = path.with_added_extension("tmp");
    let bak_path = path.with_added_extension("bak");

    write_and_sync(&tmp_path, contents).map_err(|err| format!("Failed to write to tmp file: {err}"))?; 

    delete_file_if_exists(&bak_path)?;
    rename_file_if_exists(&path, &bak_path)?;
    rename_file_if_exists(&tmp_path, &path)?; //TODO add rollback incase this fails, as the original json is required

    Ok(())
}

fn write_and_sync(path: &PathBuf, contents: &str) -> Result<(), Error> {
    let mut f = fs::File::create(&path)?;
    f.write_all(contents.as_bytes())?;
    f.sync_all()?;

    Ok(())
}

fn delete_file_if_exists(path: &PathBuf) -> Result<(), String> {
    match fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(()),
        Err(err) => Err(format!("Failed to remove old file: {err}"))
    }
}

fn rename_file_if_exists(old_path: &PathBuf, new_path: &PathBuf) -> Result<(), String> {
    match fs::rename(old_path, new_path) {
        Ok(_) => Ok(()),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(()),
        Err(err) => Err(format!("Failed to rename file: {err}"))
    }
}

fn remove_command(json: &mut FileJson, cmd_name: &str) -> Option<String> {
    json.commands.remove(cmd_name)
}

fn save_command(json: &mut FileJson, cmd_name: &str, cmd: &str) {
    json.commands.insert(cmd_name.to_owned(), cmd.to_owned());
}

fn show_command<'a>(json: &'a FileJson, cmd_name: &str) -> Option<&'a str> {
    json.commands.get(cmd_name).map(|v| v.as_str())
}

fn list_commands(json: &FileJson) {
    let mut sorted_names: Vec<&String> = json.commands.keys().collect();

    sorted_names.sort();

    println!("Available commands:");
    for name in sorted_names {
        println!(" {name}");
    }
}

fn read_cmd_file(path: &PathBuf) -> Result<FileJson, String> {
    println!("{:#?}", path);
    let contents = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read commands file at {}: {}", path.display(), e))?;

    let json: FileJson = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse commands file at {}: {}", path.display(), e))?;

    Ok(json)
}

fn check_and_create_file() -> Result<PathBuf, String> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "QuickCmd", "qc") {
        let app_data_dir = proj_dirs.data_dir();

        let cmds_file_path = app_data_dir.join("cmds.json");

        fs::create_dir_all(app_data_dir)
            .map_err(|err| format!("Unable to create commands directory: {err}"))?;

        if !cmds_file_path.exists() {
            let json = FileJson::new();
            let contents = serde_json::to_string_pretty(&json)
                .map_err(|err| format!("Failed to serialize initialize commands file: {err}"))?;

            write_and_sync(&cmds_file_path, &contents)
                .map_err(|err| format!("Failed to write while initializing commands file: {err}"))?;
        }

        return Ok(cmds_file_path);

    } else {
        Err(String::from("Unable to find path"))
    }
}