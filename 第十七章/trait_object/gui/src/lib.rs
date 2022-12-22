/*
 * @Author: wlj
 * @Date: 2022-12-22 08:15:42
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-22 09:21:15
 * @Description: 顾及不同类型值的 trait 对象
 * @see:https://kaisery.github.io/trpl-zh-cn/ch17-02-trait-objects.html
 */

 
////在第八章中，我们谈到了 vector 只能存储同种类型元素的局限。但是当时我们也举例使用了SpreadsheetCell 枚举来储存整型，浮点型和文本成员的替代方案。
//这意味着可以在每个单元中储存不同类型的数据，并仍能拥有一个代表一排单元的 vector。这在当编译代码时就知道希望可以交替使用的类型为固定集合的情况下是完全可行的。
// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }

// let row = vec![
//     SpreadsheetCell::Int(3),
//     SpreadsheetCell::Text(String::from("blue")),
//     SpreadsheetCell::Float(10.12),
//     SpreadsheetCell::Float(10.12),
// ];

//然而有时我们希望库用户在特定情况下能够扩展有效的类型集合。为了展示如何实现这一点，这里将创建一个图形用户界面(Graphical User Interface GUI)工具的例子，
//它通过遍历列表并调用每一个项目的 draw 方法来将其绘制到屏幕上 —— 此乃一个 GUI 工具的常见技术。我们将要创建一个叫做 gui 的库 crate，它含一个 GUI 库的结构.
//这个 GUI 库包含一些可供开发者使用的类型，比如 Button 或 TextField。在此之上，gui 的用户希望创建自定义的可以绘制于屏幕上的类型：比如，一个程序员可能会增加 Image，另一个可能会增加 SelectBox。
//这个例子中并不会实现一个功能完善的 GUI 库，不过会展示其中各个部分是如何结合在一起的。编写库的时候，我们不可能知晓并定义所有其他程序员希望创建的类型。
//我们所知晓的是 gui 需要记录一系列不同类型的值，并需要能够对其中每一个值调用 draw 方法。这里无需知道调用 draw 方法时具体会发生什么，只要该值会有那个方法可供我们调用。
//在拥有继承的语言中，可以定义一个名为 Component 的类，该类上有一个 draw 方法。其他的类比如 Button、Image 和 SelectBox 会从 Component 派生并因此继承 draw 方法。
//它们各自都可以覆盖 draw 方法来定义自己的行为，但是框架会把所有这些类型当作是 Component 的实例，并在其上调用 draw。不过 Rust 并没有继承，我们得另寻出路。

//定义通用行为的 trait
//为了实现 gui 所期望的行为，让我们定义一个 Draw trait，其中包含名为 draw 的方法。
//接着可以定义一个存放 trait 对象（trait object） 的 vector。trait 对象指向一个实现了我们指定 trait 的类型的实例，以及一个用于在运行时查找该类型的trait方法的表。
//我们通过指定某种指针来创建 trait 对象，例如 & 引用或 Box<T> 智能指针，还有 dyn keyword， 以及指定相关的 trait（第十九章 “动态大小类型和 Sized trait” 部分会介绍 trait 对象必须使用指针的原因）。
//我们可以使用 trait 对象代替泛型或具体类型。任何使用 trait 对象的位置，Rust 的类型系统会在编译时确保任何在此上下文中使用的值会实现其 trait 对象的 trait。如此便无需在编译时就知晓所有可能的类型。
//之前提到过，Rust 刻意不将结构体与枚举称为 “对象”，以便与其他语言中的对象相区别。在结构体或枚举中，结构体字段中的数据和 impl 块中的行为是分开的，不同于其他语言中将数据和行为组合进一个称为对象的概念中。
//trait 对象将数据和行为两者相结合，从这种意义上说 则 其更类似其他语言中的对象。不过 trait 对象不同于传统的对象，因为不能向 trait 对象增加数据。
//trait 对象并不像其他语言中的对象那么通用：其（trait 对象）具体的作用是允许对通用行为进行抽象。
//太抽象了 看具体例子：

//定义了一个带有draw方法的trait Draw
pub trait Draw {
    fn draw(&self) {}
}

//定义了一个存放名叫components的vector的结构体Screen。这个vector的类型是Box<dyn Draw>，此为一个trait对象：他是Box中任何实现了Draw trait的类型的替身
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

//在Screen结构体上，我们定义了一个方法run，该方法会对其components上的每一个组件调用draw方法。
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
//看到这里感觉会有点懵，我们之前是如何使用trait的呢？
// pub struct Screen2<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen2<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }
//这限制了 Screen 实例必须拥有一个全是 Button 类型或者全是 TextField 类型的组件列表。如果只需要同质（相同类型）集合，则倾向于使用泛型和 trait bound，因为其定义会在编译时采用具体类型进行单态化。
//trait 对象 与 trait bound 的泛型类型参数的结构体不同。泛型类型参数一次只能替代一个具体类型，而 trait 对象则允许在运行时替代多种具体类型。
//另一方面，通过使用 trait 对象的方法，一个 Screen 实例可以存放一个既能包含 Box<Button>，也能包含 Box<TextField> 的 Vec<T>。让我们看看它是如何工作的，接着会讲到其运行时性能影响。

//实现trait 再一次重申，真正实现 GUI 库超出了本书的范畴，所以 draw 方法体中不会有任何有意义的实现。
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {}
}
