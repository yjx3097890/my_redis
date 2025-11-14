

use tokio::sync::{mpsc, oneshot};
use bytes::Bytes;
use mini_redis::client;

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get{
        key: String,
        resp: Responder<Option<Bytes>>
    },
    Set{
        key: String, 
        value: Bytes,
        resp: Responder<()>,
    },
}



#[tokio::main]
async fn main() {

    let (tx, mut rx) = mpsc::channel::<Command>(32);


    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6378").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get{key, resp} => {
                    let res = client.get(&key).await;
                    let _ = resp.send(res);
                },
                Command::Set{key, value, resp} => {
                    let res = client.set(&key, value).await;
                    let _ = resp.send(res);

                }
              //  _ => unreachable!("不可能发生")
            }
        }
    });

    let tx2 = tx.clone();   

   

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        let cmd = Command::Set { key: "hello2".into(), value: "world2".into(), resp: resp_tx };

        tx2.send(cmd).await.unwrap();

        let res = resp_rx.await.unwrap().unwrap();
        println!("Set res: {:?}", res);
    });
    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get { key: "hello2".to_string(), resp: resp_tx };
        tx.send(cmd).await.unwrap();

        let res = resp_rx.await.unwrap().unwrap();
        println!("get res: {:?}", res);
    });

    t2.await.unwrap();
    t1.await.unwrap();
    manager.await.unwrap();

}


