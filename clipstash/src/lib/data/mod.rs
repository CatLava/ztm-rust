pub mod model;
pub mod query;

use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::str::FromStr;
use sqlx::Sqlite;

// here encapsulating error in our own type
// adaptable for upgrades as well 
#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error)
}

// If db changes, only need to modify Sqlite to change db
// All the database actions
pub type AppDatabase = Database<Sqlite>;
pub type DatabasePool = sqlx::sqlite::SqlitePool;
// being able to role back with transaction
pub type Transaction<'t> = sqlx::Transaction<'t, Sqlite>;
// rows and results that are returned from queries
pub type AppDatabaseRow = sqlx::sqlite::SqliteRow;
pub type AppQueryResult = sqlx::sqlite::SqliteQueryResult;

// Using generic structure for database
pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

impl Database<Sqlite> {
    pub async fn new(connection_str: &str) -> Self {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(connection_str)
            .await;
        // after waiting on connection will match on result
        match pool {
            Ok(pool) => Self(pool),
            Err(e) => {
                eprintln!("{}\n", e);
                eprintln!("If db not created, run: \n $sqlx database setup \n");
                panic!("database connection error");
            }
        }
    }
    // pool used to query the database
    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }
}

#[derive(Clone, Debug, From, Display, Deserialize, Serialize)]
pub struct DbId(Uuid);

impl DbId {
    pub fn new() -> DbId {
        Uuid::new_v4().into()
    }

    // creates UUID of all 000's
    pub fn nil() -> DbId {
        Self(Uuid::nil())
    }
}

impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for DbId {
    type Err = uuid::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(DbId(Uuid::parse_str(id)?))
    }
}