use std::collections::HashMap;

use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use http_types::headers::{HeaderName, HeaderValue, HeaderValues};
use http_types::{Headers, Request, Response, Result, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[async_std::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", 8080)).await?;
    println!("Listening on {}", listener.local_addr()?);
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        task::spawn(async {
            if let Err(err) = accept(stream).await {
                eprintln!("{}", err);
            }
        });
    }
    Ok(())
}
type Query = HashMap<String, String>;

#[derive(Serialize, Deserialize)]
struct Data {
    method: String,
    url: String,
    headers: HashMap<String, Vec<String>>,
    args: Query,
    body: String,
}

async fn accept(stream: TcpStream) -> Result<()> {
    println!("starting new connection from {}", stream.peer_addr()?);
    async_h1::accept(stream.clone(), |mut _req| async move {
        println!("url {}", _req.url());
        println!("method {}", _req.method());
        let names = _req.header_names();
        let mut headers: HashMap<String, Vec<String>> = HashMap::new();
        for k in names {
            let mut values: Vec<String> = Vec::new();
            let vs = _req.header(k).unwrap();
            for v in vs {
                values.push(v.to_string())
            }
            headers.insert(k.to_string(), values);
        }
        let args: Query = _req.query()?;
        println!("query {:?}", args);
        println!("json {}", serde_json::to_string(&headers).unwrap());
        let mut response = Response::new(StatusCode::Ok);
        response.insert_header("Content-Type", "text/plain");
        let mut body: String = "".to_string();
        match _req.header(http_types::headers::CONTENT_TYPE) {
            Some(headers) => {
                let contentType = headers.iter().last().unwrap();
                let application_json = "application/json";
                if contentType == application_json {
                    response.insert_header(http_types::headers::CONTENT_TYPE, application_json);
                }
                body = _req.body_string().await?;
                println!("{}", body);
            }
            None => {}
        };
        let data = Data {
            method: _req.method().to_string(),
            url: _req.url().to_string(),
            headers,
            args,
            body,
        };
        let text = serde_json::to_string(&data)?;
        response.set_body(text);
        Ok(response)
    })
    .await?;
    Ok(())
}
