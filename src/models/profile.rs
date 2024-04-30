use crate::prelude::*;

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Profile {
    pub commands: Vec<Command>,
}

impl Profile {
    pub fn new() -> Self {
        let base_cmd = Command::new("Empty", CommandType::Empty, "");

        let commands = vec![base_cmd; 16];

        Self { commands }
    }
}
