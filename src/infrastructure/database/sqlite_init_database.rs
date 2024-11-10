use crate::infrastructure::database::sqlite_database::{SqliteDBInfra, SqliteDBInfraInit};
use sqlite::Error;

impl SqliteDBInfraInit for SqliteDBInfra {
    fn init() -> Result<(), Error> {
        let query =
            "
            CREATE TABLE IF NOT EXISTS screen_selector (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ip varchar(255) NOT NULL UNIQUE,
            mac varchar(255) NOT NULL UNIQUE,
            hostname varchar(255) NOT NULL,
            width varchar(255) NOT NULL,
            height varchar(255) NOT NULL);

            CREATE TABLE IF NOT EXISTS screen_mapping_matrix (
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
            (9, 'SCREEN_NUMBER', '9', '6,8'),
            (10, 'SCREEN_NUMBER', '1,2', 'RIGHT'),
            (11, 'SCREEN_NUMBER', '1,4', 'BOTTOM'),
            (12, 'SCREEN_NUMBER', '2,1', 'LEFT'),
            (13, 'SCREEN_NUMBER', '2,3', 'RIGHT'),
            (14, 'SCREEN_NUMBER', '2,5', 'BOTTOM'),
            (15, 'SCREEN_NUMBER', '3,2', 'LEFT'),
            (16, 'SCREEN_NUMBER', '3,6', 'BOTTOM'),
            (17, 'SCREEN_NUMBER', '4,1', 'TOP'),
            (18, 'SCREEN_NUMBER', '4,5', 'RIGHT'),
            (19, 'SCREEN_NUMBER', '4,7', 'BOTTOM'),
            (20, 'SCREEN_NUMBER', '5,2', 'TOP'),
            (21, 'SCREEN_NUMBER', '5,4', 'LEFT'),
            (22, 'SCREEN_NUMBER', '5,6', 'RIGHT'),
            (23, 'SCREEN_NUMBER', '5,8', 'BOTTOM'),
            (24, 'SCREEN_NUMBER', '6,3', 'TOP'),
            (25, 'SCREEN_NUMBER', '6,5', 'LEFT'),
            (26, 'SCREEN_NUMBER', '6,9', 'BOTTOM'),
            (27, 'SCREEN_NUMBER', '7,4', 'TOP'),
            (28, 'SCREEN_NUMBER', '7,8', 'RIGHT'),
            (29, 'SCREEN_NUMBER', '8,5', 'TOP'),
            (30, 'SCREEN_NUMBER', '8,7', 'LEFT'),
            (31, 'SCREEN_NUMBER', '8,9', 'RIGHT'),
            (32, 'SCREEN_NUMBER', '9,6', 'TOP'),
            (33, 'SCREEN_NUMBER', '9,8', 'LEFT');

            CREATE TABLE IF NOT EXISTS setting (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            parameter_key varchar(255) NOT NULL,
            parameter_group varchar(255) NOT NULL,
            parameter_value varchar(255) NOT NULL);
            ";
        let conn = Self::connect()?;
        Ok(Self::execute(&conn, query)?)
    }
}
