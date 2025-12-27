use crate::model::FileJson;

use directories_next::ProjectDirs;
use serde_json;
use std::io::{Error, ErrorKind, Write};
use std::fs;
use std::path::PathBuf;

pub fn update_file(path: &PathBuf, contents: &str) -> Result<(), String> {
    let tmp_path = path.with_added_extension("tmp");
    let bak_path = path.with_added_extension("bak");

    write_and_sync(&tmp_path, contents).map_err(|err| format!("Failed to write to tmp file: {err}"))?; 

    delete_file_if_exists(&bak_path)?;
    rename_file_if_exists(&path, &bak_path)?;
    rename_file_if_exists(&tmp_path, &path)?; //TODO add rollback incase this fails, as the original json is required

    Ok(())
}

pub fn write_and_sync(path: &PathBuf, contents: &str) -> Result<(), Error> {
    let mut f = fs::File::create(&path)?;
    f.write_all(contents.as_bytes())?;
    f.sync_all()?;

    Ok(())
}

pub fn delete_file_if_exists(path: &PathBuf) -> Result<(), String> {
    match fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(()),
        Err(err) => Err(format!("Failed to remove old file: {err}"))
    }
}

pub fn rename_file_if_exists(old_path: &PathBuf, new_path: &PathBuf) -> Result<(), String> {
    match fs::rename(old_path, new_path) {
        Ok(_) => Ok(()),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(()),
        Err(err) => Err(format!("Failed to rename file: {err}"))
    }
}

pub fn read_cmd_file_contents(path: &PathBuf) -> Result<FileJson, String> {
    println!("{:#?}", path);
    let contents = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read commands file at {}: {}", path.display(), e))?;

    let json: FileJson = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse commands file at {}: {}", path.display(), e))?;

    Ok(json)
}

pub fn check_or_create_file() -> Result<PathBuf, String> {
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