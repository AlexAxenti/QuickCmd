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

pub fn list_commands(json: &FileJson, show_all: bool) {
    let mut sorted_names: Vec<&String> = json.commands.keys().collect();

    sorted_names.sort();

    println!("Available commands:");
    for name in sorted_names {
        if show_all {
            println!(" {name}:\n    {}\n", json.commands[name])
        } else {
            println!(" {name}");
        }
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

pub fn info_command() {
    println!(r#"QuickCmd (qc) stores and executes user-defined shell commands locally.

STORAGE
--------
Your commands are stored in a JSON file on disk.

Typical location on:
Windows:
  %APPDATA%\QuickCmd\qc\data\cmds.json
    (usually C:\Users\<your-user>\AppData\Roaming\QuickCmd\qc\data\cmds.json)

Linux:
  ~/.local/share/quickcmd/qc/cmds.json
    (or $XDG_DATA_HOME/quickcmd/qc/cmds.json)

MacOS:
  ~/Library/Application Support/QuickCmd/qc/cmds.json

FILE SAFETY & BACKUPS
--------------------
When a command is added, updated, or removed:

1. The existing cmds.json is copied to cmds.json.bak
2. Changes are written to cmds.json.tmp
3. The tmp file replaces cmds.json atomically

Only one backup is kept at a time.

If something goes wrong:
1. Locate the data directory mentioned above
2. Rename cmds.json to cmds.json.bad
3. Rename cmds.json.bak to cmds.json
4. Re-run `qc ls`

Do not manually edit the structure of cmds.json.
Changing keys or formatting may corrupt your saved commands.
You may open it read-only to copy commands if needed.

USAGE RECOMMENDATIONS
--------------------
QuickCmd is designed for speed and muscle memory.
Use short aliases whenever possible:

  qc s up "docker compose up -d"
  qc r up

When saving commands that contain shell operators
such as &&, |, >, <, or redirects, wrap the command in quotes:

  qc s test "echo one && echo two"

Using:
  qc s test -- echo one && echo two

will NOT behave the same, because the shell interprets
operators like && before qc receives the command.

TROUBLESHOOTING
---------------
- Permission errors:
    Ensure the data directory is writable and not locked by another process.
- Commands behave differently than expected:
    Check quoting and shell operator usage.
- Commands depend on paths:
    Remember commands run from the directory where qc is invoked."#
);
}