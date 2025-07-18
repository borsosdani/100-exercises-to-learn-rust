use std::{fmt::Error, sync::mpsc::{Receiver, Sender}};
use crate::{data::TicketDraft, store::TicketStore,store::TicketId,data::Ticket};

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert {
        draft: TicketDraft,
        response_sender: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_sender: Sender<Option<Ticket>>,
    }
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {draft,response_sender}) => {
                let ticket_id = store.add_ticket(draft);
                let _ = response_sender.send(ticket_id);
            }
            Ok(Command::Get {id,response_sender}) => {
                let ticket=store.get(id);
                let _ = response_sender.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break
            },
        }
    }
}
