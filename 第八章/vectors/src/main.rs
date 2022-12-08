/*
 * @Author: wulongjiang
 * @Date: 2022-12-08 22:38:15
 * @LastEditors: wulongjiang
 * @LastEditTime: 2022-12-08 23:19:31
 * @Description:使用 Vector 储存列表
 * @see：https://kaisery.github.io/trpl-zh-cn/ch08-01-vectors.html
 * @FilePath: \rust_study\第八章\vectors\src\main.rs
 */
//使用Vector存储列表
//我们要将的第一个类型是Vec<T>,也被称为vector。vector允许我们在一个单独的数据结构总存储多于一个的值，
// 它在内存中彼此相邻的排列所有的值。vector只能存储相同类型的值。它们在拥有 一系列项的场景下非常实用，例如文件的中的文本行或是购物车中商品的价格

fn main() {
    let v: Vec<i32> = Vec::new(); //新建一个空的 vector 来储存 i32 类型的值

    //通常我们会初始化来创建一个Vec<T> 而 Rust 会推断出储存值的类型，所以会很少需要这些类型注解。
    //为了方便Rust提供了vec!宏，这个宏会根据我们提供的值来创建一个新的vector。

    let v2 = vec![1, 2, 3]; //新建一个拥有值 1、2 和 3 的 Vec<i32>。推断为 i32 是因为这是默认整型类型，

    //更新vector
    let mut v3 = Vec::new(); //如第三章中讨论的任何变量一样，如果想要能够改变它的值，必须使用 mut 关键字使其可变。
                             //放入其中的所有值都是 i32 类型的，而且 Rust 也根据数据做出如此判断，所以不需要 Vec<i32> 注解。
    v3.push(5); //向vector结尾增加值

    {
        let v4 = vec![1, 2, 3, 4];

        // 处理变量 v4
    } // <- 这里 v4 离开作用域并被丢弃

    //读取vector元素
    let mut v5 = vec![1, 2, 3, 4, 5];
    println!("{}", &v5[0]);
    //v.get 返回一个 Option<&T>。
    match v5.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    //尝试访问一个包含 5 个元素的 vector 的索引 100 处的元素
    // println!("{}", &v5[100]); //这样程序会panic 恐慌
    match v5.get(100) {
        //返回None
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    //
    let mut v6 = vec![1, 2, 3, 4, 5];
    let first = &v6[0]; //指向v6第一个元素的地址
    v6.push(10);
    // println!("The first element is: {}", first);//报错
    //为什么这样会报错呢 因为 如果在vector结尾增加新的元素时，在没有足够空间将所有元素依次相邻存放的情况下，可能会要求分配新的内存
    //并将老的元素拷贝到新的 空间。这时第一个元素引用就指向了被释放的内存。

    //遍历
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; //因为这里时引用所以必须解引用
        println!("{}", i);
    }

    //使用枚举来储存多种类型
    //定义一个枚举，以便能在 vector 中存放不同类型的数据
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    //Rust 在编译时就必须准确的知道 vector 中类型的原因在于它需要知道储存每个元素到底需要多少内存。第二个好处是可以准确的知道这个 vector 中允许什么类型。如果 Rust 允许 vector 存放任意类型，那么当对 vector 元素执行操作时一个或多个类型的值就有可能会造成错误。
    //使用枚举外加 match 意味着 Rust 能在编译时就保证总是会处理所有可能的情况，正如第六章讲到的那样。

    //现在我们了解了一些使用 vector 的最常见的方式，请一定去看看标准库中 Vec 定义的很多其他实用方法的 API 文档。
    // [https://doc.rust-lang.org/std/vec/struct.Vec.html]
    //例如，除了 push 之外还有一个 pop 方法，它会移除并返回 vector 的最后一个元素。让我们继续下一个集合类型：String！
    //个人觉得跟js的Array有点像
}
