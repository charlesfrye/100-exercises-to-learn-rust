use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender, SyncSender, TrySendError};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, TrySendError<Command>> {
        let (sender, receiver) = std::sync::mpsc::channel();
        match self.sender.try_send(Command::Insert {
            draft,
            response_channel: sender,
        }) {
            Ok(()) => Ok(receiver.recv().unwrap()),
            Err(contents) => Err(contents),
        }
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, TrySendError<Command>> {
        let (sender, receiver) = std::sync::mpsc::channel();
        match self.sender.try_send(Command::Get {
            id,
            response_channel: sender,
        }) {
            Ok(()) => Ok(receiver.recv().unwrap()),
            Err(contents) => Err(contents),
        }
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

pub enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
