/*
    trait告诉rust编译器某个特定的类型有可能与其他类型共享的功能。可能通过trait以一种抽象的方式定义共享的行为。
    trait有点类似于接口但是略有不同。
    可以使用trait bounds 指定泛型是任何拥有特定行为的类型
*/
/*
    孤儿原则：
        只有当trait或者要实现trait的类型位于crate的本地作用域，才能为该类型实现trait,
*/
use std::iter::Sum;
use std::fmt::Display;

fn main() {
    println!("Hello, world!");
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course"),
        reply: false,
        retweet: false
    };
    println!("I new a sweet {}", tweet.summarize());

}

    pub trait Summary {
        //大括号里面声明实现这个trait类型所需要的方法的签名
        fn summarize(&self) -> String;  //这里是函数签名，以分号结尾
        //可以实现多个函数签名，编译器会保证任何实现Summary trait的类型都拥有与签名一致的summarize的方法
    }


    pub struct NewArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String
    }

    impl Summary for NewArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{} {}", self.username, self.content)
        }
    }
/*
    从上面看trait类似于实现与类型无关的方法，但是我们要在impl后面跟上需要实现该trait方法的类型的名称，然后再里面实现函数,接着是for和要实现的trait的名称。
    一旦我们为Tweet和NewArticle实现了该trait对应的方法，那么其实例就可以对其实现对该方法的函数调用。
*/

//同时，有时候可以为trait实现某些默认方法，而不是在每一个类型里面都对trait实现自己对应的行为，这样当实现某个类型对应的trait方法的时候可以选择保留或者重载某一个默认行为
pub trait Summary2 {
    fn summarize(&self) -> String { //这里就是直接实现了一个默认的方法，如果在具体类型里面实现了相同的方法，那么就会以重载的方式把默认方法替换掉
        String::from("(Read more...)")
    }
}
// trait作为参数
pub fn notify(iterm: impl Summary2) { //对于item参数，我们使用impl关键字和trait名称，而不是具体类型，该参数支持实现了的任何trait类型
    //在notify函数体里面，可以调用任何来之Summary2 trait的方法，比如summarize。
    //在参数传递中该函数可以接受任何NewArticle和Tweet的实例来实现notify，但是任何其他比如String和i32等没有实现在trait的类型不能作为参数使用，因为它们没有实现trait
    println!("Breaking news {}!", iterm.summarize());
}

//Trait Bound语法
pub fn notify2<T: Summary2>(iterm: T) {//trait bound 与泛型参数声明在一起
    println!("Breaking news {}!", iterm.summarize());
}

pub fn notify3(item1: impl Summary2, item2: impl Summary2) {//这可以获取两个实现了Summary2的参数

}

pub fn notify4<T: Summary2>(item1: T, item2: T) {  //这样就强制实现了参数必须为同一类型

}

//通过 +  指定多个trait bound
/*
    如果notify需要显示item的格式化形式，同时需要调用summarize函数，那么就需要同时实现两个不同的trait 这个可以通过 + 实现
*/

pub fn notify5(item: impl Summary2 + Summary) {

}
//同时 + 的语法也适用于泛型操作
pub fn notify6<T: Summary + Summary2>(item: T) { //可以看到 + 也同时适用于trait bound
//这就使得notify6的函数体可以同时调用summarize并使用自己定义的函数返回相关的字符串
}

// 通过where来简化trait bound
pub fn notify7<T, U>(t :T, u: U) -> i32
    where T: Display + Clone,  //这仅仅是为了让函数签名更加的具备可阅读性
          U: Clone
{

}
// 返回实现了trait的类型
fn return_summarizable() -> impl Summary2 { // 使用impl指定了该函数返回的是某个实现了该trait的类型，这就使得调用方不知道返回的具体类型
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course"),
        reply: false,
        retweet: false
    }
}

