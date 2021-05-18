use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::os::macos::raw::stat;
use std::thread;
use std::time::Duration;
use WebServer::ThreadPool;
/*
    TcpListener用于监听TCP连接
 */
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // bind返回Result<T, E>，这表明可能会绑定失败
    let pool= ThreadPool::new(4);
    for stream in listener.incoming() { // incoming方法返回一个迭代器,提供TcpStream类型的流，这个for循环会以此处理每个连接供我们处理
        let stream = stream.unwrap();
        pool.execute(|| {
            hand_connection(stream);
        });
        println!("Connection established");   // 当程序监听到传入的TCP流的时候，它会打印该信息
    }
}

fn hand_connection(mut stream: TcpStream) { // 参数是可变的，这是因为TcpStream实例在内部记录了所返回的数据，同时他可能读取了多余我们请求的数据并保存他们以备下一次请求数据，因此它需要是mut的，因为内部状态会改变
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request:\n{}", String::from_utf8_lossy(&buffer[..])); //函数获取一个 &[u8] 并产生一个 String。函数名的 “lossy” 部分来源于当其遇到无效的 UTF-8 序列时的行为：它使用 �，U+FFFD REPLACEMENT CHARACTER，来代替无效序列。你可能会在缓冲区的剩余部分看到这些替代字符，因为他们没有被请求数据填满。
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5)); // 休眠5秒
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{} {}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

