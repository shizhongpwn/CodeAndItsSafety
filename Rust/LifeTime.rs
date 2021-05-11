fn main() {
 /*   let r;
    {
        let x = 5;
        r = &x;
    }
    println!(" r : {}", r);
    这段代码是没办法通过的,因为 x 在离开作用域的时候会被销毁，而r是x的借用，Rust的借用检查器通过比较作用域来确保所有的借用都是有效的，
  */
    let string1 =  String::from("abcd");
    let string2 = "xyz";
    let Result = longest(string1.as_str(), string2);
    println!("the longest is {}", Result);

    let strin3 = String::from("long string is long");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a ...");
    println!("the first sentence is {}", first_sentence);
    let i = ImportantExcept { part: first_sentence }; // 这里存放了novel 变量所拥有的的String的第一个句子的引用，
    /*
        novel 的数据在 ImportantExcerpt 实例创建之前就存在。另外，直到 ImportantExcerpt 离开作用域之后 novel 都不会离开作用域，所以 ImportantExcerpt 实例中的引用是有效的。
    */

}

/*
    下面函数使用的时候报错：expected named lifetime parameter
    这是表明,返回值需要时一个泛型声明周期参数，因为Rust不知道要返回的引用是指向x还是指向y，事实上我们也不知道，
    借用检查器同时也无法断定，因为它不知道x和y的生命周期是如何与返回值的生命周期相关联的。为了修复这个错误，我们
    将增加泛型声明周期参数来定义引用之间的关系以便借用检查器可以进行分析。
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

//生命周期注解语法
/*
    声明周期注解并不改变任何引用的生命周期的长短。与函数签名中制定了泛型参数后就可以接受任何类型一样，当指定了泛型声明
    周期参数之后函数也可以接受任何生命周期的引用。生命周期注解描述了多个引用生命周期相互的关系，而不影响其生命周期。
    &i32        // 引用
    &'a i32     // 带有显式生命周期的引用
    &'a mut i32 // 带有显式生命周期的可变引用

    单个声明周期注解其实是没有意义的，因为声明周期注解的意义在于告诉Rust多个引用之间的生命周期的关系，假如有两个参数first和
    second，生命周期注解都是'a那么表示first和second必须与这泛型生命周期存在的一样久
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }else {
        y
    }
}
// 在结构体里面定义声明周期注解

struct ImportantExcept<'a> { // 这个生命周期注解该结构体的实例不能比其part字段中的引用存在更久
    part: &'a str,
}

//该函数使用了引用，但是却没有以注解的方式表明生命周期，这是因为Rust把一些特定的模式编码进了Rust引用分析，被称为声明周期省略规则
//函数或者方法的参数的生命周期被称为输入生命周期，而返回的生命值被称为输出生命周期
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }
    &s[..]
}
/*
    Rust编译器采用三条规则来判断引用何时不需要表明注解，第一条适用于输入生命周期，后两条规则适用于输出生命周期，如果编译器在检查完这三条规则之后
    仍然存在没有计算出的生命周期的引用，编译器将报错，这些规则适用于fn定义，已经impl块。
    https://kaisery.github.io/trpl-zh-cn/ch10-03-lifetime-syntax.html
*/



