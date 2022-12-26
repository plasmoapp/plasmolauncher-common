use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(tag = "type", content = "version")]
#[ts(export)]
pub enum Loader {
    Vanilla,
    Fabric(String),
}

impl Loader {
    pub fn get_type_str(&self) -> &str {
        match self {
            Loader::Vanilla => "vanilla",
            Loader::Fabric(_) => "fabric",
        }
    } 
}