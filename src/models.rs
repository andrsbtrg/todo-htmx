use crate::{Error, Result};

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

// REGION: contact types

/// The contact that will be stored and sent to the client
#[derive(Serialize, Clone, Debug)]
pub struct Contact {
    id: u32,
    pub title: String,
}

/// This helps create contacts, thus it needs Deserialize
#[derive(Deserialize)]
pub struct ContactCreate {
    title: String,
}
// ENDREGION: contact types

// REGION: model controller
pub struct ModelController {
    db: SqlitePool,
}

impl ModelController {
    pub async fn new(db: SqlitePool) -> Result<Self> {
        Ok(Self { db })
    }
}
// ENDREGION: model controller
