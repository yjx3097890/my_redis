use std::thread;
use std::time::Instant;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::sync::{Arc, Mutex};


pub struct Delay {
    pub when: Instant,
    pub waker: Option<Arc<Mutex<Waker>>>,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {

        if Instant::now() >= self.when {
            println!("Hello world");
            return Poll::Ready("done")
        } 
        
        if let Some(waker) = &self.waker {
            let mut waker = waker.lock().unwrap();
            // 检查 waker 是否需要更新
            if !waker.will_wake(cx.waker()) {
                println!("更新waker");
                *waker = cx.waker().clone();
            } 
            
        } else {
            // 第一次轮询，创建 waker 并启动线程
            let waker = Arc::new(Mutex::new(cx.waker().clone()));
            self.waker = Some(waker.clone());
            let when = self.when;

            thread::spawn(move || {
                let now = Instant::now();

                println!("生成线程");

                if now < when {
                    thread::sleep(when - now);
                }
               
                println!("线程结束");
                waker.lock().unwrap().wake_by_ref(); 
            });
        }

        Poll::Pending
        
        
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