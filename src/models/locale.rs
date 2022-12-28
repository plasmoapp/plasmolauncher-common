use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct ModLocale {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>
}

#[derive(Debug, Deserialize, Serialize, Clone, TS, PartialEq, Eq, Hash)]
#[ts(export)]
#[serde(rename_all = "lowercase")]
pub enum LocaleKey {
    Ru,
    En,
}