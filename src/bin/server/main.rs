mod sharded_db;

 use std::sync::{Arc};

use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame, Command};
use bytes::Bytes;

use sharded_db::ShardedDb;

type ShardDb = Arc<ShardedDb>;



#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6378").await.unwrap();

    println!("Server is running on port 6378");

    let db: ShardDb = Arc::new(ShardedDb::new(8));

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        let db = db.clone();

        tokio::spawn(async move {
            process(socket, db).await;
        
        });
    }
}

async fn process(socket: TcpStream, db: ShardDb) {

    // Connection, provided by `mini-redis`, handles parsing frames from
    // the socket
    let mut connection = Connection::new(socket);
    
    while let Some(frame) = connection.read_frame().await.unwrap() {        

        let response = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                let mut map_db = db.get_shard(cmd.key()).lock().unwrap();
                map_db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
                 // MutexGuard 离开作用域，自动释放锁
            }
            Command::Get(cmd) => {
                let map_db = db.get_shard(cmd.key()).lock().unwrap();
                if let Some(value) = map_db.get(cmd.key()) {
                    Frame::Bulk(Bytes::from(value.clone()))
                } else {
                    Frame::Null
                }
            }
            _ => unimplemented!(),
        };

        connection.write_frame(&response).await.unwrap();
    }

}