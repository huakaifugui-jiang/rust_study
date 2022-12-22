/*
 * @Author: wlj
 * @Date: 2022-12-22 09:19:34
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-22 09:53:43
 * @Description:
 */
use gui::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {}
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();
    //当编写库的时候，我们不知道何人会在何时增加 SelectBox 类型，不过 Screen 的实现能够操作并绘制这个新类型，因为 SelectBox 实现了 Draw trait，这意味着它实现了 draw 方法。
    //这个概念 —— 只关心值所反映的信息而不是其具体类型 —— 类似于动态类型语言中称为 鸭子类型（duck typing）的概念:
    //如果它走起来像一只鸭子，叫起来像一只鸭子，那么它就是一只鸭子！
    //在示例  中 Screen 上的 run 实现中，run 并不需要知道各个组件的具体类型是什么。它并不检查组件是 Button 或者 SelectBox 的实例。
    //Box<dyn Draw> 作为 components vector 中值的类型，我们就定义了 Screen 为需要可以在其上调用 draw 方法的值。
    //使用 trait 对象和 Rust 类型系统来进行类似鸭子类型操作的优势是无需在运行时检查一个值是否实现了特定方法.或者担心在调用时因为值没有实现方法而产生错误。
    //如果值没有实现 trait 对象所需的 trait 则 Rust 不会编译这些代码。
    // let screen2 = Screen {
    //     components: vec![Box::new(String::from("Hi"))], //报错 它并没有实现Draw trait
    // };

    // screen2.run();

    //trait 对象执行动态分发
    //回忆一下第十章 “泛型代码的性能” 部分讨论过的，当对泛型使用 trait bound 时编译器所执行的单态化处理：
    //编译器为每一个被泛型类型参数代替的具体类型生成了函数和方法的非泛型实现。单态化产生的代码在执行 静态分发（static dispatch）。静态分发发生于编译器在编译时就知晓调用了什么方法的时候。
    //这与 动态分发 （dynamic dispatch）相对，这时编译器在编译时无法知晓调用了什么方法。在动态分发的场景下，编译器生成的代码到运行时才能确定调用了什么方法。
    //当使用 trait 对象时，Rust 必须使用动态分发。编译器无法知晓所有可能用于 trait 对象代码的类型，所以它也不知道应该调用哪个类型的哪个方法实现。为此，Rust 在运行时使用 trait 对象中的指针来知晓需要调用哪个方法。
    //动态分发也阻止编译器有选择的内联方法代码，这会相应的禁用一些优化。尽管在编写示例 17-5 和可以支持示例 17-9 中的代码的过程中确实获得了额外的灵活性，但仍然需要权衡取舍。

    //trait对象需要类型安全
    //只有对象安全（object-safe）的trait可以实现为特征对象。这里有一些复杂的规则来实现trait的对象安全，但在实践中，只有两个相关的规则。如果一个 trait 中定义的所有方法都符合以下规则，则该 trait 是对象安全的
    //返回值不是 Self
    //没有泛型类型的参数

    //Self 关键字是我们在 trait 与方法上的实现的别称，trait 对象必须是对象安全的，因为一旦使用 trait 对象，Rust 将不再知晓该实现的返回类型。
    //如果一个 trait 的方法返回了一个 Self 类型，但是该 trait 对象忘记了 Self 的确切类型，那么该方法将不能使用原本的类型。
    //当 trait 使用具体类型填充的泛型类型时也一样：具体类型成为实现 trait 的对象的一部分，当使用 trait 对象却忘了类型是什么时，无法知道应该用什么类型来填充泛型类型。

    //一个非对象安全的 trait 例子是标准库中的 Clone trait。Clone trait 中的 clone 方法的声明如下：
    pub trait Clone {
        fn clone(&self) -> Self;
    }
    //String 类型实现了 Clone trait，当我们在 String 的实例对象上调用 clone 方法时，我们会得到一个 String 类型实例对象。
    //相似地，如果我们调用 Vec<T> 实例对象上的 clone 方法，我们会得到一个 Vec<T> 类型的实例对象。clone 方法的标签需要知道哪个类型是 Self 类型，因为 Self 是它的返回类型。
    //当我们尝试编译一些违反 trait 对象的对象安全规则的代码时，我们会收到编译器的提示。
    pub struct Screen {
        pub components: Vec<Box<dyn Clone>>,//`main::Clone` cannot be made into an object 这个错误意味着我们不能将此 trait 用于 trait 对象。
    }
    //这个错误意味着我们不能将此 trait 用于 trait 对象。如果你想了解更多有关对象安全的细节，请移步至 Rust RFC 255 或查看 Rust Reference

}
