use crate::domain::pojo::screen_mapping_refer_pojo::ScreenMappingRefer;
use crate::infrastructure::database::sqlite_database::SqliteDBInfra;
use sqlite::Error;

pub struct ScreenMappingReferRepository;
impl ScreenMappingReferRepository {
    pub fn find_by_key_and_group(
        key: String,
        group: String,
    ) -> Result<Vec<ScreenMappingRefer>, Error> {
        let query =
            "SELECT * FROM screen_mapping_refer WHERE parameter_key = ? AND parameter_group = ?;";
        let param = vec![key, group];
        Ok(
            SqliteDBInfra::execute_param(&SqliteDBInfra::connect()?, query, param)?
                .iter()
                .map(|r| {
                    if let Ok(row) = r {
                        ScreenMappingRefer::map(&row)
                    } else {
                        panic!("Could not find mapping")
                    }
                })
                .collect(),
        )
    }
}
