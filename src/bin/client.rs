use tokio::sync::mpsc;
use bytes::Bytes;
use mini_redis::client;

#[derive(Debug)]
enum Command {
    Get{
        key: String
    },
    Set{
        key: String, 
        value: Bytes
    },
}



#[tokio::main]
async fn main() {

    let (tx, mut rx) = mpsc::channel::<Command>(32);

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6378").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get{key} => {
                    let res = client.get(&key).await.unwrap();
                    println!("GOT = {:?}", res);
                },
                Command::Set{key, value} => {
                    client.set(&key, value).await.unwrap();
                }
                _ => unreachable!("需要实现")
            }
        }
    });

    let tx2 = tx.clone();   

   

    let t1 = tokio::spawn(async move {
        tx2.send(Command::Set { key:  "hello2".to_string(), value: "world2".into()}).await.unwrap();
    });
    let t2 = tokio::spawn(async move {
        tx.send(Command::Get{key: "hello".to_string()}).await.unwrap();
    });
    t2.await.unwrap();

    t1.await.unwrap();
    manager.await.unwrap();

}


