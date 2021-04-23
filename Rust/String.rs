fn main() {
    let test = "hello world";
    let test_string = test.to_string();//这里其实内部情形是这样的：let test_string = String::from("hello world");
    println!("this is {}", test_string);
    //更新字符串
    let mut s = String::from("what");
    let s2 = String::from(" is");
    s.push_str("s2"); //这里采用的是借用的方法，所以push_str执行完毕之后s2不会被销毁，该方法不需要获取参数的所有权
    println!("the s2 is {}", s2);
    let mut s3 = String::from("lo");
    s3.push('l');//push方法被定义为获取一个单独的字符为参数进行执行
    println!("The s3 is {}", s3);
    let s4 = String::from("hello, ");
    let s5 = String::from("world");
    /*
        在rust里面，使用+运算符时调用了add函数，其函数签名类似 fn add(self, s: &str) -> String 
        标准库里面不是那样定义的，而是使用泛型定义的，但是这是add参数为String的时候确实发生的，但是我们
        发现我们的加法中第二个参数是&String类型，这是因为&String可以被强制转化为&str，当add被调用的时候
        Rust采用了一个解引用强制多态的技术，它把&str解引用成了&str[..],
        可以看出+并没有获取str的所有权，所以s5还是可以继续使用的，但是s4就因为所有权的原因再add函数调用之后被销毁了
    */
    let s6 = s4 + &s5;//此时s4被移动了，不能再继续使用了,add函数，获得s4的所有权并附加上从s5中获取的内容
    println!("the s6 is {}", s6);
    //使用format!
    let over = format!("{}-{}-{}", s, s2, s5);//format!区别于println!，其并不输出，而是返回一个String
    println!("the over is {}", over);
    println!("the s is {}", s);
    //有一个值得注意的点就是Rust是不支持字符串索引的
    let tset1 = String::from("the");
    //let h = tset1[1];
    //println!("h is {}", h); //报错`String` cannot be indexed by `{integer}`
    /*
        其实对于rust来说，String是一个Vec<u8>的封装
    */
    
    let len = tset1.len(); //答案是3，这其实是意味着Vec的长度是3个字节，因为每一个UTF编码都占了一个字节
    let len2 = String::from("Здравствуйте").len(); //这个的答案却是24，这是因为每一个Unicode编码的字符都占用了2个字节
    //上述情况就使得Rust在进行字符索引的时候不能确定是两个字节为一个字符还是一个字节为一个字符，所以Rust拒绝编译这类型的代码，
    //从而使得rust在开发之初就杜绝了这些错误。
    


}
