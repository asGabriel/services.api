use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub tag_id: i32,
    pub value: String,
}
