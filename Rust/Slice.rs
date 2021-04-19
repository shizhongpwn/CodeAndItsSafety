/*
    slice允许你引用集合中的一段连续的元素列表，而不用引用整个集合。
    字符串slice是String中一部分值的引用
    但是对于数组slice同样有效
*/

fn main() {
    let str = String::from("test one time");
    let test_s = "hello world"; //此时的test_s是slice值 &str, 这是一个不可变引用
    let index = first_word(&str);
    println!("index is {}", index);
    //数组部分
    let array = [1, 2, 3, 4, 5];
    let slice = &array[1..3];
    println!("slice is {}", slice[1]);
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();  // as_bytes() 返回字节数组
    for (i, &item) in bytes.iter().enumerate() {// enumerate()包装了iter()的结果，将元素作为元组返回
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
