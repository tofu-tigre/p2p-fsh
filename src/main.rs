use std::error::Error;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;


use std::net::{
    SocketAddr,
};
use tokio::{
    net::TcpListener,
    io::{
        AsyncReadExt,
        AsyncWriteExt,
    }
    };

#[tokio::main]
async fn main() {
    let _ = run_server().await.unwrap();
}

/// Spawns a new thread that deals with user uploads.
/// Then, creates a TCP server that listens for incoming connections.
async fn run_server() -> Result<(), Box<dyn Error>> {
    let files: Arc<RwLock<HashMap<String, Vec<u8>>>> =
        Arc::new(RwLock::new(HashMap::new()));

    loop {
        // TODO: Catch dead tasks here!
        let listener = TcpListener::bind("localhost:8080").await?;
        spawn_client_listener_task(&listener, files.clone()).await?;
    }
}

async fn spawn_client_listener_task(listener: &TcpListener, files_handle: Arc<RwLock<HashMap<String, Vec<u8>>>>) ->
    Result<(), Box<dyn Error>> {
    let (socket, addr) = listener.accept().await?;
    println!("Client {addr} connected to server");
    tokio::spawn(async move {
        let _ = echo(socket, addr, files_handle).await;
    });
    Ok(())
}

async fn echo(
    mut socket: tokio::net::TcpStream,
    addr: SocketAddr,
    _files_handle: Arc<RwLock<HashMap<String, Vec<u8>>>>) -> Result<(), Box<dyn Error>> {
    loop {
        let mut buffer = [0u8; 1024];
        let bytes_read;
        match socket.read(&mut buffer).await {
            Ok(v) => {
                bytes_read = v;
            }
            Err(e) => { return Err(e)? }
        }
        if bytes_read == 0 {
            println!("Client {addr} disconnected");
            break;
        }
        socket.write_all(&buffer[..bytes_read]).await?;
    }
    Ok(())
}