use anyhow::Result;
use clap::Parser;
use connection::Connection;

#[tokio::main]
async fn main() -> Result<()> {
    let port = Args::parse();
    let addr = format!("0.0.0.0:{}", port.port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
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

#[derive(clap::Parser)]
pub struct Args {
    /// A port
    #[arg(short = 'p', long = "port")]
    pub port: u16,
}
