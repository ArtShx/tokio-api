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

use std::collections::HashMap;
use std::str;
use std::sync::mpsc::SyncSender;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufStream, Interest};
use tokio::net::TcpListener;


pub mod data;
pub mod store;
pub mod description;
pub mod title;
pub mod req;
pub mod resp;

use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketStore, TicketId};
use crate::resp::Response;

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

    let insert_listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let get_listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
    // let patch_listener = TcpListener::bind("127.0.0.1:3002").await.unwrap();
    let insert_addr = insert_listener.local_addr().unwrap();
    let get_addr = get_listener.local_addr().unwrap();
    println!("Listening on http://{}", insert_addr);

    // let handle = tokio::spawn(get_ticket(get_listener));
    let insert_handle = tokio::spawn(insert_ticket(insert_listener));

    // handle.await;
    insert_handle.await;

}

// async fn get_ticket(listener: TcpListener) -> Result<(), anyhow::Error> {
//     // let mut buf = Vec::new();
//     let mut buf = String::new();
//     loop {
//         println!("Ckp1");
//         let (mut socket, _) = listener.accept().await?;
//         print!("New connection");
//         let mut stream = BufStream::new(socket);
//         stream.read_line(&mut buf).await?;
//
//         let mut parts = buf.split_whitespace();
//         // let (mut reader, mut writer) = socket.split();
//         // reader.read(&mut buf).await?;
//         println!("{:?}", parts);
//     }
// }


async fn insert_ticket(listener: TcpListener) -> Result<(), anyhow::Error> {
    let mut buf = vec![0; 1024];

    loop {
        let (mut socket, _) = listener.accept().await?;
        let (mut rd, mut wr) = socket.split();
        let n = rd.read(&mut buf).await?;
        let str_request = str::from_utf8(&buf[..n]).unwrap();
        let request: req::Request = req::parse_request(str_request).unwrap();
        println!("req: {:?}", request);

        let payload = HashMap::from([
            ("Hello".to_owned(), "World".to_owned())
        ]);
        wr.write(
            &Response::make(request.headers, payload)
        ).await?;

    }
}

