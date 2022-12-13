use std::fmt::Display;

/*
 * @Author: wulongjiang
 * @Date: 2022-12-12 22:47:39
 * @LastEditors: wulongjiang
 * @LastEditTime: 2022-12-13 20:36:07
 * @Description:
 * @FilePath: \traits\src\aggregator.rs
 */
//Summary trait定义，它包含由summarize 提供的行为
//这里使用trait关键字来声名一个trait，后面是trait的名字，在这个例子中是Summary。我们也声名这个trait为pub以便依赖这个crate的crate也可以使用这个trait，
//正如我们见过的一些实例一样。在大括号中声名描述实现这个trait类型所需要的行为的方法签名在这个例子中是 fn summarize(&self) -> String
pub trait Summary {
    // fn summarize(&self) -> String;
    // 有时为 trait 中的某些或全部方法提供默认的行为，而不是在每个类型的每个实现中都定义自己的行为是很有用的。
    //这样当为某个特定类型实现 trait 时，可以选择保留或重载每个方法的默认行为。
    fn summarize(&self) -> String {
        String::from("(Read more ...)")
    }
}

//现在我们定义了Summary trait的签名，接着就可以在多媒体聚合库中实现这个类型了。

// 例如，这里有多个存放了不同类型和属性文本的结构体：结构体 NewsArticle 用于存放发生于世界各地的新闻故事，
//而结构体 Tweet 最多只能存放 280 个字符的内容，以及像是否转推或是否是对推友的回复这样的元数据。
//我们想要创建一个名为 aggregator 的多媒体聚合库用来显示可能储存在 NewsArticle 或 Tweet 实例中的数据的总结。
//每一个结构体都需要的行为是他们是能够被总结的，这样的话就可以调用实例的 summarize 方法来请求总结

//存放世界各地的新闻故事
pub struct NewsArticle {
    pub headline: String, //标题
    pub location: String, //地区
    pub author: String,   //作者
    pub content: String,  //内容
}

//在类型上实现trait 类似于实现与trait无关的方法。区别在于impl关键字后，我们提供需要实现的trait的名称 接着是for 和需要实现trait的类型的名称
//在impl块中 使用trait定义中的方法签名，不过后面不再跟分号，叙事需要在大括号中编写函数体来为特定类型实现trait方法所拥有的行为
// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{} ,by {} ({})", self.headline, self.author, self.location)
//     }
// }

//不重载trait
impl Summary for NewsArticle {}

//存放 是否转推或是否是对推友的回复这样的元数据
pub struct Tweet {
    pub username: String, //用户名
    pub content: String,  //内容
    pub reply: bool,      //回复
    pub retweet: bool,    //转发
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

//trait作为参数
//知道了如何定义trait和在类型上实现这些trait后，我们可以探索一下如何使用trait来接受多种不同类型的参数。

//例如我们为NewsArticle和Tweet类型实现了Summarytrait。我们可以定义一个函数notify来调用其参数item上的summarize方法，
//该参数是实现了Summary trait 的某种类型。为此可以使用 impl Trait语法
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }
//我们可以传递任何 NewsArticle 或 Tweet 的实例来调用 notify。任何用其它如 String 或 i32 的类型调用该函数的代码都不能编译，因为它们没有实现 Summary。

//trait bound（特征约束 我翻译为）语法  它实际上是一种较长形式语法的语法糖
pub fn notify<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
}

//通过 + 指定 多个trait bound
// pub fn notify<T: Summary + Display>(item: &T) {}

//通过where简化trait bound
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

//简化后
//     fn some_function<T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {

//也可以在返回值中使用 impl Trait 语法，来返回实现了某个 trait 的类型
//有限制 只能返回一个 实现了trait的类型
fn returns_summarizable() -> impl Summary {
    // if switch {
    NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
        ),
    }
    // } else {
    //     Tweet {
    //         username: String::from("horse_ebooks"),
    //         content: String::from("of course, as you probably already know, people"),
    //         reply: false,
    //         retweet: false,
    //     }
    // }
}
