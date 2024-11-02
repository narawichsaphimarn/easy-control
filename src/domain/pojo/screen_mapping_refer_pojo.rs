use sqlite::Row;

#[derive(Clone, Debug)]
pub struct ScreenMappingRefer {
    pub id: i64,
    pub parameter_key: String,
    pub parameter_group: String,
    pub parameter_value: String,
}

impl ScreenMappingRefer {
    pub fn maps(rows: Vec<Row>) -> Vec<ScreenMappingRefer> {
        let mut mappings = Vec::new();
        for row in rows {
            mappings.push(
                Self::map(&row)
            )
        }
        mappings
    }

    pub fn map(row: &Row) -> ScreenMappingRefer {
        ScreenMappingRefer {
            id: row.read::<i64, _>("id"),
            parameter_key: row.read::<&str, _>("parameter_key").to_string(),
            parameter_group: row.read::<&str, _>("parameter_group").to_string(),
            parameter_value: row.read::<&str, _>("parameter_value").to_string(),
        }
    }
}