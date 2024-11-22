use sqlite::Error;

use crate::{
    domain::pojo::setting_pojo::Setting, infrastructure::database::sqlite_database::SqliteDBInfra,
};

pub struct SettingRepository;

impl SettingRepository {
    pub fn find_by_key_and_group(key: String, group: String) -> Result<Vec<Setting>, Error> {
        let query = "SELECT * FROM setting WHERE parameter_key = ? AND parameter_group = ?;";
        let param = vec![key, group];
        Ok(
            SqliteDBInfra::execute_param(&SqliteDBInfra::connect()?, query, param)?
                .iter()
                .map(|r| {
                    if let Ok(row) = r {
                        Setting::map(&row)
                    } else {
                        panic!("Could not find mapping")
                    }
                })
                .collect(),
        )
    }

    pub fn save(key: String, group: String, value: String) -> Result<Vec<Setting>, Error> {
        let query =
            "INSERT INTO setting (parameter_key, parameter_group, parameter_value) VALUES (?, ?, ?);";
        let param = vec![key, group, value];
        Ok(
            SqliteDBInfra::execute_param(&SqliteDBInfra::connect()?, query, param)?
                .iter()
                .map(|r| {
                    if let Ok(row) = r {
                        Setting::map(&row)
                    } else {
                        panic!("Failed to save setting")
                    }
                })
                .collect(),
        )
    }

    pub fn update_value(key: String, group: String, value: String) -> Result<Vec<Setting>, Error> {
        let query =
            "UPDATE setting SET parameter_value = ? WHERE parameter_key = ? AND parameter_group = ?;";
        let param = vec![value, key, group];
        Ok(
            SqliteDBInfra::execute_param(&SqliteDBInfra::connect()?, query, param)?
                .iter()
                .map(|r| {
                    if let Ok(row) = r {
                        Setting::map(&row)
                    } else {
                        panic!("Failed to save setting")
                    }
                })
                .collect(),
        )
    }
}
