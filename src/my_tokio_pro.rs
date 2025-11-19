use {
    std::collections::VecDeque,
    std::future::Future,
    std::pin::Pin,
    std::task::{Context, Poll},
    futures::task,
};

pub struct MyTokio {
    scheduled: mpsc::Receiver<Arc<Task>>,
    sender: mpsc::Sender<Arc<Task>>,
}

pub struct TaskFuture {
    future: Pin<Box<dyn Future<Output=()> + Send>>,
    poll: Poll<()>,
}

pub struct Task {
    // The `Mutex` is to make `Task` implement `Sync`.
    task_future: Mutex<TaskFuture>,
    executor: mpsc::Sender<Arc<Task>>,
}

impl Task {
    fn schedule(self: &Arc<Self>) {
        self.executor
            .send(self.clone())
    }

    fn poll(self: Arc<Self>) {
        
    }
}

impl MyTokio {
    pub fn new() -> MyTokio {
        MyTokio {
            tasks: VecDeque::new(),
        }
    }

    /// Spawn a future onto the mini-tokio instance.
    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.tasks.push_back(Box::pin(future));
    }

    pub fn run(&mut self) {
        let waker = task::noop_waker();
        let mut cx = Context::from_waker(&waker);

        while let Some(mut task) = self.tasks.pop_front() {
            if task.as_mut().poll(&mut cx).is_pending() {
                self.tasks.push_back(task);
            }
        }
    }
}