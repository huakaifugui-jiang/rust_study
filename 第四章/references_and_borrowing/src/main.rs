/*
 * @Author: wulongjiang
 * @Date: 2022-12-04 17:43:43
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-05 11:01:09
 * @Description:引用与借用
 * @FilePath: \rust_study\第四章\references_and_borrowing\src\main.rs
 * @see:https://kaisery.github.io/trpl-zh-cn/ch04-02-references-and-borrowing.html
 */

//引用
fn overship_fn(s: String) -> String {
    s
}

fn learn_reference(s: &String) -> usize {
    //s是String的引用
    s.len()
} //这里s离开了作用域。但因为它并不拥有引用值的所有权，
  // 所以什么也不会发生

// fn learn_change(some_string: &String) {
//     some_string.push_str(",learn_change");
// }

fn learn_mut_change(some_string: &mut String) {
    some_string.push_str(",learn_mut_change");
}

//悬垂引用
fn learn_dangling_references() -> &String { /
    let s = String::from("hello,learn_dangling_references");
    &s //返回字符串s的引用
}//这里s离开作用域被丢弃了，其内存已经被释放了。

fn main() {
    let s = String::from("hello,world");
    let s1 = overship_fn(s); //就如上一小节提到的，s的所有权（overship）被移到到了overship_fn 函数中，下面的作用域就不能再 使用s;除非你再创建一个变量，并且返回s。这在实际中是十分繁琐的。
                             //所以为了调用后能再使用String 我们可以提供一个String的值的引用（reference）。引用（reference）像一个指针，因为它是一个地址
                             //我们可以借此访问存储于该地址的属于它的变量的数据。与指针不同的是，引用确保指向某个特定类型的有效值。

    //如何使用引用？  & 符号就是 引用
    // 注意：与使用 & 引用相反的操作是 解引用（dereferencing），它使用解引用运算符，*。我们将会在第八章遇到一些解引用运算符，并在第十五章详细讨论解引用。
    let r1 = String::from("hello,reference");
    let len = learn_reference(&r1); //创建一个指向r1的引用(这这种行为叫 借用（borrowing） ) 注意：我们 引用 了 r1 而不是获取了它的所有权。所以后面是可以再使用r1的
    println!("r1 : {} , len : {}", r1, len);

    //如果尝试借用的变量      正如变量默认是不可变的，引用也一样。（默认）不允许修改引用的值。
    //learn_change(&r1); //`some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable 报错 some_string是一个&引用，所以它引用的数据不能被借用为可变的。

    //可变引用（mutable reference）
    let mut r2 = String::from("hello");
    learn_mut_change(&mut r2);
    //可变引用有一个很大的限制：如果你创建了一个变量的引用（借用），那么再 借用 和 使用它 的 期间 是不能  在创建一个 引用 的      如：
    let mut b = String::from("hello ,multable reference");
    let b1 = &mut b; //创建了一个b的引用（借用） b1
                     // let b2 = &mut b;        //----期间 在创建一个引用 会报错
                     //let b2 = &b; //-----期间    Rust 在同时使用可变与不可变引用时也采用的类似的规则 所以会报错
    println!("b1 : {b1}"); // ------------使用b1
                           //具体限制查看https://doc.rust-lang.org/1.8.0/book/references-and-borrowing.html#mut-references 个人觉得Rust程序设计语言 简体中文版 这一段 有点怪异
                           //所以一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。
                           //编译器在作用域结束之前判断不再使用的引用的能力被称为 非词法作用域生命周期（Non-Lexical Lifetimes，简称 NLL）。你可以在 The Edition Guide 【https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#non-lexical-lifetimes】 中阅读更多关于它的信息。
                           //为什么要有这个限制呢？，它可以防止同一时间对同一数据存在多个可变引用。新Rustacean们经常难以适应这一点，因为大部分语言中变量任何时候都是可变的。这个限制的好处是 Rust 可以在编译时就避免 数据竞争 类似于竞态条件[https://zh.wikipedia.org/wiki/%E7%AB%B6%E7%88%AD%E5%8D%B1%E5%AE%B3]它可由这三个行为造成：
                           // 1.两个或更多个指针同时访问同一数据。
                           // 2.至少有一个指针被用来写入数据。
                           // 3.没有同步数据访问的机制

    //悬垂引用（Dangling References）
    //在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针（dangling pointer）也叫野指针
    //所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。当所指向的对象被释放或者收回，但是对该指针没有作任何的修改，以至于该指针仍旧指向已经回收的内存地址，此情况下该指针便称悬垂指针。
    //相比之下，在 Rust 中编译器确保引用永远也不会变成悬垂状态：当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。
    //let reference_to_nothing = learn_dangling_references(); //因为learn_dangling_references的s释放了，所以这里会指向一个一个已经被回收的地址。会报错。
    //报错信息 expected named lifetime parameter 错误信息引用了一个我们还未介绍的功能：生命周期（lifetimes）。第十章会详细介绍生命周期。不过，如果你不理会生命周期部分，错误信息中确实包含了为什么这段代码有问题的关键信息：
    //this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    //这个函数的返回类型包含一个借用值，但是没有可以借用的值
} 
