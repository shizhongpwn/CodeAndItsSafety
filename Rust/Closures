/*
    闭包是可以保存进变量或者作为参数传递给其他函数的匿名函数。可以在一个地方创建闭包，然后再不同的上下文中执行闭包运算
 */


use std::thread;
use std::time::Duration;
fn main() {

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
    let v1 = vec![1, 2, 3];
    let v2 = v1.iter();
    for i in v2 {
        println!("the i is {}", i);
    }
}
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2)); // 沉睡两秒
    intensity
}
fn generate_workout(intensity: u32, random_number: u32) {
    /*let expensive_result =
        simulated_expensive_calculation(intensity);  // 这用的算法虽然解决了之前的一个if中调用2次相同函数的问题，但是也使得之前else块中的本来不用执行任何函数的情况消除了
                                                     // 闭包的优势在于我们可以再程序的一个位置指定某些代码，并只在程序的某处实际需要结果的时候执行这些代码。
     */
    let expensive_closure = |num: u32| -> u32 { // 定义一个闭包并存储到变量expensive_closure中 ， 闭包的定义以一对竖线（|）开始，在竖线中指定闭包的参数；如果有多于一个参数，可以使用逗号分隔，比如 |param1, param2|。
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num // 如果闭包体只有一行则大括号是可以省略的。大括号之后闭包的结尾，需要用于 let 语句的分号。因为闭包体的最后一行没有分号（正如函数体一样），所以闭包体（num）最后一行的返回值作为调用闭包时的返回值
    };


    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}
//可以创建一个存放闭包和调用闭包结果的结构体。该结构体只会在需要结果时执行闭包，并会缓存结果值，这样余下的代码就不必再负责保存结果并可以复用该值。你可能见过这种模式被称 memoization 或 lazy evaluation （惰性求值）
struct Cacher<T>
    where T: Fn(u32) -> u32 { // Fn系列trait又标准库提供，所有的闭包都实现了trait Fn， FnMut 或者 FnOnce中的一个，这里我们规定了闭包的参数和返回值
    calculation: T,
    value: Option<u32>
}