 
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame, Command};
use bytes::Bytes;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6378").await.unwrap();

    println!("Server is running on port 6378");

    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        let db = db.clone();

        tokio::spawn(async move {
            process(socket, db).await;
        
        });
    }
}

async fn process(socket: TcpStream, db: Db) {

    // Connection, provided by `mini-redis`, handles parsing frames from
    // the socket
    let mut connection = Connection::new(socket);
    
    while let Some(frame) = connection.read_frame().await.unwrap() {        

        let response = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                let mut db: MutexGuard<HashMap<String, Bytes>> = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
                 // MutexGuard 离开作用域，自动释放锁
            }
            Command::Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
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