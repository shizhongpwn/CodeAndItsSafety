use std::thread;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let vec = vec![1, 2, 3, 4];
    let handle = thread::spawn(move || { // thread::spawn 的返回值类型是 JoinHandle。JoinHandle 是一个拥有所有权的值，当对其调用 join 方法时，它会等待其线程结束。
        for i in 1..10 {// move 关键字会强制闭包获取其使用的值的所有权，而不是任由 Rust 推断它应该借用值。不使用move会报错，因为 Rust 是保守的并只会为线程借用 vec，这意味着主线程理论上可能使新建线程的引用无效。通过告诉 Rust 将 v 的所有权移动到新建线程，我们向 Rust 保证主线程不会再使用 v。
            println!("the number {} in spawned thread!", i);
            thread::sleep(Duration::from_secs(i));
        }
        println!("here is the vector: {:?}", vec);
    });
    for i in 1..5 {
        println!("the number {} in main thread!", i);
        thread::sleep(Duration::from_secs(i));
    }
    handle.join().unwrap(); // 这里阻塞主线程来保证线程执行完毕
}