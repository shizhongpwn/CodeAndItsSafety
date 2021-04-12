use std::io;
use std::cmp::Ordering;
use std::process::exit;
fn main() {
    let array: [u32;5] = [1, 2, 3, 4, 5];
    println!("please input a number");
    let mut input = String::new();
    io::stdin().
    read_line(&mut input)
    .expect("failed to read");
    let index: usize = input.trim().parse().expect("Not a number");
    match index.cmp(&5) {
        Ordering::Equal => {
            println!("error");
            exit(0);
        },
        Ordering::Greater => {
            println!("error");
            exit(0);
        },
        Ordering::Less => println!("good number")
    };
    println!("The value of element at index {} is {}", index,array[index]);
}
