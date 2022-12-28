use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct ModLocale {
    pub title: String,
    pub description: Option<String>
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
#[serde(rename_all = "lowercase")]
pub enum LocaleKey {
    Ru,
    En,
}