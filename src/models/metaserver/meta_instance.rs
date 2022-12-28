use std::{collections::HashMap, path::PathBuf};

use serde::{Serialize, Deserialize};
use ts_rs::TS;
use crate::models::{instance::{Instance}, artifact::Artifact, locale::{ModLocale}};

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct MetaInstance {
    pub instance: Instance,
    pub files: Vec<MetaFile>,
    pub mods: Vec<MetaMod>,
}

impl MetaInstance {
    pub fn new(
        instance: Instance,
        files: Vec<MetaFile>,
        mods: Vec<MetaMod>
    ) -> Self {
        Self {
            instance,
            files,
            mods,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum ModCategory {
    None,
    Test,
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct MetaFile {
    pub path: PathBuf,
    pub validate: bool,
    pub artifact: Artifact,
}

impl MetaFile {
    pub fn new(path: PathBuf, validate: bool, artifact: Artifact) -> Self {
        Self {
            path,
            validate,
            artifact,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct MetaMod {
    pub id: String,
    pub version: String,
    pub optional: bool,
    pub artifact: Artifact,
    pub recommended: bool,
    pub category: ModCategory,
    pub locale: HashMap<String, ModLocale>
}

impl MetaMod {
    pub fn jar_name(&self) -> String {
        format!("{MOD_SPLIT_KEY}{id}{MOD_SPLIT_KEY}{version}{MOD_SPLIT_KEY}.jar", id = self.id, version = self.version)
    }
}

pub const MOD_SPLIT_KEY: &'static str = "###";

impl MetaMod {
    pub fn new(
        id: impl Into<String>,
        version: impl Into<String>,
        optional: bool,
        artifact: Artifact,
        recommended: bool,
        category: Option<ModCategory>,
        locale: HashMap<String, ModLocale>
    ) -> Self {
        Self {
            id: id.into(),
            version: version.into(),
            optional,
            artifact,
            recommended,
            category: category.unwrap_or(ModCategory::None),
            locale,
        }
    }
}

pub type MetaInstances = HashMap<String, MetaInstance>;

// pub fn pepega() {
//     let pepega = MetaInstances::new();
// }