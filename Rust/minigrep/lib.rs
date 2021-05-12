use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> { //&'static str 是字符串字面值的类型
        if args.len() < 3 {
            return Err("not enough arguments"); // 这里返回一个Err的值，使得调用函数可以对错误进行处理
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{ // 这里对于错误类型，我们使用了trait对象，Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型，不过无需指定具体将会返回的值的类型。这提供了在不同的错误场景可能有不同类型的错误返回值的灵活性。这也就是 dyn，它是 “动态的”（“dynamic”）的缩写
    let contents = fs::read_to_string(config.filename)?; // 这里不再需要添加expect，而是用?替代，？会从函数中返回错误值并让调用者对它进行处理
    println!("With text:\n{}", contents);
    Ok(()) // 函数执行成功的时候户返回一个Ok的值，Ok()中的()是惯用的做法，表明调用run函数只是为了它的副作用：函数并没有返回什么有意义的值
}