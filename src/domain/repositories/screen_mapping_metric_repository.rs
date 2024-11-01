use crate::infrastructure::database::sqlite_database::SqliteDBInfra;
use sqlite::{Error, Row};

pub struct ScreenMappingMetricRepository;

impl ScreenMappingMetricRepository {
    pub fn truncate() -> Result<(), Error> {
        let query = "DELETE FROM screen_mapping_metric;";
        SqliteDBInfra::execute(&SqliteDBInfra::connect()?, query)
    }

    pub fn save(source: String, target: String, edge: String) -> Result<Vec<Row>, Error> {
        let query = "INSERT INTO screen_mapping_metric (mac_source, mac_target, edge) VALUES (?, ?, ?);";
        let param = vec![source, target, edge];
        Ok(SqliteDBInfra::execute_param(&SqliteDBInfra::connect()?, query, param)?)
    }
}