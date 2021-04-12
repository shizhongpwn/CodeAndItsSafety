# Rust

Rust is an expression-based language.

statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value. Let’s look at some examples.

rust和C/C++不同，在controlFlow里面，if必须Get一个bool类型，加入给了一个number:

~~~
 --> src/main.rs:3:8
  |
3 |     if number {
  |        ^^^^^^ expected `bool`, found integer
~~~

同时，rust必须在编译时，确定自己的类型：

~~~rust
fn main() {
    let condition = true;
    let number = if condition {5} else {"six"};
    println!("the number is {}", number);
}
~~~

这样就会报错，因为编译器无法确定number的类型，同时不允许一个变量拥有多种类型。