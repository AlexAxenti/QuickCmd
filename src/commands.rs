use crate::model::FileJson;
use std::process::{Command, ExitStatus};

pub fn remove_command(json: &mut FileJson, cmd_name: &str) -> Option<String> {
    json.commands.remove(cmd_name)
}

pub fn save_command(json: &mut FileJson, cmd_name: &str, cmd: &str) {
    json.commands.insert(cmd_name.to_owned(), cmd.to_owned());
}

pub fn show_command<'a>(json: &'a FileJson, cmd_name: &str) -> Option<&'a str> {
    json.commands.get(cmd_name).map(|v| v.as_str())
}

pub fn list_commands(json: &FileJson) {
    let mut sorted_names: Vec<&String> = json.commands.keys().collect();

    sorted_names.sort();

    println!("Available commands:");
    for name in sorted_names {
        println!(" {name}");
    }
}

pub fn run_command(json: &FileJson, cmd_name: &str) -> Result<(), String> {
    let cmd = json.commands.get(cmd_name).ok_or_else(|| format!("Unable to find command: {cmd_name}"))?;

    let status = run_in_shell(cmd).map_err(|err| format!("Failed to run command: {err}"))?;

    if status.success() {
        return Ok(())
    } else {
        return Err(format!("Command exited with status: {status}"))
    }
}

fn run_in_shell(cmd: &str) -> std::io::Result<ExitStatus> {
    if cfg!(windows) {
        Command::new("cmd")
            .args(["/C", cmd])
            .status()
    } else {
        Command::new("sh")
            .args(["-c", cmd])
            .status()
    }
}

