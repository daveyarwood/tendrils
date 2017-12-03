mod migrations;
pub mod actor;

use rusqlite::Connection;
use std::path::PathBuf;

pub fn get_connection(db_file: PathBuf) -> Connection {
    Connection::open(db_file).unwrap()
}

pub fn run_migrations(conn: &Connection)
    -> Result<(), migrations::MigrationError> {
    migrations::run(conn)
}
