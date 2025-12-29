
use tokio::sync::mpsc::channel;
use mini_redis::{Frame, cmd::Get, client};
use bytes::Bytes;
enum Command {
    Get(String),
    Set(String, Bytes),
}
#[tokio::main]
async fn main() {
    // build a channel
    // clients send commands
    // channel send commands to the server
    let mut client = client::connect("127.0.0.1:7878").await.unwrap();
    let (tx, mut rx) = channel::<Command>(32);
    let tx1 = tx.clone();
    tokio::spawn(async move {
        tx.send(Command::Get("ying".to_string())).await.unwrap();
    });
    tokio::spawn(async move {
        tx1.send(Command::Set("ying".to_string(), Bytes::from("ying"))).await.unwrap();
    });

    while let Some(cmd) = rx.recv().await {
        match cmd {
            Command::Get(key) => {
                if let Some(val) = client.get(&key).await.unwrap() {
                    println!("got value from key {}: {:?}", key, val);
                } else {
                    println!("key {} not found", key);
                }
            }
            Command::Set(key, val) => {
                // todo: Error handling
                client.set(&key, val).await.unwrap();
                println!("set key {} done", key);
            }
        }
    }
}