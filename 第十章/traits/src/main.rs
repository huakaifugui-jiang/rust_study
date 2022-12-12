/*
 * @Author: wulongjiang
 * @Date: 2022-12-12 20:55:26
 * @LastEditors: wulongjiang
 * @LastEditTime: 2022-12-12 22:55:46
 * @Description:Trait：定义共同行为
 * @see:https://kaisery.github.io/trpl-zh-cn/ch10-02-traits.html
 * @FilePath: \traits\src\main.rs
 */
//Trait:定义共同行为

//trait告诉Rust编译器某个特定类型拥有可能与其他类型共享的功能。可以通过trait以一种抽象的方式定义共享的行为。
//可以使用trait bounds 指定泛型是任何拥有特定行为的类型。

// 注意：trait 类似于其他语言中的常被称为 接口（interfaces）的功能，虽然有一些不同。

//这里有多个存放了不同类型和属性文本的结构体：结构体 NewsArticle 用于存放发生于世界各地的新闻故事，
//而结构体 Tweet 最多只能存放 280 个字符的内容，以及像是否转推或是否是对推友的回复这样的元数据。
//我们想要创建一个名为 aggregator 的多媒体聚合库用来显示可能储存在 NewsArticle 或 Tweet 实例中的数据的总结。
//每一个结构体都需要的行为是他们是能够被总结的，这样的话就可以调用实例的 summarize 方法来请求总结
mod aggregator;

use aggregator::{NewsArticle, Summary, Tweet}; //引入Summary trait 和 Tweet

// //其他依赖 aggregator crate 的 crate 也可以将 Summary 引入作用域以便为其自己的类型实现该 trait。
// 实现 trait 时需要注意的一个限制是，只有当至少一个 trait 或者要实现 trait 的类型位于 crate 的本地作用域时，才能为该类型实现 trait。
// 例如，可以为 aggregator crate 的自定义类型 Tweet 实现如标准库中的 Display trait，这是因为 Tweet 类型位于 aggregator crate 本地的作用域中。
// 类似地，也可以在 aggregator crate 中为 Vec<T> 实现 Summary，这是因为 Summary trait 位于 aggregator crate 本地作用域中。

// 但是不能为外部类型实现外部 trait。例如，不能在 aggregator crate 中为 Vec<T> 实现 Display trait。这是因为 Display 和 Vec<T> 都定义于标准库中，
// 它们并不位于 aggregator crate 本地作用域中。这个限制是被称为 相干性（coherence） 的程序属性的一部分，或者更具体的说是 孤儿规则（orphan rule）
// ，其得名于不存在父类型。这条规则确保了其他人编写的代码不会破坏你代码，
// 反之亦然。没有这条规则的话，两个 crate 可以分别对相同类型实现相同的 trait，而 Rust 将无从得知应该使用哪一个实现。
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}
