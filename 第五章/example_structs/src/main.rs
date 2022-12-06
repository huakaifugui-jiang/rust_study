/*
 * @Author: wlj
 * @Date: 2022-12-06 09:31:51
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-06 10:04:57
 * @Description: 结构体示例程序
 * @see:https://kaisery.github.io/trpl-zh-cn/ch05-02-example-structs.html
 */
//为了理解何时会需要使用结构体，让我们编写一个计算长方形面积的程序。我们会从单独的变量开始，接着重构程序直到使用结构体替代他们为止。
fn area(width: u32, height: u32) -> u32 {
    width * height
} //缺点：函数 area 本应该计算一个长方形的面积，不过函数却有两个参数。这两个参数是相关联的，不过程序本身却没有表现出这一点。将长度和宽度组合在一起将更易懂也更易处理

//使用结构体
#[derive(Debug)] //为了打印结构体 它可以给 struct加上一个 Debug trait，它允许我们以一种对开发者有帮助的方式打印结构体，以便当我们调试代码时能看到它的值。
struct Rectangle {
    width: u32,
    height: u32,
}
fn better_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let width1: u32 = 30;
    let height1: u32 = 30;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    //使用结构体
    let rect = Rectangle {
        width: 30,
        height: 30,
    };
    println!(
        "The area of the rectangle is {} square pixels by better_area.",
        better_area(&rect) //函数 area 现在被定义为接收一个名叫 rectangle 的参数，其类型是一个结构体 Rectangle 实例的不可变借用。第四章讲到过，我们希望借用结构体而不是获取它的所有权，
                           //这样 main 函数就可以保持 rect1 的所有权并继续使用它，所以这就是为什么在函数签名和调用的地方会有 &。
    );

    //打印结构体的方法
    // println!("Rectangle : {}", rect);报错 the trait `std::fmt::Display` is not implemented for `Rectangle` 发现 {} 是需要Display格式的
    //下面两种方式可以打印Debug格式
    println!("Rectangle : {:?}", &rect);
    println!("Rectangle : {:#?}", rect); //引用也可以忽略

    //另外一种打印Debug格式的方法 dbg!宏 。dbg! 宏接收一个表达式的所有权（与 println! 宏相反，后者接收的是引用），
    //打印出代码中调用 dbg! 宏时所在的文件和行号，以及该表达式的结果值，并返回该值的所有权。
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
    println!("rect1 : {:?}", dbg!(&rect1));

    //除了 Debug trait，Rust 还为我们提供了很多可以通过 derive 属性来使用的 trait，他们可以为我们的自定义类型增加实用的行为。
    //。附录 C 【https://kaisery.github.io/trpl-zh-cn/appendix-03-derivable-traits.html】中列出了这些 trait 和行为。第十章会介绍如何通过自定义行为来实现这些 trait，
    //同时还有如何创建你自己的 trait。除了 derive 之外，还有很多属性；更多信息请参见 Rust Reference 【https://doc.rust-lang.org/stable/reference/attributes.html】 的 Attributes 部分。
}
