/*
 * @Author: wlj
 * @Date: 2022-12-20 14:05:56
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-20 15:59:13
 * @Description: RefCell<T> 和内部可变性模式
 * @see:https://kaisery.github.io/trpl-zh-cn/ch15-05-interior-mutability.html
 */

// let mut a = String::from("aaa");
// let b = &a;
// a.push('2');
// println!("b:{}", b); 报错
//内部可变性（interior mutability） 是Rust中的一个设计模式、它允许你即使在有不可变引用时也可以改变数据，这通常是借用规则所不允许的。
//为了改变数据，该模式在数据结构中使用 unsafe 代码来模糊 Rust 通常的可变性和借用规则。我们还未讲到不安全代码；第十九章会学习它们。

//当可以确保代码在运行时会遵守借用规则，即使编译器不能保证的情况，可以选择使用那些运用内部可变性模式的类型。
//所涉及的 unsafe 代码将被封装进安全的 API 中，而外部类型仍然是不可变的。
//让我们通过遵循内部可变性模式的 RefCell<T> 类型来开始探索。

//通过 RefCell<T> 在运行时检查借用规则
//不同于 Rc<T>，RefCell<T> 代表其数据的唯一的所有权。。那么是什么让 RefCell<T> 不同于像 Box<T> 这样的类型呢？回忆一下第四章所学的借用规则：
//1.在任意给定时刻，只能拥有一个可变引用或任意数量的不可变引用 之一（而不是两者）。
//2.引用必须总是有效的。
//对于引用和 Box<T>，借用规则的不可变性作用于编译时。
//对于 RefCell<T>，这些不可变性作用于 运行时。
//对于引用，如果违反这些规则，会得到一个编译错误。而对于 RefCell<T>，如果违反这些规则程序会 panic 并退出。
//在编译时检查借用规则的优势是这些错误将在开发过程的早期被捕获，同时对运行时没有性能影响，因为所有的分析都提前完成了。
//为此，在编译时检查借用规则是大部分情况的最佳选择，这也正是其为何是 Rust 的默认行为。
//相反在运行时检查借用规则的好处则是允许出现特定内存安全的场景，而它们在编译时检查中是不允许的。
//静态分析，正如 Rust 编译器，是天生保守的。但代码的一些属性不可能通过分析代码发现：其中最著名的就是 停机问题（Halting Problem）https://zh.wikipedia.org/wiki/%E5%81%9C%E6%9C%BA%E9%97%AE%E9%A2%98 ，这超出了本书的范畴，不过如果你感兴趣的话这是一个值得研究的有趣主题。
//因为一些分析是不可能的，如果 Rust 编译器不能通过所有权规则编译，它可能会拒绝一个正确的程序；
//从这种角度考虑它是保守的。如果 Rust 接受不正确的程序，那么用户也就不会相信 Rust 所做的保证了。然而，如果 Rust 拒绝正确的程序，虽然会给程序员带来不便，但不会带来灾难。
//RefCell<T> 正是用于当你确信代码遵守借用规则，而编译器不能理解和确定的时候。
//类似于 Rc<T>，RefCell<T> 只能用于单线程场景。如果尝试在多线程上下文中使用RefCell<T>，会得到一个编译错误。第十六章会介绍如何在多线程程序中使用 RefCell<T> 的功能。

//如下为选择 Box<T>，Rc<T> 或 RefCell<T> 的理由：
//Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
//Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；RefCell<T> 允许在运行时执行不可变或可变借用检查。
//因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。
//在不可变值内部改变值就是 内部可变性 模式。让我们看看何时内部可变性是有用的，并讨论这是如何成为可能的。

//内部可变性：不可变值的可变借用
//借用规则的一个推论是当有一个不可变值时，不能可变地借用它。例如，如下代码不能编译
// let x = 5;
// let y = &mut x;
// 然而，特定情况下，令一个值在其方法内部能够修改自身，而在其他代码中仍视为不可变，是很有用的。值方法外部的代码就不能修改其值了。
// RefCell<T> 是一个获得内部可变性的方法。编译器中的借用检查器允许内部可变性并相应地在运行时检查借用规则。如果违反了这些规则，会出现 panic 而不是编译错误。
// 让我们通过一个实际的例子来探索何处可以使用 RefCell<T> 来修改不可变值并看看为何这么做是有意义的。

//内部可变性的用例：mock 对象
//测试替身(test double )是一个通用的编程概念，它代表一个在测试中替代某个类型的类型。mock 对象 是特定类型的测试替身，它们记录测试过程中发生了什么以便可以断言操作是正确的。

//虽然 Rust 中的对象与其他语言中的对象并不是一回事，Rust 也没有像其他语言那样在标准库中内建 mock 对象功能，不过我们确实可以创建一个与 mock 对象有着相同功能的结构体。

//如下是一个我们想要测试的场景：我们在编写一个记录某个值与最大值的差距的库，并根据当前值与最大值的差距来发送消息。例如，这个库可以用于记录用户所允许的 API 调用数量限额。
//该库只提供记录与最大值的差距，以及何种情况发送什么消息的功能。
//用此库的程序则期望提供实际发送消息的机制：程序可以选择记录一条消息、发送 email、发送短信等等。库本身无需知道这些细节；只需实现其提供的 Messenger trait 即可。

//拥有一个方法 send 的 Messenger trait
pub trait Messenger {
    //其获取一个 self 的不可变引用和文本信息。
    fn send(&self, msg: &str);
    //这个 trait 是 mock 对象所需要实现的接口库，这样 mock 就能像一个真正的对象那样使用了。
}

pub struct LimitTracker<'a, T> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

//另一个重要的部分是我们需要测试 LimitTracker 的 set_value 方法的行为。可以改变传递的 value 参数的值，不过 set_value 并没有返回任何可供断言的值。
//也就是说，如果使用某个实现了 Messenger trait 的值和特定的 max 创建 LimitTracker，当传递不同 value 值时，消息发送者应被告知发送合适的消息。
//我们所需的 mock 对象是，调用 send 并不实际发送 email 或消息，而是只记录信息被通知要发送了。
//可以新建一个 mock 对象实例，用其创建 LimitTracker，调用 LimitTracker 的 set_value 方法，然后检查 mock 对象是否有我们期望的消息。

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
                //现在 sent_messages 字段的类型是 RefCell<Vec<String>> 而不是 Vec<String>。在 new 函数中新建了一个 RefCell<Vec<String>> 实例替代空 vector。
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            //对于 send 方法的实现，第一个参数仍为 self 的不可变借用，这是符合方法定义的。
            //我们调用 self.sent_messages 中 RefCell 的 borrow_mut 方法来获取 RefCell 中值的可变引用这是一个 vector。接着可以对 vector 的可变引用调用 push 以便记录测试过程中看到的消息。
            self.sent_messages.borrow_mut().push(String::from(msg)); //报错self.sent_messages.push(String::from(msg))self` is a `&` reference, so the data it refers to cannot be borrowed as mutable因为 send 方法获取了 self 的不可变引用。 所以我们不能更改
                                                                     //我们也不能参考错误文本的建议使用 &mut self 替代，因为这样 send 的签名就不符合 Messenger trait 定义中的签名了（可以试着这么改，看看会出现什么错误信息）。
                                                                     //这正是内部可变性的用武之地！我们将通过 RefCell 来储存 sent_messages，然后 send 将能够修改 sent_messages 并储存消息。

            // let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut();

            // one_borrow.push(String::from(msg));
            // two_borrow.push(String::from(msg));

            //运行的时候会报错  因为就像编译时借用规则一样，RefCell<T> 在任何时候只允许有多个不可变借用或一个可变借用。
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        //最后必须做出的修改位于断言中：为了看到其内部 vector 中有多少个项，需要调用 RefCell 的 borrow 以获取 vector 的不可变引用。
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        //在测试中，我们测试了当 LimitTracker 被告知将 value 设置为超过 max 值 75% 的某个值。首先新建一个 MockMessenger，其从空消息列表开始。
        //接着新建一个 LimitTracker 并传递新建 MockMessenger 的引用和 max 值 100。
        //我们使用值 80 调用 LimitTracker 的 set_value 方法，这超过了 100 的 75%。接着断言 MockMessenger 中记录的消息列表应该有一条消息。

        //现在我们见识了如何使用 RefCell<T>，让我们研究一下它怎样工作的！
        //RefCell<T> 在运行时记录借用
        //当创建不可变和可变引用时，我们分别使用 & 和 &mut 语法。对于 RefCell<T> 来说，则是 borrow 和 borrow_mut 方法，这属于 RefCell<T> 安全 API 的一部分。
        //borrow 方法返回 Ref<T> 类型的智能指针，borrow_mut 方法返回 RefMut<T> 类型的智能指针。这两个类型都实现了 Deref，所以可以当作常规引用对待。

        //RefCell<T> 记录当前有多少个活动的 Ref<T> 和 RefMut<T> 智能指针。每次调用 borrow，RefCell<T> 将活动的不可变借用计数加一。当 Ref<T> 值离开作用域时，不可变借用计数减一。
        //就像编译时借用规则一样，RefCell<T> 在任何时候只允许有多个不可变借用或一个可变借用。
        //如果我们尝试违反这些规则，相比引用时的编译时错误，RefCell<T> 的实现会在运行时出现 panic。
        //在运行时捕获借用错误而不是编译时意味着将会在开发过程的后期才会发现错误，甚至有可能发布到生产环境才发现；还会因为在运行时而不是编译时记录借用而导致少量的运行时性能惩罚。
        //然而，使用 RefCell 使得在只允许不可变值的上下文中编写修改自身以记录消息的 mock 对象成为可能。虽然有取舍，但是我们可以选择使用 RefCell<T> 来获得比常规引用所能提供的更多的功能。

        //结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者
        #[derive(Debug)]
        enum List {
            Cons(Rc<RefCell<i32>>, Rc<List>),
            Nil,
        }

        use std::cell::RefCell;
        use std::rc::Rc;
        use List::{Cons, Nil};

        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
        *value.borrow_mut() += 10;// value.borrow_mut()获取可变引用 然后再解引用

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
        //太乱了。。。。、
        //这里创建了一个Rc<RefCell<i32>>实例并存储在变量value中以便之后直接访问。接着在a中用包含value的Cons船舰了一个List。
        // 需要克隆value一遍 a 和 value 都能拥有其内部值5的所有权。而不是将所有权从value移动到a或者让a借用value。a
        // 我们将列表 a 封装进了 Rc<T> 这样当创建列表 b 和 c 时，他们都可以引用 a，

        //标准库中也有其他提供内部可变性的类型，比如 Cell<T>，它类似 RefCell<T> 但有一点除外：它并非提供内部值的引用，而是把值拷贝进和拷贝出 Cell<T>。还有 Mutex<T>，其提供线程间安全的内部可变性，我们将在第 16 章中讨论其用法。
    }
}
