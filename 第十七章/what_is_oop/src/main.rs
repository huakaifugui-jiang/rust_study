/*
 * @Author: wlj
 * @Date: 2022-12-21 16:48:10
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-21 17:42:53
 * @Description: 面向对象语言的特征
 * @see:https://kaisery.github.io/trpl-zh-cn/ch17-01-what-is-oo.html
 */

//面向对象语言的特征
//关于一个语言被称为面向对象所需的功能，在编程社区内并未达成一致意见。Rust 被很多不同的编程范式影响，包括面向对象编程；比如第十三章提到了来自函数式编程的特性。
//面向对象编程语言所共享的一些特性往往是对象、封装和继承。让我们看一下这每一个概念的含义以及 Rust 是否支持他们。

//对象包含数据和行为
//由 Erich Gamma、Richard Helm、Ralph Johnson 和 John Vlissides（Addison-Wesley Professional, 1994）编写的书 Design Patterns: Elements of Reusable Object-Oriented Software
//被俗称为 The Gang of Four (字面意思为 “四人帮”)，它是面向对象编程模式的目录。它这样定义面向对象编程：
//Object-oriented programs are made up of objects. An object packages both data and the procedures that operate on that data. The procedures are typically called methods or operations.
//面向对象的程序是由对象组成的。一个 对象 包含数据和操作这些数据的过程。这些过程通常被称为 方法 或 操作。

//在这个定义下，Rust 是面向对象的：结构体和枚举包含数据而 impl 块提供了在结构体和枚举之上的方法。虽然带有方法的结构体和枚举并不被 称为 对象，但是他们提供了与对象相同的功能，参考 The Gang of Four 中对象的定义。

//封装隐藏了实现细节
//另一个通常与面向对象编程相关的方面是 封装（encapsulation）的思想:
//对象的实现细节不能被使用对象的代码获取到。所以唯一与对象交互的方式是通过对象提供的公有 API；使用对象的代码无法深入到对象内部并直接改变数据或者行为。封装使得改变和重构对象的内部时无需改变使用对象的代码。
//就像我们在第七章讨论的那样：可以使用 pub 关键字来决定模块、类型、函数和方法是公有的，而默认情况下其他一切都是私有的
//比如，我们可以定义一个包含一个 i32 类型 vector 的结构体 AveragedCollection 。结构体也可以有一个字段，该字段保存了 vector 中所有值的平均值。
//这样，希望知道结构体中的 vector 的平均值的人可以随时获取它，而无需自己计算。
//换句话说，AveragedCollection 会为我们缓存平均值结果。
//注意，结构体自身被标记为 pub，这样其他代码就可以使用这个结构体，但是在结构体内部的字段仍然是私有的。
//这是非常重要的，因为我们希望保证变量被增加到列表或者被从列表删除时，也会同时更新平均值。可以通过在结构体上实现 add、remove 和 average 方法来做到这一点
pub mod Test {
    #[derive(Debug)]
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }

    impl AveragedCollection {
        pub fn new(list: Vec<i32>) -> AveragedCollection {
            let total: i32 = list.iter().sum();
            let average = total as f64 / list.len() as f64;
            AveragedCollection { list, average }
        }
        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None => None,
            }
        }

        pub fn average(&self) -> f64 {
            self.average
        }
        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }
}

use crate::Test::AveragedCollection;
fn main() {
    //公有方法 new,add、remove 和 average 是修改 AveragedCollection 实例的唯一方式。
    //当使用 add 方法把一个元素加入到 list 或者使用 remove 方法来删除时，这些方法的实现同时会调用私有的 update_average 方法来更新 average 字段。
    //list 和 average 是私有的，所以没有其他方式来使得外部的代码直接向 list 增加或者删除元素，否则 list 改变时可能会导致 average 字段不同步。
    //average 方法返回 average 字段的值，这使得外部的代码只能读取 average 而不能修改它。
    //因为我们已经封装好了 AveragedCollection 的实现细节，将来可以轻松改变类似数据结构这些方面的内容。例如，可以使用 HashSet<i32> 代替 Vec<i32> 作为 list 字段的类型。
    //只要 add、remove 和 average 公有函数的签名保持不变，使用 AveragedCollection 的代码就无需改变。
    //相反如果使得 list 为公有，就未必都会如此了： HashSet<i32> 和 Vec<i32> 使用不同的方法增加或移除项，所以如果要想直接修改 list 的话，外部的代码可能不得不做出修改。
    let mut a = AveragedCollection::new(vec![1, 2, 3]);
    // a.average = 100.0;报错因为是私有的
    // a.list.push(1);
    // println!("{}", a.average);
    println!("{:?}", a);

    //继承，作为类型系统与代码共享
    //继承（Inheritance）是一个很多编程语言都提供的机制，一个对象可以定义为继承另一个对象的定义，这使其可以获得父对象的数据和行为，而无需重新定义。
    //如果一个语言必须有继承才能被称为面向对象语言的话，那么 Rust 就不是面向对象的。无法定义一个结构体继承父结构体的成员和方法。
    //然而，如果你过去常常在你的编程工具箱使用继承，根据你最初考虑继承的原因，Rust 也提供了其他的解决方案。
    //选择继承有两个主要的原因。第一个是为了重用代码：一旦为一个类型实现了特定行为，继承可以对一个不同的类型重用这个实现。相反 Rust 代码可以使用默认 trait 方法实现来进行共享，
    //我们见过在 Summary trait 上增加的 summarize 方法的默认实现。任何实现了 Summary trait 的类型都可以使用 summarize 方法而无须进一步实现。第十章的例子
    //这类似于父类有一个方法的实现，而通过继承子类也拥有这个方法的实现。当实现 Summary trait 时也可以选择覆盖 summarize 的默认实现，这类似于子类覆盖从父类继承的方法实现。
    //第二个使用继承的原因与类型系统有关：表现为子类型可以用于父类型被使用的地方。这也被称为 多态（polymorphism），这意味着如果多种对象共享特定的属性，则可以相互替代使用。

    //很多人将多态描述为继承的同义词。不过它是一个有关可以用于多种类型的代码的更广泛的概念。对于继承来说，这些类型通常是子类。
    // Rust 则通过泛型来对不同的可能类型进行抽象，并通过 trait bounds 对这些类型所必须提供的内容施加约束。这有时被称为 bounded parametric polymorphism。
    //近来继承作为一种语言设计的解决方案在很多语言中失宠了，因为其时常带有共享多于所需的代码的风险。子类不应总是共享其父类的所有特征，但是继承却始终如此。
    //如此会使程序设计更为不灵活，并引入无意义的子类方法调用，或由于方法实际并不适用于子类而造成错误的可能性。某些语言还只允许子类继承一个父类，进一步限制了程序设计的灵活性。
    //因为这些原因，Rust 选择了一个不同的途径，使用 trait 对象而不是继承。让我们看一下 Rust 中的 trait 对象是如何实现多态的。
}
