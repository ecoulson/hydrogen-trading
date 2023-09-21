use tokio_postgres::{connect, types::ToSql, Client, NoTls, Row, Transaction};

use crate::schema::errors::{Error, Result};

pub struct DatabaseClient {
    client: Client,
}

pub struct DatabaseConnectionParameters {
    user: String,
    host: String,
    db: String,
}

impl DatabaseConnectionParameters {
    pub fn new(user: &str, host: &str, db: &str) -> DatabaseConnectionParameters {
        DatabaseConnectionParameters {
            user: String::from(user),
            host: String::from(host),
            db: String::from(db),
        }
    }
}

impl DatabaseClient {
    pub async fn open(
        connection_parameters: &DatabaseConnectionParameters,
    ) -> Result<DatabaseClient> {
        let connection_string = format!(
            "postgres://{}@{}/{}",
            connection_parameters.user, connection_parameters.host, connection_parameters.db
        );
        let (client, connection) = connect(&connection_string, NoTls)
            .await
            .map_err(|_| Error::create_not_found_error("Couldn't connect to db"))?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Failed to connect: {e}")
            }
        });

        Ok(DatabaseClient { client })
    }

    pub async fn query(&mut self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>> {
        self.client
            .query(query, params)
            .await
            .map_err(|err| Error::create_unknown_error(&err.to_string()))
    }

    pub async fn transaction(&mut self) -> Result<Transaction> {
        self.client
            .transaction()
            .await
            .map_err(|err| Error::create_unknown_error(&err.to_string()))
    }
}
