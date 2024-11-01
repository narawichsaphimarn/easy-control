use crate::infrastructure::database::sqlite_database::SqliteDBInfra;
use sqlite::{Error, Row};

pub struct ScreenMappingReferRepository;
impl ScreenMappingReferRepository {
    pub fn find_by_key_and_group(key: String, group: String) -> Result<Vec<Row>, Error> {
        let query = "SELECT * FROM screen_mapping_refer WHERE key = $1 AND group = $2;";
        let param = vec![key, group];
        Ok(SqliteDBInfra::execute_param(&SqliteDBInfra::connect()?, query, param)?)
    }
}