/*
    src/main.rs和src/lib.rs都叫做crate根，之所以这样叫他们是因为这两个文件的内容都分别在
    crate模块结构的根组成了一个crate模块，该结构体被称为模块树
    示例：
    crate
    └── front_of_house
        ├── hosting
        │   ├── add_to_waitlist
        │   └── seat_at_table
        └── serving
            ├── take_order
            ├── serve_order
            └── take_payment
    Rust使用路径的方式找到模块树中一个项的位置，就像在文件系统中使用路径的方式一样
        绝对路径和相对路径
    同时Rust存在私有性边界这一概念，这条界限不允许外部代码了解，调用和依赖被封装的细节，所以如果你想
    创建一个私有的函数或者结构体，你可以将其放入模块中。
    Rust默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的，父模块 中的项不可以使用子模块中的私有项。
    但是子模块中的项可以使用其父模块中的项，同时通过pub关键字可以创建公共项，是子模块中的内部部分暴露给父模块。
    同时，兄弟模块之间是公有的，这也是使用相对路径的基础
*/
pub fn eat_at_restaurant() {
    crate::front_of_house::hostings::add_to_waitlist();//使用绝对路径调用函数，因为add_to_waitlist()函数和
    //eat_at_restaurant()函数被定义在同一crate，这意味着我们可以使用crate作为起始的绝对路劲
    front_of_house::serving::take_payment(); //使用相对路径调用函数
    //因为front_of_house模块树和eat_at_restaurant()函数定义在同一层级，所以可以使用相对路径调用
}
mod front_of_house {
    pub mod hostings { //如果只在子模块这里加了pub，那么build的时候依然报错，因为只是使得模块公有，但是其内容还是私有的
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    pub mod serving {
        fn take_over() {}
        fn server_order() {}
        pub fn take_payment() {}
    }
}
fn server_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::server_order(); //通过使用super，可以引用父模块中的内容
    }
    fn cook_order() {}
}
