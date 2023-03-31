use anyhow::Result;
use connection::Connection;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7272").await?;
    let addr = listener.local_addr()?;
    println!("Listening on: {}", addr);

    loop {
        let (socket, addr) = listener.accept().await?;
        tokio::spawn(async move {
            println!("Peer connected from {}", addr);
            let mut conn = Connection::new(socket);

            loop {
                let s = conn.read::<String>().await.unwrap().unwrap();
                println!("{}: {:?}", addr, s);
                conn.write::<String>(&s).await.unwrap();
            }
        });
    }
}
