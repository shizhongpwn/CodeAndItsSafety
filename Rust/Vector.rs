fn main() {
    let mut v1: Vec<i32> = Vec::new(); //创建一个空的vector保存i32类型的数据,因为没有插入值，所以要使用<>注解
    v1.push(1);
    {
        let mut v2: Vec<i32> = Vec::new();
        v2.push(2);
    } //vector离开作用域也会被释放
    let v2 = vec![1,2,3,4,5,6,7,8,9];
    println!("v2.get(2) is {}", v2.get(2).expect("error"));
    match v2.get(2) {//使用get方法返回的是一个Option<&T>类型，相比于[]的索引，这样的程序更加稳定，因为[]索引一旦越界就一定导致程序崩溃，但是get会返回None
        Some(3) => println!("this is {}", v2.get(2).expect("error")),
        None => println!("this is none"),
        _ => () //因为使用的Some(3)，所以要把所有的i32类型都考虑到
    }
    let third: &i32 = &v2[2]; //同样，当出现引用的时候，vector不能再增加值，因为所有权和借用规则
    //v2.push(12); //这会报错
    println!("The third element is {}", third);

    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),//third是一个固定的值
        None => println!("There is no third element."),
    }
    for i in v2 {
        println!("i is {}", i);
    }
    let mut v3 = vec![1,2,3,4,5];
    for i in &mut v3 {
        *i += 50;
        println!("i is {}", *i); //这里加不加*都可以，就很神奇，可能有指针机制有关
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(4.5),
        SpreadsheetCell::Text(String::from("hello world"))
    ];
    for i in row { 
        println!("The row is {:?}", i);
    }
}
    //我们可以使用枚举类型来使得在vector中存储不同类型的值
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}
