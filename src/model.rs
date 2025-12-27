use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct FileJson {
    #[allow(dead_code)] // not read but used in serialization
    pub version: u8,
    pub commands: HashMap<String, String>,
}

impl FileJson {
    pub fn new() -> Self {
        Self {
            version: 1,
            commands: HashMap:: new(),
        }
    }
}