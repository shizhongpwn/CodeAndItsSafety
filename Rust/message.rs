use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let (tx, rx) = mpsc::channel();//mpsc::channel 函数创建一个新的通道
    let tx1 = mpsc::Sender::clone(&tx);// mpsc本来就是多个生产者单个消费者的意思，我们可以通过克隆通道的发送端来做到这一点。

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();  // send函数会转移所有权并移动这个值归接受者所有
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
       let vals = vec![
            String::from("more"),
           String::from("message"),
           String::from("for"),
           String::from("you")
       ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });


    //let received = rx.recv().unwrap();//这个方法会阻塞主线程执行直到从通道中接收一个值
    for received in rx { //这里不再显示的调用recv函数，而是将rx作为一个迭代器。当通道关闭的时候迭代结束
        println!("Got: {}", received);
    }



}
