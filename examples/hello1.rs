use tokio::io::{self, AsyncWriteExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;

    // 由于缓冲区大小限制，写入操作可能不会写入全部数据的
    let n = file.write(b"some bytesWrote the firstbytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytesWrote the first {} bytes of 'some bytes").await?;

    println!("Wrote the first {} bytes of 'some bytes'.", n);
    Ok(())
}