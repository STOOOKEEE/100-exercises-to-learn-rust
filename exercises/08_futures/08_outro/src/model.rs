use serde::{Deserialize, Serialize};

const MAX_TITLE_LEN: usize = 50;
const MAX_DESCRIPTION_LEN: usize = 500;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct TicketId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Ticket {
    pub id: TicketId,
    pub title: String,
    pub description: String,
    pub status: Status,
}

/// Body of a `POST /tickets` request.
#[derive(Debug, Deserialize)]
pub struct TicketDraft {
    pub title: String,
    pub description: String,
}

/// Body of a `PATCH /tickets/{id}` request: every field is optional.
#[derive(Debug, Deserialize)]
pub struct TicketPatch {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<Status>,
}

pub fn validate_title(title: &str) -> Result<(), &'static str> {
    if title.is_empty() {
        Err("The title cannot be empty")
    } else if title.len() > MAX_TITLE_LEN {
        Err("The title cannot be longer than 50 bytes")
    } else {
        Ok(())
    }
}

pub fn validate_description(description: &str) -> Result<(), &'static str> {
    if description.is_empty() {
        Err("The description cannot be empty")
    } else if description.len() > MAX_DESCRIPTION_LEN {
        Err("The description cannot be longer than 500 bytes")
    } else {
        Ok(())
    }
}
