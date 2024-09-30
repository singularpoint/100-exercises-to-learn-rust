// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

#[derive(thiserror::Error, Debug)]
pub enum TicketCommandError {
    #[error("error sending command")]
    CommunicationError,
    #[error("server does not respond")]
    ServerDown,
}

impl TicketStoreClient {
    pub fn new(sender: SyncSender<Command>) -> Self {
        Self { sender }
    }

    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, TicketCommandError> {
        let (response_sender, response_receiver) = 
            sync_channel(1);
        self.sender.try_send(
            Command::Insert { draft, response_channel: response_sender }
        ).map_err(|_| TicketCommandError::CommunicationError)?;
        Ok(response_receiver.recv().map_err(|_| TicketCommandError::ServerDown)?)
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, TicketCommandError> {
        let (response_sender, response_receiver) =
            sync_channel(1);
        self.sender.try_send(
            Command::Get { id, response_channel: response_sender }
        ).map_err(|_| TicketCommandError::CommunicationError)?;
        Ok(response_receiver.recv().map_err(|_| TicketCommandError::ServerDown)?)
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient::new(sender)
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
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
                response_channel.send(id).unwrap()
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                response_channel.send(ticket.cloned()).unwrap()
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
