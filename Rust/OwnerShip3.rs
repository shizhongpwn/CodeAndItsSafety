/* */

fn main() {
    let s1 = give_ownership();//这里会get一个值给s1,它是give_ownership move something传递过来的
    let s2 = String::from("hello");
    let s3 = takes_and_give_back(s2);//s2被move to a_string,然后a_string被move to s3.
    let (s4, s5) = calculate_length(s1);
    println!("The length is '{}' is {}", s4, s5);
}// 这里s3被销毁，s2被move了已经，所以不会发生什么特殊的，s1被销毁

fn give_ownership() -> String {
    let something = String::from("hello");
    something // 这里something被move给calling fuction
}

fn takes_and_give_back(a_string: String) -> String {
    a_string // a_string 被move to the calling function.
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

