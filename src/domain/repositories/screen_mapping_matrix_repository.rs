use crate::domain::pojo::screen_mapping_matrix_pojo::ScreenMappingMatrix;
use crate::infrastructure::database::sqlite_database::SqliteDBInfra;
use sqlite::Error;

pub struct ScreenMappingMetricRepository;

impl ScreenMappingMetricRepository {
    pub fn truncate() -> Result<(), Error> {
        let query = "DELETE FROM screen_mapping_matrix;";
        SqliteDBInfra::execute(&SqliteDBInfra::connect()?, query)
    }

    pub fn save(source: String, target: String, edge: String) -> Result<Vec<ScreenMappingMatrix>, Error> {
        let query = "INSERT INTO screen_mapping_matrix (mac_source, mac_target, edge) VALUES (?, ?, ?);";
        let param = vec![source, target, edge];
        Ok(SqliteDBInfra::execute_param(&SqliteDBInfra::connect()?, query, param)?.iter().map(|r| {
            if let Ok(row) = r {
                ScreenMappingMatrix::map(&row)
            } else {
                panic!("Failed to map screen mapping")
            }
        }).collect())
    }

    pub fn find_all() -> Result<Vec<ScreenMappingMatrix>, Error> {
        let query = "SELECT * FROM screen_mapping_matrix;";
        let param = vec![];
        Ok(SqliteDBInfra::execute_param(&SqliteDBInfra::connect()?, query, param)?.iter().map(|r| {
            if let Ok(row) = r {
                ScreenMappingMatrix::map(&row)
            } else {
                panic!("Failed to map screen mapping")
            }
        }).collect())
    }
}