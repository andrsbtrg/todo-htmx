/*
Model layer
*/

use crate::{Error, Result};

use serde::{Deserialize, Serialize};

// REGION: ticket types

/// The ticket that will be stored and sent to the client
#[derive(Serialize, Clone, Debug)]
pub struct Ticket {
    id: u32,
    pub title: String,
}

/// This helps create tickets, thus it needs Deserialize
#[derive(Deserialize)]
pub struct TicketCreate {
    title: String,
}
// ENDREGION: ticket types

// REGION: model controller

// ENDREGION: model controller
