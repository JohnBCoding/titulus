use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    pub commands: Vec<Command>,
}

impl Profile {
    pub fn new() -> Self {
        let base_cmd = Command::new(
            "Reddittttttttttt",
            CommandType::Link("https://www.reddit.com".to_string()),
            "a",
        );

        let commands = vec![base_cmd; 16];

        Self { commands }
    }
}
