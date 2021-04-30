/*
    和vector一样，hashmap同样把他们的数据建立在堆上面。
    hashmap 以键值对的形式存在，通过键可以对值进行快速索引，同时所有的键必须是相同类型的，值也一样必须是相同类型的
*/

use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Bule"), 0);
    scores.insert(String::from("Red"), 1);
    //另一个构建hashmap的方式是采用vector的collect方法，其中每一个元组包含一个键值对，collect方法将数据收集进一系列集合类型
    let teams = vec![String::from("Bule"), String::from("Yellow")];
    let initial_socres = vec![10, 50];
    //这里的<_, _> 的注解是必要的，因为可能collect很多不同的数据结构，除非显式的指定类型，否则Rust无法知道你想要的类型
    //但是对于键值对类型来说，可以使用下划线占位，而Rust可以根据vector中的数据类型推断出hashmap中包含的类型
    let scores2: HashMap<_, _> = teams.iter().zip(initial_socres.iter()).collect();//zip方法用于创建一个元组的vector

    /*
        hashmap和所有权
        对于i32这种的实现了copy trait的类型，其值都可以直接拷贝进入Hashmap，对于像String这样的拥有所有权的值，其值将被移动
        而Hashmap会成为这些值的所有者。
    */
    let field_name = String::from("Facorite color");
    let field_value = String::from("Bule");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //此时，这里的field_name和field_value不再有效

    //通过get可以访问Hashmap中的值
    let tmp_name = String::from("Facorite color");
    let over = map.get(&tmp_name); //get方法的返回类型是Option<V>，所以结果会被装入Some，如果不存在将返回None
    println!("the over is {:?}", over);
    for (key, value) in &map { //可以通过for循环遍历HashMap
        println!("{} : {}", key, value);
    }
    /*
        键值对的数量是可以增长的，但是每个键都只能关联一个值，下面看如何更新HashMap
    */
    map.insert(String::from("Facorite color"), String::from("Red")); //已经有了该键，替换对应的值为Red
    println!("the map now is {:?}", map.get(&tmp_name));
    // 只在键值对没有的时候插入
    map.entry(String::from("Bule")).or_insert(String::from("10")); //entry方法只在键没有对用的值的时候插入
    map.entry(String::from("Bule")).or_insert(String::from("20"));//entry的or_insert方法在键对应的值存在的时候就返回这个值的可变引用
    //如果不存在则将参数作为新的值插入，并返回新值的可变引用
    for (key, value) in &map {
        println!("{} : {}", key, value);
    }
    /*
        HashMap根据一个旧的值更新新的值
    */
    let text = "hello world wonderful world";
    let mut new_map = HashMap::new();
    for word in text.split_whitespace() {
        let count = new_map.entry(word).or_insert(0);// or_insert方法事实上会返回这个键的值的一个可变引用(&mut v)
        *count += 1; //统计单次出现的次数
    }
    println!("the new_map is {:?}", new_map);
}
