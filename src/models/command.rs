use crate::prelude::*;

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Command {
    pub name: String,
    pub command_type: CommandType,
    pub hotkey: String,
    pub highlight: bool,
}

impl Command {
    pub fn new(name: &str, command_type: CommandType, hotkey: &str) -> Self {
        Self {
            name: name.to_string(),
            command_type,
            hotkey: hotkey.to_string(),
            highlight: false,
        }
    }
}
