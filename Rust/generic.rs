use std::thread::sleep;

fn main() {
    let number_lsit = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_lsit);
    println!("the largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("the largest char is {}", result);
    //开始泛型部分
    let wont_work = Point {x: 5, y: 6};
    //let wont_work = Point {x: 5, y: 0.7};//这就会报错因为，他们必须拥有相同的泛型，这这个例子里面，当5被传递给结构体的时候，其泛型已经被具体化为整形，那么对y就会造成类型不匹配错误
    let wont_work = Point2 {x: 5, y: 0.2}; //使用两个泛型 就可以解决上面的问题了
    let p1 = Point2 {x:5 , y: 10.4};
    let p2 = Point2 {x: "hello", y: 'x'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}
struct Point<T> { //<>里面是需要使用到的泛型
    x: T,
    y: T
}

struct Point2<T, U> {
    x: T,
    y :U
}

impl<T> Point<T> {//x函数用户返回自身字段的引用，唯一值得注意的是必须在impl的后面跟上<T>，这样Rust就知道Point<>中的是泛型而不是类型
    fn x(&self) -> &T {
        &self.x
    }
}

//也可以为泛型中的某个类型来实现一个具体的函数
impl Point<f32> {//该方法只能用于浮点型的数据运算
    fn distance_from_origin(&self) ->f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y
        }
    }
}
//在性能方面，Rust通过在编译的时候进行泛型代码的单态化来保证效率。编译器寻找所有泛型代码被调用的位置并使用泛型代码针对具体类型生成代码
/* 标准库中的Option
enum Option<T> { 这里采用的就是泛型的方式
    Some(T),
    None
}

enum Result<T, E> { //对于Result可以说是拥有两个类型,这种定义使得Result可以很方便的表达任何可能的成功，也可能失败的操作
    Ok(T),
    Err(E)
}
*/
/*fn largest<T>(list: &[T]) -> T { //为了定义泛型化版本的函数，类型参数声明位于函数名称和参数列表中间的尖括号里面<>
    //就该函数来说，它拥有泛型类型T，它有一个参数list，其类型元素是T的slice，其返回值的也是T类型
    //在编写函数体的时候我们需要考虑到T所对应的所有的类型,但是下面的肯定没有考虑全面，所以会报错
    //cannot be applied to type `T`
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}*/

fn largest_i32(list: &[i32]) -> i32 {
    let mut largetst = list[0];
    for &item in list.iter() {
        if item > largetst {
            largetst = item;
        }
    }
    largetst
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}