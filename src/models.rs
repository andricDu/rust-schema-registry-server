use serde::{Deserialize, Serialize};

use crate::schema::schemas;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Schema {
    #[serde(default = "default_int")]
    pub id: u64,
    #[serde(default = "default_int")]
    pub version: u64,
    pub subject: String,
    pub format: String,
    pub definition: String,
}

#[serde(deny_unknown_fields)]
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name="schemas"]
pub struct NewSchema {
    pub subject: String,
    pub format: String,
    pub definition: String,
}

fn default_int() -> u64 {
    0
}