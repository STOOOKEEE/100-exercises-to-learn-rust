use crate::model::{validate_description, validate_title, Status, Ticket, TicketDraft, TicketId, TicketPatch};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
pub enum StoreError {
    NotFound,
    Invalid(&'static str),
}

#[derive(Default)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Ticket>,
    counter: u64,
}

impl TicketStore {
    pub fn add(&mut self, draft: TicketDraft) -> Result<Ticket, StoreError> {
        validate_title(&draft.title).map_err(StoreError::Invalid)?;
        validate_description(&draft.description).map_err(StoreError::Invalid)?;

        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: draft.title,
            description: draft.description,
            status: Status::ToDo,
        };
        self.tickets.insert(id, ticket.clone());
        Ok(ticket)
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.get(&id)
    }

    /// Validates every provided field before applying any of them.
    pub fn patch(&mut self, id: TicketId, patch: TicketPatch) -> Result<Ticket, StoreError> {
        let ticket = self.tickets.get_mut(&id).ok_or(StoreError::NotFound)?;
        if let Some(title) = &patch.title {
            validate_title(title).map_err(StoreError::Invalid)?;
        }
        if let Some(description) = &patch.description {
            validate_description(description).map_err(StoreError::Invalid)?;
        }

        if let Some(title) = patch.title {
            ticket.title = title;
        }
        if let Some(description) = patch.description {
            ticket.description = description;
        }
        if let Some(status) = patch.status {
            ticket.status = status;
        }
        Ok(ticket.clone())
    }
}
