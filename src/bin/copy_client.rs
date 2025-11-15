
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};



#[tokio::main]
async fn main() {
    let mut client = TcpStream::connect("127.0.0.1:6380").await.unwrap();

    // 写入数据
    client.write_all(b"hello\r\n").await.unwrap();
    client.write_all(b"world\r\n").await.unwrap();
    println!("写入完成");

    // 关闭写入端，但保持读取端打开
    client.shutdown().await.unwrap();

    let mut buf = vec![0; 128];

    loop {
        let n = client.read(&mut buf).await.unwrap();

        if n == 0 {
            break;
        }

        println!("收到 {} 字节: {}", n, String::from_utf8_lossy(&buf[..n]));
    }

    println!("连接关闭");
}