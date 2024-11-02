use sqlite::Row;

#[derive(Clone, Debug)]
pub struct ScreenSelector {
    pub id: i64,
    pub ip: String,
    pub mac: String,
    pub hostname: String,
    pub width: String,
    pub height: String,
}

impl ScreenSelector {
    pub fn maps(rows: Vec<Row>) -> Vec<ScreenSelector> {
        let mut mappings = Vec::new();
        for row in rows {
            mappings.push(
                Self::map(&row)
            )
        }
        mappings
    }

    pub fn map(row: &Row) -> ScreenSelector {
        ScreenSelector {
            id: row.read::<i64, _>("id"),
            ip: row.read::<&str, _>("ip").to_string(),
            mac: row.read::<&str, _>("mac").to_string(),
            hostname: row.read::<&str, _>("hostname").to_string(),
            width: row.read::<&str, _>("width").to_string(),
            height: row.read::<&str, _>("height").to_string(),
        }
    }
}