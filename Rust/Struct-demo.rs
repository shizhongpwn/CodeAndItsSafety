/*
    对于println!宏，使用{}其能处理很多格式，但是无法直接输出一个结构体，因此我们需要使用{:?}, :? 指示符
    告诉println!我们想要使用Debug模式进行输出，但是对于结构体也有他特殊的要求，即必须为结构体显示的声明
    这个功能
*/
#[derive(Debug)]
struct Test1 {
    test: String,
    test2: u64
}


fn main() {
    let test1 = Rectangle {
        height: 15,
        width: 123
    };
    println!("the over is {}", area(&test1));    //通过结构体我们能很快的明白参数的含义，同时使得，餐宿的
    println!("the height is {}", test1.height); //定义函数的时候记得使用借用，函数结束的时候回把参数drop，但是应为引用没有所有权，所以不会被drop，这里才不会报错
    //通过派生trait增加使用功能
    let test2 = Test1 {
        test: String::from("today"),
        test2: 32
    };
    println!("the test2 is {:?}", test2); //使用:? 标识符采用debug模式
    println!("the test2 is {:#?}", test2); //使用:#? 标识符采用输出的更像结构体
}
struct Rectangle {
    height: u64,
    width: u64
}
fn area(arg1: &Rectangle) -> u64 {
    arg1.height * arg1.width
}
