use sqlite::Row;

#[derive(Clone, Debug)]
pub struct ScreenMappingMatrix {
    pub id: i64,
    pub mac_source: String,
    pub mac_target: String,
    pub edge: String,
}

impl ScreenMappingMatrix {
    pub fn maps(rows: Vec<Row>) -> Vec<ScreenMappingMatrix> {
        let mut mappings = Vec::new();
        for row in rows {
            mappings.push(
                Self::map(&row)
            )
        }
        mappings
    }

    pub fn map(row: &Row) -> ScreenMappingMatrix {
        ScreenMappingMatrix {
            id: row.read::<i64, _>("id"),
            mac_source: row.read::<&str, _>("mac_source").to_string(),
            mac_target: row.read::<&str, _>("mac_target").to_string(),
            edge: row.read::<&str, _>("edge").to_string(),
        }
    }
}