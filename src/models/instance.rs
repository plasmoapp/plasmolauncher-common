use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::loader::Loader;

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct Instance {
    pub name: String,
    pub game_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    pub loader: Loader,
}
