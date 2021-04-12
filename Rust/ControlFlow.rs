fn main() {
    let mut counter = 0;
    let mut over = loop {
        counter += 1;
        if counter >= 10 {
            break counter * 2;
        }
    };
    println!("over is {}", over);
    while over != 0 {
        over -= 1;
        println!("now over is {}", over);
    }
    let array = [0, 1, 2, 3, 4, 5];
    let mut index = 0;
    while index < 5 {
        println!("array {} is {}", index, array[index]);
        index += 1;
    }
    for i in array.iter() {
        println!("the value is {}", i)
    }
    for i in (0..10).rev() {
        println!("the i is {}", i);
    }
}