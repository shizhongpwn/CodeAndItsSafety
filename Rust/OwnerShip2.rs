fn main() {
    let s = String::from("hello");
    takes_ownership(s); //此时s的值被move进函数，那么就在main scope里面不再合法
    //println!("s is {}", s);  this is a error, now
    let x = 5;
    makes_copy(x);// 此时x 传入函数执行的是Copy，因此x在main scope 依然合法，因为复制出了另一个x传入了函数。
    println!("x is {}", x); 
}
fn takes_ownership(something: String){
    println!("takes_ownership is {}",something);
}// 在此处 something 被销毁

fn makes_copy(some_integer: i32) {
    println!("some_integer is {}",some_integer);
}// 但是这里什么也不会发生
