use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    version,
    name = "qc", 
    about = "QuickCmd - save and run frequently used shell commands",
    arg_required_else_help = true,
    subcommand_required = true,
    subcommand_help_heading = "Commands", 
    long_about = None,
    after_help = r#"Examples:
    qc s up -- docker compose up -d
    qc r up"#,
)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(
        alias = "s",
        // visible_alias = "s",
        about = "Save a command (alias: s)",
        long_about = r#"
Save a shell command under a name.

Everything after `--` is stored exactly as written.

Examples:
    qc s up -- docker compose up -d
    qc save logs "docker logs -f app""#
    )]
    Save { 
        name: String,
        #[arg(trailing_var_arg = true)]
        cmd: Vec<String>,
    },
    #[command(
        alias = "r",
        // visible_alias = "r",
        about = "Run a command (alias: r)",
        long_about = r#"
Run a saved command

Examples:
    qc s up -- docker compose up -d
    qc r up
    qc run up"#
    )]
    Run {
        name: String,
    },
    #[command(
        alias = "ls",
        // visible_alias = "ls",
        about = "List all commands (alias: ls)",
        long_about = r#"
List all saved commands

Examples:
    qc ls
    qc list"#
    )]
    List {
        /// Show full command text for each entry
        #[arg(short = 'a', long = "all")]
        all: bool
    },
    #[command(
        alias = "sh",
        // visible_alias = "sh",
        about = "Show the command saved to a name (alias: sh)",
        long_about = r#"
Show the command saved to a name

Examples:
    qc s up -- docker compose up -d
    qc sh up
    qc show up"#
    )]
    Show {
        name: String,
    },
    #[command(
        alias = "rm",
        // visible_alias = "rm",
        about = "Remove a command (alias: rm)",
        long_about = r#"
Remove a saved command

Examples:
    qc s up -- docker compose up -d
    qc rm up
    qc remove up"#
    )]
    Remove {
        name: String
    },
    #[command(
        about = "Info about QuickCmd",
        long_about = r#"
Info about QuickCmd such as:
- Path used
- Debugging tips
- Usage recommendations, etc."#
    )]
    Info
}