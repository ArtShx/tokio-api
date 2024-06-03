// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.

use std::sync::mpsc::SyncSender;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

// use crate::data::{Ticket, TicketDraft};
use crate::data::*;
use crate::store::*;


pub mod data;
pub mod store;
pub mod description;
pub mod title;

// use crate::description::{TicketDescription, TicketTitle};

enum Command {
    Insert {
        draft: TicketDraft,
        response: SyncSender<TicketId>
    },
    Get {
        id: TicketId,
        response: SyncSender<Ticket>
    }
}

#[tokio::main]
async fn main() {
    let store = TicketStore::new();

    let create_listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let get_listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
    // let patch_listener = TcpListener::bind("127.0.0.1:3002").await.unwrap();
    let create_addr = create_listener.local_addr().unwrap();
    let get_addr = create_listener.local_addr().unwrap();
    // loop {
        let handle = tokio::spawn(get_ticket(get_listener));
    // }
    handle.await;

}

async fn get_ticket(listener: TcpListener) -> Result<(), anyhow::Error> {
    println!("Ckp0");
    let mut buf = Vec::new();
    loop {
        println!("Ckp1");
        let (mut socket, _) = listener.accept().await?;
        let (mut reader, mut writer) = socket.split();
        reader.read(&mut buf).await?;
        println!("{:?}", buf);
    }
}



