fn main() {


    let mut User1  = User {
        email: String::from("someone@example.com"),
        username: String::from("clock"),
        Sign_in_count: 1,
        active: true
    };
    User1.email = String::from("clock@example.com");
    println!("the user's email is {}", User1.email);
    let user_test = build_user(String::from("clock@qq.com"), String::from("clock2"));
    println!("the user_test's eamil is {}", user_test.email);

    let User2 = User {
        email: String::from("test@qq.com"),
        username: String::from("shadan"),
        Sign_in_count: User1.Sign_in_count,
        active: User1.active
    };
    // 上面这种有点复杂，可以进行简化,想下面这样
    let User3 = User {
        email: String::from("test2@qq.com"),
        username: String::from("hanhan"),
        ..User2
    };
    println!("the test3 is {}", User3.Sign_in_count);
    //下面展示元组结构体
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(1, 3, 65);
    let origin = Point(3, 5, 7);
    println!("the black one is {}", black.1);
    /*
    可以看到元组结构体没有具体的字段名字，只有字段的类型，当你想给整个元组起一个名字，
    并使得元组成为与其他元组不同的类型的时候，元组结构体很有用，这时候像常规结构体那样给每个
    字段进行命名就会显示的很多余。
    */
    //同时也可以定义一个空结构体，被称为类单元结构体，其为unit类型
}
struct NULL {
    
}
struct User {
    username: String,
    email: String,
    Sign_in_count: u64,
    active: bool
}
fn build_user(email: String, username: String) -> User {
    User {
        username: username, //此处也可以写成 username, 因为变量与参数名称相同
        email: email,
        Sign_in_count: 1,
        active: true
    }
}

