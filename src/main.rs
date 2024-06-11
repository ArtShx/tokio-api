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

use std::str;
use std::sync::mpsc::SyncSender;
use std::sync::Arc;
use req::Request;
use serde_json::{json, Value};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;


pub mod data;
pub mod store;
pub mod description;
pub mod title;
pub mod req;
pub mod resp;

use crate::data::{Ticket, TicketDraft};
use crate::description::TicketDescription;
use crate::store::{TicketStore, TicketId};
use crate::resp::Response;
use crate::title::TicketTitle;

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

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let addr = listener.local_addr().unwrap();
    println!("Listening on http://{}", addr);
    gateway(listener).await;
}


async fn gateway(listener: TcpListener) -> Result<(), anyhow::Error> {
    // using store as a global variable here
    // todo: implement a simples database
    // maybe use mspc, run a separate thread only for this store and communicate with it this way
    let store = Arc::new(Mutex::new(TicketStore::new()));
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(router(socket, Arc::clone(&store)));
    }
}

async fn router(mut socket: TcpStream, store: Arc<Mutex<TicketStore>>) -> Result<(), anyhow::Error> {
    let mut buf = vec![0; 1024];
    let (mut rd, mut wr) = socket.split();
    let n = rd.read(&mut buf).await?;
    let str_request = str::from_utf8(&buf[..n]).unwrap();
    let request: req::Request = req::parse_request(str_request).unwrap();
    println!("req: {:?}", request);

    let response = match request.path.as_str() {
        "/hello_world" => { hello_world(request, store).await }
        "/insert" => { insert_ticket(request, store).await },
        "/get" => { get_ticket(request, store).await },
        _ => { panic!("Invalid route" )}
    };
    wr.write(
        &Response::make(response.headers, response.payload)
    ).await?;
    Ok(())
}


async fn hello_world(req: Request, store: Arc<Mutex<TicketStore>>) -> Response {
    Response { 
        status: resp::Status::Ok, 
        headers: Response::default_headers(), 
        payload: json!({
            "Hello": "World"
        })
    }
}

async fn insert_ticket(req: Request, store: Arc<Mutex<TicketStore>>) -> Response {
    let title = req.payload["title"].as_str().unwrap();
    let description = req.payload["description"].as_str().unwrap();

    let ticket = TicketDraft {
        title: TicketTitle::try_from(title).unwrap(),
        description: TicketDescription::try_from(description).unwrap()
    };
    let id = store.lock().await.add_ticket(ticket);

    dbg!(id);
    Response { 
        status: resp::Status::Ok, 
        headers: Response::default_headers(), 
        payload: json!({
            "id": id.0
        })
    }
}

async fn get_ticket(req: Request, store: Arc<Mutex<TicketStore>>) -> Response {
    let id = req.payload["id"].as_u64().unwrap();
    let ticket = store.lock().await.get(TicketId(id));
    let mut payload = Value::default();

    if let Some(tkt) = ticket {

        let read = tkt.read().unwrap();

        // todo: parsing to string the TicketTitle and others automatically, without having to get
        // the .0 value
        payload = json!({
            "ticket": {
                "id": read.id.0,
                "title": *read.title.0
            }
        });
    }

    Response { 
        status: resp::Status::Ok, 
        headers: Response::default_headers(), 
        payload
    }
}
