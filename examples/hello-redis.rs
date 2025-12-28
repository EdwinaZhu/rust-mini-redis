use mini_redis::{client, server};

#[tokio::main]
async fn main() {
    // let thread = thread::spawn(|| {
    //     server::run("127.0.0.1:7878", ()).await.unwrap();
    // })
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878").await.unwrap();
    let server_handle = tokio::spawn(async move {
        server::run(listener, async {
            tokio::signal::ctrl_c().await.unwrap();
        }).await.unwrap();
    });
    
    // start the client
    let mut client = match client::connect("127.0.0.1:7878").await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to connect to Redis server: {}", e);
            return;
        }
    };
    let key = "ying";
    client.set(key, "zhu".into()).await.unwrap();
    let result = client.get(key).await.unwrap();
    println!("got value from redis: {:?}", result);

    server_handle.await.unwrap();
}
