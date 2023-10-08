use std::env;
use server::{
    run_server,
    ServerMessage
};

mod client;
mod server;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let rv = match args[1].as_str() {
        "server" => {
            run_server().await
        }
        "client" => {
            match args[2].as_str() {
                "ping" => client::run_client(ServerMessage::Ping).await,
                "pong" => client::run_client(ServerMessage::Pong).await,
                _ => panic!(),
            }
        }
        _ => panic!(),
    };
    match rv {
        Ok(()) => (),
        Err(e) => println!("Error: {e}"),
    };
}