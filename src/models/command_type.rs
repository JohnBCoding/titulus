use crate::prelude::*;

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub enum CommandType {
    Empty,
    Link(String),
}
