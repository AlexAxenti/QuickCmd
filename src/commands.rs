use crate::model::FileJson;

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
