use crate::prelude::*;

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Profile {
    pub commands: Vec<Command>,
    #[serde(default = "default_search")]
    pub search_template: String,
    #[serde(default = "default_proxy")]
    pub proxies: Vec<String>,
    pub current_proxy: usize,
}

fn default_search() -> String {
    "https://duckduckgo.com/?q={}".to_string()
}

fn default_proxy() -> Vec<String> {
    vec![
        "https://corsproxy.io/?".to_string(),
        "https://thingproxy.freeboard.io/fetch/".to_string(),
    ]
}

impl Profile {
    pub fn new() -> Self {
        let base_cmd = Command::new("Empty", CommandType::Empty, "");

        let commands = vec![base_cmd; 16];

        Self {
            commands,
            search_template: "https://duckduckgo.com/?q={}".to_string(),
            proxies: vec![
                "https://corsproxy.io/?".to_string(),
                "https://thingproxy.freeboard.io/fetch/".to_string(),
            ],
            current_proxy: 0,
        }
    }

    /// Checks if hotkey matches any command, highlights it if so
    pub fn check_hotkey(&mut self, hotkey: &str) -> bool {
        let mut found = false;
        self.commands.iter_mut().for_each(|command| {
            command.highlight = false;
            if command.hotkey == hotkey && !hotkey.is_empty() {
                command.highlight = true;
                found = true;
            }
        });

        found
    }

    // Returns current proxy then updates to a new proxy.
    pub fn get_random_proxy(&mut self) -> String {
        let current = self.proxies[self.current_proxy].clone();
        self.current_proxy += 1;
        if self.current_proxy >= self.proxies.len() {
            self.current_proxy = 0;
        }

        current
    }
}
