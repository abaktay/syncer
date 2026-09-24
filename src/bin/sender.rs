use dirs::home_dir;
use std::io::stdin;
use sync::transfer::send_file;
use tokio::{fs::create_dir_all, net::TcpStream};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Enter the username (needs MagicDns) or IP address of the recipient:");
    let mut recipient = String::new();

    stdin()
        .read_line(&mut recipient)
        .expect("error reading user input");

    let mut socket = TcpStream::connect(format!("{}:9000", recipient.trim())).await?;

    let sync_dir = home_dir()
        .expect("Couldn't find home directory")
        .join("sync");

    create_dir_all(&sync_dir).await?;

    loop {
        println!("Enter the name of the file");
        let mut file_name = String::new();
        stdin()
            .read_line(&mut file_name)
            .expect("error reading user input");

        let file_path = sync_dir.join(file_name.trim());

        send_file(&mut socket, &file_path).await?;
    }
}
