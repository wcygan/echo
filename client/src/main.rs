use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    tokio::select! {
        _ = tokio::spawn(async { run(args).await }) => {}
        _ = tokio::signal::ctrl_c() => {}
    }

    Ok(())
}

async fn run(args: Args) -> Result<()> {
    let mut stdin = tokio_utils::recv_from_stdin(5);
    let mut connection = connection::Connection::dial(args.address).await?;

    loop {
        tokio::select! {
            incoming = connection.read::<String>() => {
                if let Ok(Some(s)) = incoming {
                    println!("{:?}", s);
                }
            }
            from_keyboard = stdin.recv() => {
                if let Some(msg) = from_keyboard {
                    connection.write::<String>(&msg).await?;
                }
            }
        }
    }
}

#[derive(clap::Parser)]
pub struct Args {
    /// An address
    #[arg(short = 'a', long = "address")]
    pub address: String,
}
