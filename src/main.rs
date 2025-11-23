mod my_tokio;
mod my_tokio_pro;
mod delay;

use std::{time::{Duration, Instant}};

use my_tokio_pro::MyTokio;
use delay::Delay;


fn main() {
    let mut my_tokio = MyTokio::new();

    my_tokio.spawn(async {
        let when = Instant::now() + Duration::from_millis(1000);
        let future = Delay { when, waker: None };

        let out = future.await;

        println!("out: {}", out); 
    
    });

    my_tokio.run();
}