use anyhow::Result;
use connection::Connection;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7272").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut conn = Connection::new(socket);

            loop {
                let s = conn.read::<String>().await.unwrap().unwrap();
                println!("{:?}", s);
                conn.write::<String>(&s).await.unwrap();
            }
        });
    }
}
