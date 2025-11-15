use tokio;
use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let server = TcpListener::bind("127.0.0.1:6380").await.unwrap();

    loop {
        let (mut socket, _) = server.accept().await.unwrap();
    

        tokio::spawn(async move {
            let (mut rd, mut wr) = socket.split();

            io::copy(&mut rd, &mut wr).await
        });
    }
}    