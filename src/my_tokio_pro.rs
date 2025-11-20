use {
    std::future::Future,
    std::pin::Pin,
    std::task::{Context, Poll},
    futures::task,
    std::sync::{Arc, Mutex},
    futures::task::ArcWake,
    tokio::sync::mpsc,
 };



/// Task 代表一个可调度的异步任务
/// 包含要执行的 Future 和用于重新调度自己的 executor 引用
pub struct Task {
    /// 使用 Mutex 包装 Future，使 Task 可以在多线程间共享（实现 Sync）
    /// Option 用于在 Future 完成后取出并丢弃它
    future: Mutex<Option<Pin<Box<dyn Future<Output=()> + Send>>>>,
    /// executor 的发送端，用于将自己重新加入调度队列
    executor: mpsc::Sender<Arc<Task>>,
}

impl Task {
    /// 将任务重新调度到 executor 的队列中
    /// 当 Future 返回 Pending 并需要稍后再次轮询时调用
    fn schedule(self: &Arc<Self>) {
        let _ = self.executor
            .clone()
            .try_send(self.clone());  // 使用 try_send 避免阻塞
    }

    /// 轮询任务中的 Future
    /// 创建一个 Waker，当 Future 准备好继续执行时会调用它
    fn poll(self: Arc<Self>) {
        // 从 Task 创建一个 Waker，当 Future 被唤醒时会调用 ArcWake::wake_by_ref
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);

        // 获取 Future 的锁
        let mut future_opt = self.future.try_lock().unwrap();
        
        // 如果 Future 还存在（未完成），则轮询它
        if let Some(future) = future_opt.as_mut() {
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(()) => {
                    // Future 完成，将其从 Option 中移除，释放资源
                    *future_opt = None;
                }
                Poll::Pending => {
                    // Future 未完成，等待下次被唤醒
                    println!("pending...")
                }
            }
        }
    }

    /// 创建一个新任务并将其发送到 executor
    fn spawn<F>(future: F, sender: &mpsc::Sender<Arc<Task>>) 
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Some(Box::pin(future))),
            executor: sender.clone(),
        });

        // 将新任务发送到调度队列
        let _ = sender.clone().try_send(task);
    }   
}

/// 实现 ArcWake trait，使 Task 可以作为 Waker 使用
/// 当 Future 准备好继续执行时，会调用这个方法
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // 将任务重新加入调度队列
        arc_self.schedule();
    }
}

/// MyTokio 是一个简化的异步运行时
/// 它维护一个任务队列，并负责轮询这些任务
pub struct MyTokio {
    /// 接收端，用于接收待调度的任务
    scheduled: mpsc::Receiver<Arc<Task>>,
    /// 发送端，用于将任务加入调度队列
    sender: mpsc::Sender<Arc<Task>>,
}

impl MyTokio {
    /// 创建一个新的 MyTokio 运行时实例
    pub fn new() -> MyTokio {
        // 创建一个容量为 10000 的 channel
        let (sender, scheduled) = mpsc::channel(10000);
        MyTokio {
            sender, 
            scheduled
        }
    }

    /// 在运行时上生成一个新的异步任务
    /// 任务会被加入调度队列，等待执行
    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Task::spawn(future, &self.sender)
    }

    /// 运行事件循环
    /// 不断从队列中取出任务并轮询，直到所有任务完成
    pub fn run(&mut self) { 
        // 使用 blocking_recv() 同步阻塞地获取下一个任务
        while let Some(task) = self.scheduled.blocking_recv() {
            task.poll();
        }
    }
}