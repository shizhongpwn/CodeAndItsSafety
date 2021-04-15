/*OwnerShip 存在的价值在于跟踪代码的那些部分正在使用堆上面的数据，最大程度的减少堆上的重复数据，
同时清理堆上面的未使用数据以免耗尽内存空间，正是因为OwnerShip的存在，用户可以不用在过多的考虑
栈和堆的问题，相对于其他系统级编程语言，这是Rust的一大优势。
*/


/* OwnerShip Rules:
    Rust中所有的值都有一个变量叫做它的Owner
    一次只能拥有一个Owner
    当Owner被清理后，对应的值也会消失
*/
fn main() {
    println!("Hello, world!");
    let s = "Hello";  //字面值不能更改，因为其内容在编译的时候硬编码
    let s2 = String::new();// 对于字符串rust类似于 C++包含三部分：ptr , len , capacity,这三部分存在于栈中，但是ptr指向的字符串内容被存放在堆中
    let mut s3 = String::from("hello world");//当s3出该scope的时候，rust会调用drop函数
    s3.push_str(", dsadasada");
    println!("Hello {}", s3);

    
    let x = 4;
    let y = x; // 在栈上面形成两个相同的值
    let s4 = s3; // 和上面不同，此时其实是两个ptr指向了堆块里面的同一处内存,这是一种类似浅复制的方式，如果deep copy的话，需要使用clone
    println!("s4 is {}", s3); 
/* 上面的println!语句会报错：value borrowed here after move，这其实是Rust防止double free的一种方法，当
let s4 = s3; 之后，Rust认为s3不再有效，这就使得，在scope结束的时候，rust不再使用drop销毁s3，防止了同时销毁s3,s4来造成
double free */
}//就在此时，rust调用drop清理掉s4
