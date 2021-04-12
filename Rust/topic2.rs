use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    let secert_number = rand::thread_rng().gen_range(1, 100);
    println!("the secert_number is: {}", secert_number);
    loop {
        println!("please input a random number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
        .expect("failed to read line");
        println!("guess {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                num
            },
            Err(_) => continue
        };
        match guess.cmp(&secert_number) {
            Ordering::Equal => {
                println!("You win");
                break;
            },
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small")
        }

    }

}