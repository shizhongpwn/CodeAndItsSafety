use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
pub struct ThreadPool {
    works: Vec<Worker>,
    sender: mpsc::Sender<Job>
}
struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>
}
type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    fn new(id: usize , receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();//通过lock可以获取互斥锁，recv()从通道中接受Job。最后调用 recv 会阻塞当前线程，所以如果还没有任务，其会等待直到有可用的任务。Mutex<T> 确保一次只有一个 Worker 线程尝试请求任务。
                println!("Worker {} got a job; executing.", id);
                job();
            }
        });
        Worker {
            id: id,
            handle: thread,
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel(); // 这里新建了一个通道，并让线程池在接收端等待
        let receiver = Arc::new(Mutex::new(receiver)); //Arc 使得多个 worker 拥有接收端，而 Mutex 则确保一次只有一个 worker 能从接收端得到任务。
        let mut workers = Vec::with_capacity(size); // 创建一个vector来存放线程
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));//将通道的接收端放入一个 Arc 和一个 Mutex 中。对于每一个新 worker，克隆 Arc 来增加引用计数，如此这些 worker 就可以共享接收端的所有权了。
        }
        ThreadPool {
            works: workers,
            sender
        }
    }

    pub fn execute<F>(&self, f:F)
        where F: FnOnce() + Send + 'static{//FnOnce trait 仍然需要之后的 ()，因为这里的 FnOnce 代表一个没有参数也没有返回值的闭包。正如函数的定义，返回值类型可以从签名中省略，不过即便没有参数也需要括号。
        let job = Box::new(f); // 为存放每一个闭包的 Box 创建一个 Job 类型别名，接着在通道中发出任务
        self.sender.send(job).unwrap();
    }
}