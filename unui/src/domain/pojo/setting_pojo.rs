use serde::{Deserialize, Serialize};
use sqlite::Row;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Setting {
    pub id: i64,
    pub parameter_key: String,
    pub parameter_group: String,
    pub parameter_value: String,
}

impl Setting {
    pub fn maps(rows: Vec<Row>) -> Vec<Setting> {
        let mut mappings = Vec::new();
        for row in rows {
            mappings.push(Self::map(&row));
        }
        mappings
    }

    pub fn map(row: &Row) -> Setting {
        Setting {
            id: row.read::<i64, _>("id"),
            parameter_key: row.read::<&str, _>("parameter_key").to_string(),
            parameter_group: row.read::<&str, _>("parameter_group").to_string(),
            parameter_value: row.read::<&str, _>("parameter_value").to_string(),
        }
    }
}
