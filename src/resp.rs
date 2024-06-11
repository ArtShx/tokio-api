use std::{collections::HashMap, hash::Hash};

use serde_json::Value;

// use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};


#[derive(Debug, Clone)]
// pub struct Response<S: AsyncRead + Unpin> {
pub struct Response {
    pub status: Status,
    pub headers: HashMap<String, String>,
    pub payload: Value,
}

impl Response {
    pub fn default_headers() -> HashMap<String, String> {
        let mut hm = HashMap::new();
        hm.insert("Content-type".to_string(), "application/json".to_string());
        hm
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Status {
    NotFound,
    Ok
}

impl Response {
    pub fn make(headers: HashMap<String, String>, payload: Value) -> Vec<u8> {
        let content = format!("{:?}", payload);
        let content_len = content.len();
        let content = format!("HTTP/1.1 200 OK\n{:?}\nContent-Length: {}\r\n\n{:?}\r\n\r\n", 
            headers, content_len, payload);
        content.clone().into_bytes()
    }

}
