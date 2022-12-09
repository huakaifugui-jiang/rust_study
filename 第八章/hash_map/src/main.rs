/*
 * @Author: wlj
 * @Date: 2022-12-09 14:41:17
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-09 17:16:21
 * @Description: 使用 Hash Map 储存键值对
 * @see:https://kaisery.github.io/trpl-zh-cn/ch08-03-hash-maps.html
 */
//使用 Hash Map 存储键值对

//最后介绍的常用集合类型是哈希map（hash map）。HashMap<K,V>类型存储了一个键类型 K 对应一个值类型 V 的映射。
//它通过一个哈希函数（hashing function）来实现映射，决定如何将键和值放入内存中。很多变成语言支持这种数据结构，不过通常有不同的名字：
//哈希、map、对象、哈希表或者关联数组

//哈希map 可以用于需要任何类型作为键来寻找数据的情况，而不是像vector那样通过索引。例如在一个游戏中，你可以将每个团队的分数记录到哈希map中，
//而其中键是队伍的名字而值是每个队伍的分数。给出一个队名，就能得到他们的得分。

// 本章我们会介绍哈希 map 的基本 API，不过还有更多吸引人的功能隐藏于标准库在 HashMap<K, V> 上定义的函数中。一如既往请查看标准库文档来了解更多信息。

use std::collections::HashMap; //注意必须首先 use 标准库中集合部分的 HashMap。在这三个常用集合中，HashMap 是最不常用的，
                               //所以并没有被 prelude 自动引用。标准库中对 HashMap 的支持也相对较少，例如，并没有内建的构建宏

fn learn_new_hash_map() {
    //创建一个哈希map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // scores.insert(String::from("Yellow2"), "4564"); 类似于vector，哈希map所有的键必须是相同类型，值也必须是相同类型

    //另一个构建哈希map的方法时在一个元组的vector上使用迭代器（iterator）和collect方法，其中每个元组包含一个键值对
    //我们会在第十三章介绍迭代器和其关联的方法。
    //collect方法可以将数据收集进一系列的集合，包括hashmap。例如，如果队伍的名字和初始化分数分别在两个vector中，可以
    //使用zip方法来创建一个元组的迭代器

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    //这里 HashMap<_, _> 类型注解是必要的，因为可能 collect 为很多不同的数据结构，而除非显式指定否则 Rust 无从得知你需要的类型。但是对于键和值的类型参数来说，
    //可以使用下划线占位，而 Rust 能够根据 vector 中数据的类型推断出 HashMap 所包含的类型。

    //哈希map和所有权

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //对于像 i32 这样的实现了 Copy trait 的类型，其值可以拷贝进哈希 map。
    //对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者，
    // 这里 field_name 和 field_value 不再有效，

    //访问哈希map中的值
    //可以通过get方法并提供对应的键来从哈希map中获取值。

    let score = scores.get(&String::from("Blue")); //返回的是一个Option

    if let Some(value) = score {
        println!("value is {}", value);
    }

    //遍历
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

//更新hash map
//尽管键值对的数量是可以增长的，不过任何时候，每个键只能关联一个值。当我们想要改变哈希map中的数据时，必须决定如何处理一个键已经有了值的情况。
//可以选择完全无视旧值并用新值代替旧值。可以选择保留旧值而忽略新值，并只在键没有对应值时增加新值。或者可以结合新旧两值。
fn learn_update_hash_map() {
    //覆盖一个值
    //如果我们插入了一个键值对，接着用相同的键插入一个不同的值，与这个键相关联的旧值将被替换。

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);
    println!("learn_update_hash_map {:?}", scores); //原来的10被覆盖了

    //只在键没有对应值时插入
    //我们经常会检查某个特定的键是否有值。
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    //使用 entry 方法只在键没有对应一个值时插入
    //entry的or_insert方法在键对应的值存在时就返回这个值的可变引用，如果不存在则将参数作为新值插入并返回新值的可变引用
    println!("learn_update_hash_map {:?}", scores); //原来的10被覆盖了

    //根据旧值更新一个值
    //另一个常见的哈希map的应用场景是找到一个键对应的值并且根据旧的值更新它。

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); //返回的是新的值的引用
        *count += 1;
    }
    println!("word : {:?}", map);
    // 这会打印出 {"world": 2, "hello": 1, "wonderful": 1}。
    // split_whitespace 方法会迭代 text 的值由空格分隔的子 slice。
    // or_insert 方法返回这个键的值的一个可变引用（&mut V）。
    // 这里我们将这个可变引用储存在 count 变量中，所以为了赋值必须首先使用星号（*）解引用 count。
    // 这个可变引用在 for 循环的结尾离开作用域，这样所有这些改变都是安全的并符合借用规则。
}

//哈希函数 HashMap HashMap 默认使用一种叫做 SipHash 的哈希函数，它可以抵御涉及哈希表（hash table）1 的拒绝服务（Denial of Service, DoS）攻击。
// 然而这并不是可用的最快的算法，不过为了更高的安全性值得付出一些性能的代价。如果性能监测显示此哈希函数非常慢，以致于你无法接受，
// 你可以指定一个不同的 hasher 来切换为其它函数。hasher 是一个实现了 BuildHasher trait 的类型。第十章会讨论 trait 和如何实现它们。
// 你并不需要从头开始实现你自己的 hasher；crates.io 有其他人分享的实现了许多常用哈希算法的 hasher 的库。

fn main() {
    learn_new_hash_map();
    learn_update_hash_map();
}
