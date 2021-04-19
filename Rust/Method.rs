fn main() {
    println!("Hello, world!");

    let rect1 = Rectangle {
        width: 123,
        height: 123
    };
    println!("the over is {}", rect1.area());
    let test_mothd = Rectangle::get_self(32);
    println!("the test_mothd is {:#?}", test_mothd);
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
impl Rectangle {//这里实现的是结构体Rectangle的方法，其中定义了area方法，self指的是结构体自身，但是每个结构体可以同时拥有多个impl方法
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn get_self(size: u32) -> Rectangle {//关联函数
        Rectangle {
            width: size,
            height: size
        }
    }
}
//关联函数
/*
    在impl中，可以使用不以self为参数的函数，这个被称为关联函数，它们不作用于一个结构体的实例，比如String::from()
    关联函数经常被用于返回一个结构体实例的构造函数
*/