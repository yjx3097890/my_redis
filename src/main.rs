mod my_tokio;
mod delay;

use std::time::{Duration, Instant};

use my_tokio::MyTokio;
use delay::Delay;


fn main() {
    let mut my_tokio = MyTokio::new();

    my_tokio.spawn(async {
        let when = Instant::now() + Duration::from_millis(10);
        let future = Delay { when };

        let out = future.await;

        println!("out: {}", out); 
    });

    my_tokio.run();
}