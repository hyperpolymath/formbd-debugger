// SPDX-License-Identifier: AGPL-3.0-or-later
//! Native FormDB adapter for FormDB Debugger
//!
//! Provides direct access to FormDB's append-only journal,
//! provenance tracking, and Merkle-verified snapshots.

use thiserror::Error;

pub mod journal;
pub mod snapshot;
pub mod provenance;

/// Errors that can occur when interacting with FormDB
#[derive(Error, Debug)]
pub enum FormDBError {
    #[error("Journal read failed: {0}")]
    JournalError(String),

    #[error("Snapshot invalid: {0}")]
    SnapshotError(String),

    #[error("Merkle verification failed: {0}")]
    MerkleError(String),

    #[error("Provenance missing: {0}")]
    ProvenanceError(String),
}

/// FormDB database connection
pub struct FormDBConnection {
    path: std::path::PathBuf,
    opened: bool,
}

impl FormDBConnection {
    /// Open a FormDB database at the given path
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, FormDBError> {
        Ok(Self {
            path: path.as_ref().to_path_buf(),
            opened: true,
        })
    }

    /// Get the journal path
    pub fn journal_path(&self) -> std::path::PathBuf {
        self.path.join("journal")
    }

    /// Check if opened
    pub fn is_opened(&self) -> bool {
        self.opened
    }
}
