use std::env;
use std::{thread, time};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let nodes = env::var("NODES").unwrap_or("".to_string());
    let bind_addr = env::var("BIND").unwrap_or("0.0.0.0:6379".to_string());
    let listen_task_handle = tokio::spawn(async move {
        let listener = TcpListener::bind(&bind_addr).await.unwrap();
        println!("Listening to {}...", bind_addr);
        loop {
            let (_socket, _) = listener.accept().await.unwrap();
            println!("Accepted");
        }
    });

    let client_handle = tokio::spawn(async move {
        let split = nodes.split(",");
        for s in split {
            println!("Connecting to peer: {}", s);
            let _socket = TcpStream::connect(s).await.unwrap();
            println!("Connected!");
        }
    });

    listen_task_handle.await.unwrap();
    thread::sleep(time::Duration::from_secs(5));
    client_handle.await.unwrap();
}
