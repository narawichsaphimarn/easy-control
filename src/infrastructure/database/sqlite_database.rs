use sqlite::{Connection, Error, Row};
use std::collections::HashMap;
use std::env;

pub struct SqliteDBInfra;

pub trait SqliteDBInfraInit {
    fn init() -> Result<(), Error>;
}

impl SqliteDBInfra {
    pub fn connect() -> Result<Connection, Error> {
        let host = env::var("DATABASE_NAME").expect("DATABASE_URL must be set");
        Ok(sqlite::open(host)?)
    }

    pub fn execute(db_conn: &Connection, query: &str) -> Result<(), Error> {
        Ok(db_conn.execute(query)?)
    }

    pub fn execute_param(db_conn: &Connection, query: &str, param: Vec<String>) -> Result<Vec<Row>, Error> {
        let mut statement = db_conn.prepare(query)?;
        for (index, value) in param.iter().enumerate() {
            statement.bind((index + 1, value.as_str()))?;
        }
        Ok(statement.iter().map(|row| row.unwrap()).collect())
    }

    pub fn execute_param_hashmap(db_conn: &Connection, query: &str, param: HashMap<&str, String>) -> Result<Vec<Row>, Error> {
        let mut statement = db_conn.prepare(query)?;
        for (key, value) in param {
            statement.bind(((":".to_owned() + key).as_str(), value.as_str()))?;
        }
        Ok(statement.iter().map(|row| row.unwrap()).collect())
    }
}