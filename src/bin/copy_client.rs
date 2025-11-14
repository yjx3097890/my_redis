use bytes::buf;
use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};



#[tokio::main]
async fn main() {
    let client = TcpStream::connect("127.0.0.1:6380").await.unwrap();

    let (mut rc, mut wc) = io::split(client);


    tokio::spawn(async move {
        wc.write_all(b"hello\r\n").await.unwrap();
        wc.write_all(b"world\r\n").await.unwrap();

    });

    let mut buf = vec![0; 128];

    loop {
        let n = rc.read(&mut buf).await.unwrap();

        if n==0 {
            break;
        }
    }

}