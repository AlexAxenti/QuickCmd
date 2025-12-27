use clap::{Parser, Subcommand};
use std::{collections::HashMap, path::PathBuf};
use directories_next::ProjectDirs;
use std::fs::{File,create_dir_all};
use std::io::Error;

#[derive(Parser)]
#[command(
    version,
    name = "qc", 
    about = "QuickCmd - save and run frequently used shell commands",
    arg_required_else_help = true,
    subcommand_required = true,
    subcommand_help_heading = "Commands", 
    long_about = None,
    after_help = r#"
Examples:
    qc s up -- docker compose up -d
    qc r up
"#,
)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(
        alias = "s",
        // visible_alias = "s",
        about = "Save a command (alias: s)",
        long_about = r#"
Save a shell command under a name.

Everything after `--` is stored exactly as written.

Examples:
    qc s up -- docker compose up -d
    qc save logs "docker logs -f app"
"#
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
    qc run up
"#
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
    qc list
"#
    )]
    List,
    #[command(
        alias = "sh",
        // visible_alias = "sh",
        about = "Show the command saved to a name (alias: sh)",
        long_about = r#"
Show the command saved to a name

Examples:
    qc s up -- docker compose up -d
    qc sh up
    qc show up
"#
    )]
    Show {
        name: String,
    },
    #[command(
        alias = "rm",
        // visible_alias = "rm",
        about = "Save a command (alias: rm)",
        long_about = r#"
Remove a saved command

Examples:
    qc s up -- docker compose up -d
    qc rm up
    qc remove up
"#
    )]
    Remove {
        name: String
    }
}


fn main() {
    let cli = Cli::parse();

    let cmds_path = check_and_create_file().unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        std::process::exit(1);
    });
        

    match &cli.command {
        Commands::Save { name, cmd } => {
            check_and_create_file();
            println!("save command called: {name} {:#?}", cmd);
        },
        Commands::Run { name } => {
            check_and_create_file();
            println!("run command called: {name}");
        },
        Commands::List => {
            check_and_create_file();
            println!("list command called");
        },
        Commands::Show { name } => {
            check_and_create_file();
            println!("show command called: {name}")
        }, 
        Commands::Remove { name } => {
            check_and_create_file();
            println!("remove command called: {name}")
        }
    }
}

struct FileJson {
    version: String,
    commands: HashMap<String, String>,
}

fn check_and_create_file() -> Result<PathBuf, String> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "QuickCmd", "qc") {
        let app_data_dir = proj_dirs.data_dir();
        println!("{:?}", app_data_dir);

        let cmds_file_path = app_data_dir.join("cmds.json");

        if let Err(_) = create_dir_all(app_data_dir) {
            return Err(String::from("Unable to create commands file"));
        }

        if !cmds_file_path.exists() {
            if let Err(_) = File::create(&cmds_file_path) {
                return Err(String::from("Unable to create commands file"));
            }
        }
        return Ok(cmds_file_path);

    } else {
        Err(String::from("Unable to find path"))
    }
}