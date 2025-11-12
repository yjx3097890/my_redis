use mini_redis::client;
use bytes::Bytes;

#[tokio::main]
async fn main() {

  let mut client = client::connect("127.0.0.1:6378").await.unwrap();  

  let res = client.set("key", Bytes::from("value")).await.unwrap();
  println!("res: {:?}", res);

  let value = client.get("key").await.unwrap().unwrap();

  println!("value: {:?}", value);
} 