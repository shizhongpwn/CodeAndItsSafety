use std::error::Error;
use std::fs;
use std::env; // 这里为程序因为了环境变量

pub struct Config {
    pub query: String,
    pub filename: String,
    pub cast_sensitive: bool
}


impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> { //&'static str 是字符串字面值的类型
        if args.len() < 3 {
            return Err("not enough arguments"); // 这里返回一个Err的值，使得调用函数可以对错误进行处理
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive1 = env::var("CASE_INSENSITIVE").is_err();// var方法返回一个Result，它在环境变量里面被设置的时候返回包含其值的Ok成员，并在环境变量未被设置的时候返回Err成员,可以使用export设置环境变量
        // Result存在一个is_err方法来检查其是否是一个error（也就是环境变量没有被设置的时候），这也就意味着我们需要一个大小写敏感的搜索，如果环境变量被设置为任何值，那么is_err会返回false，那么就会进行大小写不敏感的搜索
        println!("the env is {}", case_sensitive1);
        Ok(Config {query: query, filename: filename, cast_sensitive: case_sensitive1})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{ // 这里对于错误类型，我们使用了trait对象，Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型，不过无需指定具体将会返回的值的类型。这提供了在不同的错误场景可能有不同类型的错误返回值的灵活性。这也就是 dyn，它是 “动态的”（“dynamic”）的缩写
    let contents = fs::read_to_string(config.filename)?; // 这里不再需要添加expect，而是用?替代，？会从函数中返回错误值并让调用者对它进行处理
    let results = if config.cast_sensitive {
        search(&config.query, &contents) // 这里为啥不能跟 ; 呢？  因为这里算是一个 {} 的返回值，下同
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(()) // 函数执行成功的时候户返回一个Ok的值，Ok()中的()是惯用的做法，表明调用run函数只是为了它的副作用：函数并没有返回什么有意义的值
}

//编写失败测试
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
    #[test]
    fn case_insensitive() { //大小写不敏感的测试
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
//测试函数
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {//生命周期参数指定那个参数的生命周期与返回值的生命周期相关联
    //在这个例子中，我们表明返回的 vector 中应该包含引用参数 contents（而不是参数query） slice 的字符串 slice。
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}
pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase(); //转换为小写
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}