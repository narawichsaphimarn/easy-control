use sqlite::{Connection, Error, Statement};
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

    pub fn execute_param<'a>(
        db_conn: &'a Connection,
        query: &'a str,
        param: Vec<String>,
    ) -> Result<Statement<'a>, Error> {
        let mut statement = db_conn.prepare(query)?;
        for (index, value) in param.iter().enumerate() {
            statement.bind((index + 1, value.as_str()))?;
        }
        Ok(statement)
    }

    pub fn execute_param_hashmap<'a>(
        db_conn: &'a Connection,
        query: &'a str,
        param: HashMap<&'a str, String>,
    ) -> Result<Statement<'a>, Error> {
        let mut statement = db_conn.prepare(query)?;
        for (key, value) in param {
            statement.bind(((":".to_owned() + key).as_str(), value.as_str()))?;
        }
        Ok(statement)
    }
}
