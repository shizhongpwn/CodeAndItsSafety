fn main() {
    println!("Hello, world!");
    another_function(1,2);
    let y = {
        let x = 2;
        x + 1
    };
    println!("y is {}", y);
    let z = five();
    println!("z is {}", z);
    let h = plus_one(5);
    println!("h is {}", h);
}
fn another_function(x :u32, y :u32) {
    println!("x is {}",x);
    println!("x + y is {}",y + x);
}
fn five() -> i32 {
    5
}
fn plus_one(x :i32) -> i32 {
    x + 1;
}
