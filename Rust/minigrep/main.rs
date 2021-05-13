use std::env;
use std::fs;
use std::process;
use std::error::Error;
/*
mod lib;  这样的方式也可以引入lib中的内容
use lib::*;
*/
use minigrep::Config;
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {  // 这里的err算是对new返回的错误进行接收
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) { // if let可以检查run是否返回一个Err的值
        eprintln!("application error: {}", e); // 这里标准库提供了eprintln!宏来答应到标准错误流 ， println!宏会打印到标准输出流
        process::exit(1);
    }

}

/*
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone(); // 许多不同的方式可以处理String数据，而最简单同时不太高校的方式就是调用这些值的clone方法，这会生成Config实例可以拥有的数据的完整拷贝，不过会比存储字符串数据的引用消耗更多的时间和内存，但是
    let filename = args[2].clone();
    Config { query, filename }
}
*/

