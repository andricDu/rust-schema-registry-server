use serde::{Deserialize, Serialize};

#[serde(deny_unknown_fields)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    
    #[serde(default = "default_int")]
    id: u64,

    #[serde(default = "default_int")]
    version: u64,

    subject: String,

    format: String,

    definition: String,
}

fn default_int() -> u64 {
    0
}

fn default_str() -> String {
    "".to_string()
}
