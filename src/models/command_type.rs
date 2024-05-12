use crate::prelude::*;

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub enum CommandType {
    Empty,
    Link((String, String)),
    Text(String),
}

impl ToString for CommandType {
    fn to_string(&self) -> String {
        match self {
            CommandType::Empty => "".to_string(),
            CommandType::Link(_) => "link".to_string(),
            CommandType::Text(_) => "text".to_string(),
        }
    }
}
