use std::error::Error;
use std::io::ErrorKind;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::thread;


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
    let task_type = "upload";
    match task_type {
        "download" => { todo!(); }
        "upload" => { let _ = run_server().await; }
        &_ => { todo!(); }
    };
}

/// Spawns a new thread that deals with user uploads.
/// Then, creates a TCP server that listens for incoming connections.
async fn run_server() -> Result<(), Box<dyn Error>> {
    let files: Arc<RwLock<HashMap<String, Vec<u8>>>> =
        Arc::new(RwLock::new(HashMap::new()));
    
    spawn_upload_task(files.clone()).await?;
    spawn_client_listener_tasks(String::from("localhost::8080"), files.clone()).await?;
    unreachable!()
}

async fn spawn_client_listener_tasks(
    addr: String, 
    files_handle: Arc<RwLock<HashMap<String, Vec<u8>>>>) ->
    Result<(), Box<dyn Error>> {
    
    let listener = TcpListener::bind(addr.as_str()).await?;
    loop {
        let fh = files_handle.clone();
        let (socket, addr) = listener.accept().await?;
        println!("Client {addr} connected to server");
        tokio::spawn(async move {
            let _ = echo(socket, addr, fh).await;
        });
    }
}


async fn spawn_upload_task(files_handle: Arc<RwLock<HashMap<String, Vec<u8>>>>) ->
    Result<(), Box<dyn Error>> {
    tokio::spawn(async move {
        loop {
            let line = String::new();
            if line.as_str() == "exit" {
                let mut rw_guard = files_handle.write().await;
                rw_guard.insert(String::from("Foo"), b"foo".to_vec());
            }
        }
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