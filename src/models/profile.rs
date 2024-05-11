use crate::prelude::*;

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Profile {
    #[serde(default = "default_commands")]
    pub commands: Vec<Command>,
    #[serde(default = "default_search")]
    pub search_template: String,
    #[serde(default = "default_proxies")]
    pub proxies: Vec<String>,
    #[serde(default)]
    pub current_proxy: usize,
    #[serde(default = "default_themes")]
    pub themes: Vec<(String, String)>,
    #[serde(default)]
    pub current_theme: usize,
}

impl Profile {
    pub fn new() -> Self {
        Self {
            commands: default_commands(),
            search_template: default_search(),
            proxies: default_proxies(),
            current_proxy: 0,
            themes: default_themes(),
            current_theme: 0,
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
    pub fn get_current_proxy(&mut self) -> String {
        let current = self.proxies[self.current_proxy].clone();
        self.current_proxy += 1;
        if self.current_proxy >= self.proxies.len() {
            self.current_proxy = 0;
        }

        current
    }

    // Updates current theme index then returns the theme at that index
    pub fn update_theme(&mut self, new_index: usize) -> (String, String) {
        self.current_theme = new_index;
        if self.current_theme >= self.themes.len() {
            self.current_theme = 0;
        }

        self.themes[self.current_theme].clone()
    }
}

fn default_commands() -> Vec<Command> {
    let base_cmd = Command::new("Empty", CommandType::Empty, "");
    vec![base_cmd; 16]
}

fn default_search() -> String {
    "https://duckduckgo.com/?q={}".to_string()
}

fn default_proxies() -> Vec<String> {
    vec![
        "https://corsproxy.io/?".to_string(),
        "https://api.allorigins.win/raw?url=".to_string(),
    ]
}

fn default_themes() -> Vec<(String, String)> {
    vec![
        ("default".to_string(), "Default Theme".to_string()),
        ("titanstone".to_string(), "Titanstone".to_string()),
        ("lava-gb".to_string(), "Lava-GB".to_string()),
        ("timeless-night".to_string(), "Timeless Night".to_string()),
        ("vireo-4".to_string(), "Vireo-4".to_string()),
    ]
}
