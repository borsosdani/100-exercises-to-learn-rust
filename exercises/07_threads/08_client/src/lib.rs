use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

#[derive(Clone)]
// TODO: flesh out the client implementation.
pub struct TicketStoreClient {
    sender : Sender<Command>
}

impl TicketStoreClient {
    // Feel free to panic on all errors, for simplicity.
    pub fn insert(&self, draft: TicketDraft) -> TicketId {
        let (resp_sender, resp_receiver) = std::sync::mpsc::channel();
        let _ = self.sender.send(Command::Insert { draft: draft, response_channel: resp_sender });
        resp_receiver.recv().unwrap()
    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> {
         let (resp_sender, resp_receiver) = std::sync::mpsc::channel();
         self.sender.send(Command::Get { id: id, response_channel: resp_sender }).unwrap();
         resp_receiver.recv().unwrap()
    }
}

pub fn launch() -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    let ticket_store_client = TicketStoreClient{
        sender
    };
    ticket_store_client
}

// No longer public! This becomes an internal detail of the library now.
enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
