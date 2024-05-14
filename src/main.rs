use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::prelude::*;
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

mod gen;
mod logger;

#[derive(Serialize, Deserialize)]
struct Request {
    client_ip: String,
    client_port: String,
    path: String,
    host: String,
    user_agent: String,
}

fn parse(ip: String, port: String, incoming: [u8; 1024]) -> Request {
    let mut host = "unknown".to_string();
    let mut path = "unknown".to_string();
    let mut user_agent = "unknown".to_string();

    let _: Vec<()> = incoming
        .split(|&b| b == b'\n')
        .map(|line| line.strip_suffix(b"\r").unwrap_or(line))
        .map(|line| {
            if line.starts_with(b"GET ") || line.starts_with(b"POST ") || line.starts_with(b"PUT ")
            {
                path = String::from_utf8(
                    line.split(|&b| b == b' ')
                        .nth(1)
                        .map(|it| it.to_vec())
                        .unwrap_or(b"unable to find path".to_vec()),
                )
                .unwrap_or("unable to decode path".to_string())
            } else if line.starts_with(b"User-Agent: ") {
                user_agent = String::from_utf8(line[12..].to_vec())
                    .unwrap_or("unable to decode agent".to_string())
            } else if line.starts_with(b"Host: ") {
                host = String::from_utf8(line[6..].to_vec())
                    .unwrap_or("unable to decode host".to_string())
            }
        })
        .collect();

    return Request {
        client_ip: ip,
        client_port: port,
        host,
        path,
        user_agent,
    };
}

async fn handle_connection(mut stream: TcpStream, logger: Sender<String>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let (ip, port) = stream
        .peer_addr()
        .map(|peer| (format!("{}", peer.ip()), format!("{}", peer.port())))
        .unwrap_or((
            String::from("unable to get peer ip"),
            String::from("unable to get peer port"),
        ));
    let request = parse(ip, port, buffer);

    let status_line = if request.path == "/" {
        "HTTP/1.1 200 OK\r\n\r\n"
    } else {
        "HTTP/1.1 404 NOT FOUND\r\n\r\n"
    };
    let content = gen::gen(&request.host);
    let response = format!("{status_line}{content}");

    // Logging
    let _ = logger.send(
        serde_json::to_string(&request)
            .unwrap_or(String::from("{\"message\": \"error serializing request\"}")),
    );

    // Panic if the connection is not behaving as expected
    // This should be wrapped into a restart logic outside
    let _ = stream.write(response.as_bytes()).await.unwrap();
    let _ = stream.flush().await.unwrap();
}

#[async_std::main]
async fn main() {
    let listener_ip = env::var("SCANME_IP").unwrap_or("127.0.0.1".to_string());
    let listener_port = env::var("SCANME_PORT").unwrap_or("7878".to_string());
    let listener_interface = format!("{}:{}", listener_ip, listener_port);
    let listener = TcpListener::bind(listener_interface).await.unwrap();

    // Set up logging
    let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();
    thread::spawn(move || logger::start(receiver));

    // Start server
    listener
        .incoming()
        .for_each_concurrent(None, |tcpstream| {
            let value = sender.clone();
            async move {
                let tcpstream = tcpstream.unwrap();
                handle_connection(tcpstream, value).await;
            }
        })
        .await;
}
