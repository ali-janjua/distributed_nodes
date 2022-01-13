use std::env;
use std::{thread, time};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let nodes = env::var("NODES").unwrap_or("".to_string());
    let split = nodes.split(" ");

    let listener = TcpListener::bind("0.0.0.0:6379").await.unwrap();
    println!("Listening...");
    let listen_task_handle = tokio::spawn(async move {
        loop {
            let (_socket, _) = listener.accept().await.unwrap();
            println!("Accepted");
        }
    });

    for s in split {
        println!("Connecting to peer: {}", s);
        thread::sleep(time::Duration::from_secs(5));
        let _socket = TcpStream::connect(&s).await;
        println!("Connected!");
    }
    listen_task_handle.await.unwrap();
}
