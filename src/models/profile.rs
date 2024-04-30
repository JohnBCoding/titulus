use crate::prelude::*;

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Profile {
    pub commands: Vec<Command>,
    pub search_template: String,
}

impl Profile {
    pub fn new() -> Self {
        let base_cmd = Command::new("Empty", CommandType::Empty, "");

        let commands = vec![base_cmd; 16];

        Self {
            commands,
            search_template: "https://duckduckgo.com/?q={}".to_string(),
        }
    }
}
