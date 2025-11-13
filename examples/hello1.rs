use std::sync::{Mutex, Arc};
use tokio;
use std::time::Duration;


#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(0));
    
    tokio::spawn(async move {
        let mut num = data.lock().unwrap();
        *num += 1;
        
        // ❌ 编译错误！
      //  tokio::time::sleep(Duration::from_secs(1)).await;
      
    });
}