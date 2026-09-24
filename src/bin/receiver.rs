use dirs::home_dir;
use sync::transfer::*;
use tokio::fs::create_dir_all;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9000").await?;

    // sync directory
    let sync_dir = home_dir()
        .expect("Couldn't find home directory")
        .join("sync");
    create_dir_all(&sync_dir).await?;

    loop {
        let (mut stream, addr) = listener.accept().await?;
        println!("Connection from {}", addr);
        let sync_dir = sync_dir.clone();
        tokio::spawn(async move {
            loop {
                match read_file(&mut stream, sync_dir.clone()).await {
                    Ok(()) => {
                        println!("File received successfully.");
                    }
                    Err(e) => {
                        eprintln!("Error: {e}");
                        break;
                    }
                }
            }
        });
    }
}
