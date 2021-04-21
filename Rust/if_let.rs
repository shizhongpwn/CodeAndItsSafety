/*
    if let存在的意义在于解决match可能带来的十分冗长的情况，通过if let可以只处理某种模式的情况而忽略其它模式，

*/
fn main() {
    let some_u8_value = Some(3);
    match some_u8_value {
        Some(3) => println!("this is three"),//这表明match只关心值为3的情况
        _ => ()
    }
    if let Some(3) = some_u8_value { //这就是if let带来的简便性,但是这同时失去了match强制要求的穷尽性检查
        println!("three");
    }
    //加一个else或许可以弥补失去穷尽性检查的遗憾
    if let Some(4) = some_u8_value {
        println!("four");
    } else {
        println!("found other！！！");
    }


}
