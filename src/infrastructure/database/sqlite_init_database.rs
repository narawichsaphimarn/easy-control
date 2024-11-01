use crate::infrastructure::database::sqlite_database::{SqliteDBInfra, SqliteDBInfraInit};
use sqlite::Error;

impl SqliteDBInfraInit for SqliteDBInfra {
    fn init() -> Result<(), Error> {
        let query = "
            CREATE TABLE IF NOT EXISTS screen_selector (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ip varchar(255) NOT NULL UNIQUE,
            mac varchar(255) NOT NULL UNIQUE,
            hostname varchar(255) NOT NULL,
            width varchar(255) NOT NULL,
            height varchar(255) NOT NULL);

            CREATE TABLE IF NOT EXISTS screen_mapping_metric (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            mac_source varchar(255) NOT NULL,
            mac_target varchar(255) NOT NULL,
            edge varchar(25) NOT NULL);

            CREATE TABLE IF NOT EXISTS screen_mapping_refer (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            parameter_key varchar(255) NOT NULL,
            parameter_group varchar(255) NOT NULL,
            parameter_value varchar(255) NOT NULL);

            INSERT OR REPLACE INTO screen_mapping_refer (id, parameter_key, parameter_group, parameter_value) VALUES
            (1, 'SCREEN_NUMBER', '1', '2,4'),
            (2, 'SCREEN_NUMBER', '2', '1,3,5'),
            (3, 'SCREEN_NUMBER', '3', '2,6'),
            (4, 'SCREEN_NUMBER', '4', '1,5,7'),
            (5, 'SCREEN_NUMBER', '5', '2,4,6,8'),
            (6, 'SCREEN_NUMBER', '6', '3,5,9'),
            (7, 'SCREEN_NUMBER', '7', '4,8'),
            (8, 'SCREEN_NUMBER', '8', '7,5,9'),
            (9, 'SCREEN_NUMBER', '9', '6,8');
            ";
        let conn = Self::connect()?;
        Ok(Self::execute(&conn, query)?)
    }
}