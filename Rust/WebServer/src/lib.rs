use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
pub struct ThreadPool {
    works: Vec<Worker>,
    sender: mpsc::Sender<Message>
}
struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate
}

impl Worker {
    fn new(id: usize , receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();//通过lock可以获取互斥锁，recv()从通道中接受Job。最后调用 recv 会阻塞当前线程，所以如果还没有任务，其会等待直到有可用的任务。Mutex<T> 确保一次只有一个 Worker 线程尝试请求任务。
                println!("Worker {} got a job; executing.", id);
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    },
                }
            }
        });
        Worker {
            id: id,
            handle: Some(thread),
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
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.works {
                self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers."); // 现在遍历了 worker 两次，一次向每个 worker 发送一个 Terminate 消息，一个调用每个 worker 线程上的 join。如果尝试在同一循环中发送消息并立即 join 线程，则无法保证当前迭代的 worker 是从通道收到终止消息的 worker。
        for worker in &mut self.works {
            println!("shutting down worker {}", worker.id);
            if let Some(thread) = worker.handle.take() {
                thread.join().unwrap();
            }
        }
    }
}
