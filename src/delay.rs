use std::thread;
use std::time::Instant;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};


pub struct Delay {
    pub when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {

        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            let waker = cx.waker().clone();
            let when = self.when;

            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }
                waker.wake();
            });

            Poll::Pending
        }
        
    }
}

// #[tokio::main]
// async fn main() {
//     let when = Instant::now() + std::time::Duration::from_secs(4);
//     let delay = Delay { when };
//     println!("Hello!");
//     let out = delay.await;
//      println!("world!");
//     println!("{}", out);
// }