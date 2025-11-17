use tokio;
use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let server = TcpListener::bind("127.0.0.1:6380").await.unwrap();


    loop {
        let ( socket, _) = server.accept().await.unwrap();

        let (mut rd, mut wr) = socket.into_split();

        tokio::spawn(async move {

        

            io::copy(&mut rd, &mut wr).await
        });
    }
}    