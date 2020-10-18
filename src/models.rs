use serde::{Deserialize, Serialize};

use crate::schema::schemas;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Schema {
    #[serde(default = "default_int")]
    pub id: i32,
    #[serde(default = "default_int")]
    pub version: i32,
    pub subject: String,
    pub format: String,
    pub definition: String,
}

impl PartialEq<Schema> for Schema {
    fn eq(&self, other: &Schema) -> bool {
        (self.id == other.id) && (self.version == other.version) && (self.subject == other.subject)
    }
}

#[serde(deny_unknown_fields)]
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "schemas"]
pub struct NewSchema {
    pub version: i32,
    pub subject: String,
    pub format: String,
    pub definition: String,
}

#[serde(deny_unknown_fields)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSchemaRequest {
    pub subject: String,
    pub format: String,
    pub definition: String,
}

fn default_int() -> i32 {
    0
}
