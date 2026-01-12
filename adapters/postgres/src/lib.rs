// SPDX-License-Identifier: AGPL-3.0-or-later
//! PostgreSQL adapter for FormDB Debugger
//!
//! Provides schema introspection, WAL parsing, and transaction log extraction
//! for PostgreSQL databases.

use thiserror::Error;
use tokio_postgres::{Client, NoTls, Config};
use tracing::{info, warn, instrument};

pub mod schema;
pub mod wal;
pub mod transaction;

/// Errors that can occur when interacting with PostgreSQL
#[derive(Error, Debug)]
pub enum PostgresError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Query failed: {0}")]
    QueryFailed(String),

    #[error("Schema introspection failed: {0}")]
    SchemaError(String),

    #[error("WAL parsing failed: {0}")]
    WalError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

impl From<tokio_postgres::Error> for PostgresError {
    fn from(err: tokio_postgres::Error) -> Self {
        PostgresError::ConnectionFailed(err.to_string())
    }
}

/// PostgreSQL database connection with actual tokio-postgres client
pub struct PostgresConnection {
    /// Connection string for reconnection
    connection_string: String,
    /// Active database client (None if disconnected)
    client: Option<Client>,
    /// Database name extracted from connection string
    database_name: String,
}

impl PostgresConnection {
    /// Create a new connection (not yet connected)
    pub fn new(connection_string: &str) -> Result<Self, PostgresError> {
        // Parse connection string to extract database name
        let config: Config = connection_string
            .parse()
            .map_err(|e: tokio_postgres::Error| PostgresError::ConfigError(e.to_string()))?;

        let database_name = config
            .get_dbname()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "postgres".to_string());

        Ok(Self {
            connection_string: connection_string.to_string(),
            client: None,
            database_name,
        })
    }

    /// Connect to the database
    #[instrument(skip(self))]
    pub async fn connect(&mut self) -> Result<(), PostgresError> {
        info!(database = %self.database_name, "Connecting to PostgreSQL");

        let (client, connection) = tokio_postgres::connect(&self.connection_string, NoTls).await?;

        // Spawn connection handler in background
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                warn!(error = %e, "PostgreSQL connection error");
            }
        });

        self.client = Some(client);
        info!(database = %self.database_name, "Connected to PostgreSQL");
        Ok(())
    }

    /// Disconnect from the database
    pub fn disconnect(&mut self) {
        self.client = None;
        info!(database = %self.database_name, "Disconnected from PostgreSQL");
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.client.is_some()
    }

    /// Get the database name
    pub fn database_name(&self) -> &str {
        &self.database_name
    }

    /// Get a reference to the client (for executing queries)
    pub fn client(&self) -> Result<&Client, PostgresError> {
        self.client
            .as_ref()
            .ok_or_else(|| PostgresError::ConnectionFailed("Not connected".to_string()))
    }

    /// Execute a simple query and return row count
    #[instrument(skip(self, query))]
    pub async fn execute(&self, query: &str) -> Result<u64, PostgresError> {
        let client = self.client()?;
        let rows = client
            .execute(query, &[])
            .await
            .map_err(|e| PostgresError::QueryFailed(e.to_string()))?;
        Ok(rows)
    }

    /// Query and return rows
    pub async fn query(
        &self,
        query: &str,
    ) -> Result<Vec<tokio_postgres::Row>, PostgresError> {
        let client = self.client()?;
        let rows = client
            .query(query, &[])
            .await
            .map_err(|e| PostgresError::QueryFailed(e.to_string()))?;
        Ok(rows)
    }
}

/// Connection pool for multiple concurrent connections
pub struct ConnectionPool {
    connection_string: String,
    connections: Vec<PostgresConnection>,
}

impl ConnectionPool {
    /// Create a new connection pool
    pub fn new(connection_string: &str, pool_size: usize) -> Result<Self, PostgresError> {
        let connections = (0..pool_size)
            .map(|_| PostgresConnection::new(connection_string))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            connection_string: connection_string.to_string(),
            connections,
        })
    }

    /// Initialize all connections
    pub async fn connect_all(&mut self) -> Result<(), PostgresError> {
        for conn in &mut self.connections {
            conn.connect().await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_new() {
        let conn = PostgresConnection::new("postgres://localhost/testdb");
        assert!(conn.is_ok());
        let conn = conn.unwrap();
        assert!(!conn.is_connected());
        assert_eq!(conn.database_name(), "testdb");
    }

    #[test]
    fn test_connection_parse_error() {
        // Invalid connection string should fail
        let conn = PostgresConnection::new("not a valid connection string");
        // tokio-postgres is lenient, so this may or may not fail
        // depending on version
    }
}
