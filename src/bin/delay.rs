use futures::future::poll_fn;
use std::future::Future;
use std::pin::Pin;
use std::task::Poll;
use std::time::{Duration, Instant};
use my_redis::delay::*;

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_millis(10);
    let mut delay = Some(Delay { when, waker: None });

    // 第一步：轮询 delay 并验证它返回 Pending
    let handle = poll_fn(move |cx| {
        let mut delay = delay.take().unwrap();
        let res = Pin::new(&mut delay).poll(cx);
        assert!(res.is_pending());
        println!("第一次轮询返回 Pending");
        
        // 生成一个任务来等待 delay 完成
        let handle = tokio::spawn(async move {
            delay.await;
            println!("Delay 完成！");
        });

        Poll::Ready(handle)
    }).await;

    // 等待 spawn 的任务完成
   handle.await.unwrap();
}

