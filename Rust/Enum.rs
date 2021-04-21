/*
    rust和其他语言不通，其没有空值，因为NULL的引入会带来很多危害，当你像使用非空值那样使用空值的时候，就会引发大量错误，
    但是NULL的存在确实是有用的，为了弥补没有NULL存在的缺点，Rust实现了一个可以编码存在或者不存在这个概念的枚举类型，
    这个枚举类型是Option<T>，其定义于标准库里面：
    enum Option<T> {
        Some(T),
        None
    }
    这是否有用，使用Rust的时候你不需要引入该类型，可以直接使用，甚至可以直接使用枚举内部的成员
*/


fn main() {
    let some_number = Some(5);
    let some_string = Some("Hello world");
    //但是如果你使用None类型的时候，你必须告诉编译器定义变量的类型，因为通过None，Rust无法推断出变量类型
    let absent_number: Option<i32> = None;
    /*
        为什么这比NULL要好？看起来None和NULL拥有相同的意义，因为Option<T>和其中的T不是同一个类型，T可以是任意类型，编译器
        不会允许像一个肯定有效的值那样使用Option<T>，例子：
        let x: i32 = 4;
        let y: Option<i32> = Some(5);
        let sum = x + y;
        上面的代码会报错no implementation for `i32 + Option<i32> ,在Rust中x这种类型的时候，编译器确保它总是一个有效的值，因为担心y可能会是空值，编译器会确保我们在使用
        y之前已经对可能出现的空值情况做了处理。
        这表明在对Option<T>进行运算之前必须将其转换为类型T，这有效的帮我们解决了某个值不为空，但是事实上为空的情况发生。
        通过其expect()方法，我们可以实现上述功能。
        相关文档：https://doc.rust-lang.org/std/option/enum.Option.html
    */
    let x: i32 = 4;
    let y: Option<i32> = Some(5);
    let sum = x + y.expect("error"); //这里进行类型转处理
    println!("sum is {}", sum);



    println!("Hello, world!");
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;
    route(six);
    let home = IpAddr {
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1")
    };
    let home_v6 = IpAddr {
        kind: IpAddrKind::v6,
        address: String::from("::1")
    };
    /*但是，其实对于rust，其标准库定义个一个开箱即用的定义https://doc.rust-lang.org/std/net/enum.IpAddr.html ，
    在官方中，它将成员地址数据嵌入到了两个不同形式的结构体中
    struct Ipv4Addr {
        // --snip--
    }

    struct Ipv6Addr {
        // --snip--
    }

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
    这表明我们可以把任意类型的数据放入到枚举成员中，甚是可以包含另一个枚举类型
    */
    let message = Message::Write(String::from("Hello"));
    message.call();
}
enum IpAddrKind {
    v4,
    v6
}
enum Message {
    Quir,//没有关联任何数据
    Move {x: i32, y: i32}, //这是一种匿名结构体类型
    Write(String), //包含一个单独的String
    ChangeColor(i32, i32, i32) //包含三个i32
}
/*
    枚举和结构体之间的另一个相似之处在于他们都可以实现方法，
*/
impl Message {
    fn call(&self) {
    }
}
fn route(ip_type: IpAddrKind) {

}
//通过枚举我们可以打包出更通用的结构体
struct IpAddr {
    kind: IpAddrKind,
    address: String
}

