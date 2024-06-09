use std::{collections::HashMap, hash::Hash};

// use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};


#[derive(Debug, Clone)]
// pub struct Response<S: AsyncRead + Unpin> {
pub struct Response {
    pub status: Status,
    pub headers: HashMap<String, String>,
    pub payload: HashMap<String, String>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Status {
    NotFound,
}

impl Response {
    pub fn make(headers: HashMap<String, String>, payload: HashMap<String, String>) -> Vec<u8> {
        let content = format!("{:?}", payload);
        let content_len = content.len();
        let content = format!("HTTP/1.1 200 OK\n{:?}\nContent-Length: {}\r\n\n{:?}\r\n\r\n", 
            headers, content_len, payload);
        content.clone().into_bytes()
    }

}
