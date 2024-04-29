use crate::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Command {
    pub name: String,
    pub command_type: CommandType,
    pub hotkey: String,
}

impl Command {
    pub fn new(name: &str, command_type: CommandType, hotkey: &str) -> Self {
        Self {
            name: name.to_string(),
            command_type,
            hotkey: hotkey.to_string(),
        }
    }
}
