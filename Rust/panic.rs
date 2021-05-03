//Rust存在panic!宏调用，当执行这个宏的时候程序会打印一个错误信息，展开并清理栈数据，然后直接退出程序


fn main() {
    let v = vec![1, 2, 3];
    v[99]; //此时就会调用panic!宏，当你索引一个不存在的值的 时候，Rust会停止并拒绝执行
/*
    thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:6:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    报错信息提示我们设置RUST_BACKTRACE环境变量，这样可以得到一个backtrace，backtrace是一个执行到目前位置的所有被调用
    函数的列表，阅读backtrace的关键在于从开始读到发现你编写的文件，这里往往是问题的发源地，这一行上面是你的代码调用的代码
    往下面是调用你的代码的代码.
    RUST_BACKTRACE=1 cargo run
    上面的命令就可以获取带有信息的backtrace，但是必须开启debug表示，当不以
*/


}
