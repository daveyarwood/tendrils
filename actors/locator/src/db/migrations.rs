use rusqlite;
use schemamama;
use schemamama_rusqlite::{SqliteAdapter, SqliteMigration, SqliteMigrationError};

pub type MigrationError = schemamama::Error<SqliteMigrationError>;

macro_rules! locator_migration {
    ( $version:expr, $migration:ident, $desc:expr, $up:expr, $down:expr) => {
        struct $migration;
        migration!($migration, $version, $desc);

        impl SqliteMigration for $migration {
            fn up(&self, conn: &rusqlite::Connection) -> rusqlite::Result<()> {
                conn.execute($up, &[]).map(|_| ())
            }

            fn down(&self, conn: &rusqlite::Connection) -> rusqlite::Result<()> {
                conn.execute($down, &[]).map(|_| ())
            }
        }
    }
}

locator_migration!(1481764405, CreateActors, "create actors table",
                   "CREATE TABLE locator_actor(
                      id TEXT PRIMARY KEY NOT NULL,
                      location TEXT,
                      lastseen INTEGER NOT NULL,
                      online INTEGER NOT NULL
                    );",
                    "DROP TABLE locator_actor;");

macro_rules! register {
    ( $migrator:expr, $( $migration:ident ),* ) => {
        $(
            $migrator.register(Box::new($migration));
         )*
    }
}

pub fn run(conn: &rusqlite::Connection) -> Result<(), MigrationError> {
    let adapter = SqliteAdapter::new(conn);
    adapter.setup_schema();

    let mut migrator = schemamama::Migrator::new(adapter);
    register!(migrator, CreateActors);
    migrator.up(None)
}
