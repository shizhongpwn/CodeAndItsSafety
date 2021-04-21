/*
    match是一个强大的流控制运算符，它允许我们将一个值与一系列值进行比较，并根据匹配模式执行相应的代码
*/
/*
    match的另一个功能是可以绑定匹配的模式的部分值，这也是如何从枚举类型中提取值的。
*/
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState) //枚举类型内部包含枚举类型
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin { //和if不同的是if的条件表达式返回的必须是bool类型，但是match就有很多选择，可以是任意类型
        Coin::Penny => {
            println!("Lucky penny");
            1//函数返回值
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {//当匹配到Quater的时候，变量state会绑定25美分对应的州
            println!("State quater is from {:?}", state);
            25
        },

    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
fn main() {
    println!("Hello, world!");
    let money = Coin::Quater;
    let over = value_in_cents(money(UsState::Alaska));
    println!("the over is {}", over);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six is {}", six.expect("error"));
    let some_u8_value = 9;
    match some_u8_value {
        1 => println!{"this is one"},
        3 => println!("this is three"),
        5 => println!("this is five"),
        7 => println!("this is seven"),
        _ => (), //这里是通配符，表示除了已经列出的值以外的所有值，()表示unit的值，所以_代表的情况什么也不做。
    }
}
    /*
        使用match匹配Option<T>;
    */
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}
