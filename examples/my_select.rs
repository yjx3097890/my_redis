use {
    std::future::Future,
    std::pin::Pin,
    std::task::{Context, Poll},
    tokio::sync::oneshot,
};


struct MySelect {
    rx1: oneshot::Receiver<&'static str>,
    rx2: oneshot::Receiver<&'static str>,
}

impl Future for MySelect {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Poll::Ready(v) = Pin::new(&mut self.rx1).poll(cx) {
            println!("rx1 completed first with {:?}", v);
            return Poll::Ready(());
        }
        if let Poll::Ready(v) = Pin::new(&mut self.rx2).poll(cx) {
            println!("rx2 completed first with {:?}", v);
            return Poll::Ready(());
        }
        Poll::Pending
    }
}

#[tokio::main]
async fn main() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async move {
        tx1.send("one").unwrap();
    });
    tokio::spawn(async move {
        tx2.send("two").unwrap();
    });

    MySelect { rx1, rx2 }.await;
}

