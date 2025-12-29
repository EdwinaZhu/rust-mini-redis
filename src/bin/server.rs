// server part: 
// 1. receive connections from clients and spawn a thread for each connection
// 2. read commands from clients, process them, and send back the responsed
// data structure:
// use a hash map to store key-value pairs in memory, making the hash map shared across multiple threads
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Command};
use mini_redis::Result;
use bytes::Bytes;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub type Db<K, V> = Arc<Mutex<HashMap<K, V>>>;

#[tokio::main]
async fn main() -> crate::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    let db: Db<String, Bytes> = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (socket, _) = listener.accept().await?;
        let db= db.clone();
        tokio::spawn(async move {
            process(socket, db).await.expect("Fail to build socket connection");
        });
    }
    Ok(())
}

async fn process(socket: TcpStream, db: Db<String, Bytes>) -> Result<()> {
    let mut conn = Connection::new(socket);
    while let Some(frame) = conn.read_frame().await? {
        // process the frame
        println!("GOT: {:?}", frame);
        let cmd = Command::from_frame(frame)?;
        println!("CMD: {:?}", cmd);
        match cmd {
            Command::Get(cmd) => {
                // handle Get cmd
                let value = db.lock().unwrap().get(cmd.key()).cloned();
                let response = match value {
                    Some(v) => mini_redis::Frame::Bulk(v),
                    None => mini_redis::Frame::Null
                };
                conn.write_frame(&response).await?;
            }
            Command::Set(cmd) => {
                // handle Set cmd
                db.lock().unwrap().insert(cmd.key().to_string(), cmd.value().clone());
                let response = mini_redis::Frame::Simple("OK".to_string());
                conn.write_frame(&response).await?;
            }
            _ => {
                unimplemented!();
            }
        }
    }
    Ok(())
}