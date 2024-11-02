use crate::domain::pojo::screen_selector_pojo::ScreenSelector;
use crate::infrastructure::database::sqlite_database::SqliteDBInfra;
use sqlite::Error;
use std::collections::HashMap;

pub struct ScreenSelectorRepository;

impl ScreenSelectorRepository {
    pub fn truncate() -> Result<(), Error> {
        let query = "DELETE FROM screen_selector;";
        SqliteDBInfra::execute(&SqliteDBInfra::connect()?, query)
    }

    pub fn save(ip: String, mac: String, hostname: String, width: String, height: String) -> Result<Vec<ScreenSelector>, Error> {
        let query = "INSERT INTO screen_selector (ip, mac, hostname, width, height) VALUES (:ip, :mac, :hostname, :width, :height);";
        let mut param = HashMap::new();
        param.insert("ip", ip);
        param.insert("mac", mac);
        param.insert("hostname", hostname);
        param.insert("width", width);
        param.insert("height", height);
        Ok(SqliteDBInfra::execute_param_hashmap(&SqliteDBInfra::connect()?, query, param)?.iter().map(|r| {
            if let Ok(row) = r {
                ScreenSelector::map(&row)
            } else {
                panic!("Could not find screen selector")
            }
        }).collect())
    }


    pub fn find_all() -> Result<Vec<ScreenSelector>, Error> {
        let query = "SELECT * FROM screen_selector;";
        let param = vec![];
        Ok(SqliteDBInfra::execute_param(&SqliteDBInfra::connect()?, query, param)?.iter().map(|r| {
            if let Ok(row) = r {
                ScreenSelector::map(&row)
            } else {
                panic!("Could not find screen selector")
            }
        }).collect())
    }
}