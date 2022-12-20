/*
 * @Author: wlj
 * @Date: 2022-12-20 16:03:08
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-20 17:37:31
 * @Description: 引用循环与内存泄漏
 * @see:https://kaisery.github.io/trpl-zh-cn/ch15-06-reference-cycles.html
 */

//引用循环与内存泄漏
//Rust 的内存安全性保证使其难以意外地制造永远也不会被清理的内存（被称为 内存泄漏（memory leak）），但并不是不可能。
//与在编译时拒绝数据竞争不同， Rust 并不保证完全地避免内存泄漏，这意味着内存泄漏在 Rust 被认为是内存安全的。
//这一点可以通过 Rc<T> 和 RefCell<T> 看出：创建引用循环的可能性是存在的。这会造成内存泄漏，因为每一项的引用计数永远也到不了 0，其值也永远不会被丢弃。

//制造引用循环
//让我们看看引用循环是如何发生的以及如何避免它。
//
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>), //RefCell内部可变模式，Rc允许多个元素同时使用同一个值的所有权 我们希望能够修改 Cons 成员所指向的 List。
    Nil,
}

impl List {
    //方便我们再有Cons成员的时候访问其第二项
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil)))); //在a变量中创建了一个Rc<List> 实例来存放初值为 5, Nil 的 List 值。
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a)))); //在变量 b 中创建了存放包含值 10 和指向列表 a 的 List 的另一个 Rc<List> 实例。
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b) //修改 a 使其指向 b 而不是 Nil，这就创建了一个循环。
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    //可以看到将列表a修改为指向b后，a和b中的Rc实例的引用计数都是2。在main结尾，Rust丢弃b，这会让b实例的引用计数从2减1为1。然而，b 并不能被回收，因为
    //它的引用计数是1而不是0.接下来Rust会丢弃a将a的rc实例从2减为1.这个实例也不能被回收，因为b实例任然引用它，所以其引用计数是1.这些列表的内存将永远保持未被回收的状态。
    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail())
}
