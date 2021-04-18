/*
    可变引用有一个很大的限制，在特定作用域中特定数据只能有一个可变引用
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);
    这样会导致报错，但是这样的好处在于Rust在编译的时候就避免了数据竞争，它甚至不会编译存在数据竞争的代码
    let mut s = String::from("hello");
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    let r3 = &mut s; // 大问题
    println!("{}, {}, and {}", r1, r2, r3);
    从上面可以看出rust同样不允许在拥有可变引用的同时存在不可变引用，但是多个不可变引用同时存在是可以的
    但是一个引用的作用域是从声明的地方开始，到最后一次使用为止的，如果在最后一次使用不可变引用在声明可变引用之前，
    那么这样是可以的。
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);

*/


fn main() {
    let mut s1 = String::from("hello");//对于change函数来说，这里的mut也是必须的
    let len = calculate_length(&s1);// 这里&s1是一个引用，他并不own s1,当其离开scope的时候，它指向的值不会被drop
    println!("The si length is {}", len);
    change(&mut s1); //在此将其转换为可变引用
    let mut test1 = String::from("hello");
    {
        let r1 = &mut test1;
    }// r1离开这里的时候就相当于离开了作用域
    let r2 = &mut test1; //无错，因为rust只是不能同时拥有两个可变引用，这并不是同时


}

fn calculate_length(s: &String) -> usize {
    s.len()
}// 这里s离开 scope，但是因为它没有ownship，不发生Drop，总结：当引用作为参数的时候，离开作用域不被drop，因为它没有指向对象的所有权
/*fn change(some_string:  String) { //此时会报错
    some_string.push_str(", world!"); 
}*/
fn change(some_string: &mut String) { //此时我们定义了可变引用
    some_string.push_str(", world");
}


