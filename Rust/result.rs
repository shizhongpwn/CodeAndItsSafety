use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};
fn main() {
    let f= File::open("/Users/clock/RustProjects/rust-learning/result/target/debug/hello.txt");
    //open函数的返回值类型是Result<T，E>,这里的泛型参数T放入了成功值的类型std::fs::File,它是一个文件句柄,E被用在失败
    //值上面的时候，其类型是std::io::Error
    //使用Result枚举
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {//Err返回的成员中的值类型io::Error，它是一个标准库中的结构体，其包含一个返回ErrorKind值的kind方法来进行调用
            ErrorKind::NotFound => match File::create("hello.txt") {//io::ErrorKind是一个标准库的枚举，它的成员对应io操作可能导致不同类型的错误,这里感兴趣的是Notfound成员,如果是Notfound成员那么就通过File::create创建文件。
                Ok(fc) => fc, //文件创建成功
                Err(e) => panic!("problem creating the file: {:?}", e),//文件创建失败
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        }
    };
    //值得注意的是和Option一样Result枚举和其成员也被导入到了prelude中，所以就不需要在match分支中指定Result::
    let f2 = File::open("hello.txt").unwrap_or_else(|error| {//这里使用了更好用的unwrap_or_else
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file {:?}", error);
            })
        } else {
            panic!("Problem opening the file!")
        }
    });
        /*失败时panic的简写：unwrap 和 expect*/
    let f3 = File::open("hello.txt").unwrap();
    let f4 = File::open("hello.txt").expect("the file open error");//通过expect可以定制化panic信息
    let mut s = String::new();
    //当出现错误的时候也可以选择错误传播使得调用者可以知道该如何处理这个错误
}

fn read_username_from_file() -> Result<String, io::Error> {
    /*let mut f = File::open("hello.txt")?;//通过?可以把错误传播给调用者
    let mut s = String::new();
    f.read_to_string(&mut s)?;*/
    //简单的写法如下：
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
    //?运算符可被用于返回值是Result的函数，因为他定义了和match相同的处理方式，所以函数的返回值必须是Result才能和match的处理结果相同。

}
