use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Method {
    Get,
    Insert
}

pub fn parse_request(str_request: &str) -> anyhow::Result<Request> {
    let mut parts_raw: Vec<&str> = str_request
        .split("\r")
        .to_owned()
        .collect();

    let parts_prep = parts_raw
        .iter_mut()
        .map(|line| { 
            line
                .trim()
                .replace("\n", "")
                .replace("\t", "")

        })
        .collect::<Vec<String>>();

    let nb_params = parts_prep.len();
    if parts_prep.len() < 6 {
        panic!("Invalid request, {:?} .. {:?}", parts_prep, parts_prep.len());
    }

    let method_and_route: Vec<&str> = parts_prep[0]
        .split(" ")
        .collect::<Vec<&str>>();

    if method_and_route.len() != 3 {
        panic!("Method and route failed: {:?}", method_and_route)
    }

    let method = match method_and_route[0] {
        "POST" => { Method::Insert },
        "GET" => { Method::Get },
        _ => {
            panic!("Method not supported {}", parts_prep[0]);
        }
    };

    let path = method_and_route[1];

    let headers_range = (3 as usize, nb_params-3);
    let mut headers: HashMap<String, String> = HashMap::new();
    for i in headers_range.0 .. headers_range.1 {
        println!("Header> {}", parts_prep[i]);
        let keyval: Vec<&str> = parts_prep[i].split(": ").collect();
        if keyval.len() != 2 {
            panic!("Header invalid");
        }
        headers.insert(
            keyval[0].to_string(),
            keyval[1].to_string());
    }

    Ok(Request {
        method,
        path: String::from(path),
        headers
    })

}
