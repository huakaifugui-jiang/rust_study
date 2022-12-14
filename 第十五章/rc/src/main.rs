/*
 * @Author: wlj
 * @Date: 2022-12-20 10:31:43
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-20 11:31:36
 * @Description: Rc<T> 引用计数智能指针
 * @see:https://kaisery.github.io/trpl-zh-cn/ch15-04-rc.html
 */

//Rc<T> 引用计数智能指针
//大部分情况下所有权是非常明确的：可以准确地知道哪个变量拥有某个值。然而，有些情况单个值可能会有多个所有者。
//例如，在图数据结构中，多个边可能指向相同的节点，而这个节点从概念上讲为所有指向它的边所拥有。节点直到没有任何边指向它之前都不应该被清理。
//为了启用多所有权，Rust 有一个叫做 Rc<T> 的类型。其名称为引用计数(reference counting)的缩写。
//引用计数意味着记录一个值引用的数量来知晓这个值是否仍在被使用。如果某个值有零个引用，就代表没有任何有效引用并可以被清理。
//可以将其想象为客厅中的电视。当一个人进来看电视时，他打开电视。其他人也可以进来看电视。当最后一个人离开房间时，他关掉电视因为它不再被使用了。
//如果某人在其他人还在看的时候就关掉了电视，正在看电视的人肯定会抓狂的！

//Rc<T> 用于当我们希望在堆上分配一些内存供程序的多个部分读取，而且无法在编译时确定程序的哪一部分会最后结束使用它的时候。
//如果确实知道哪部分是最后一个结束使用的话，就可以令其成为数据的所有者，正常的所有权规则就可以在编译时生效。
//注意 Rc<T> 只能用于单线程场景；第十六章并发会涉及到如何在多线程程序中进行引用计数。

//使用 Rc<T> 共享数据
//让我们回到示例 中使用 Box<T> 定义 cons list 的例子。这一次，我们希望创建两个共享第三个列表所有权的列表

//需要使用 use 语句将 Rc<T> 引入作用域，因为它不在 prelude 中。
use std::rc::Rc;
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use crate::List::{Cons, Nil};

fn main() {
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));//被共享的数据
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));//报错 因为a的所有权已经被移动move了 如何解决呢？

    //我们修改 List 的定义为使用 Rc<T> 代替 Box<T>,现在每一个 Cons 变量都包含一个值和一个指向 List 的 Rc<T>
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil))))); //在 main 中创建了存放 5 和 10 的列表并将其存放在 a 的新的 Rc<List> 中
    // let b = Cons(3, Rc::clone(&a)); //当创建 b 时，不同于获取 a 的所有权，这里会克隆 a 所包含的 Rc<List>,这会将引用计数从 1 增加到 2 并允许 a 和 b 共享 Rc<List> 中数据的所有权
    // let c = Cons(4, Rc::clone(&a)); //创建 c 时也会克隆 a，这会将引用计数从 2 增加为 3
    //每次调用 Rc::clone，Rc<List> 中数据的引用计数都会增加，直到有零个引用之前其数据都不会被清理。
    //也可以调用 a.clone() 而不是 Rc::clone(&a)，不过在这里 Rust 的习惯是使用 Rc::clone。
    //Rc::clone 的实现并不像大部分类型的 clone 实现那样对所有数据进行深拷贝。Rc::clone 只会增加引用计数，这并不会花费多少时间。
    //深拷贝可能会花费很长时间。通过使用 Rc::clone 进行引用计数，可以明显的区别深拷贝类的克隆和增加引用计数类的克隆。
    //当查找代码中的性能问题时，只需考虑深拷贝类的克隆而无需考虑 Rc::clone 调用。

    //克隆 Rc<T> 会增加引用计数
    //让我们修改示例 的代码以便观察创建和丢弃 a 中 Rc<List> 的引用时引用计数的变化。
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));//Rc::strong_count 函数 可以拿到a的引用计数 这个函数叫做 strong_count 而不是 count 是因为 Rc<T> 也有 weak_count
    //在 “避免引用循环：将 Rc<T> 变为 Weak<T>” 部分会讲解 weak_count 的用途。
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    //我们能够看到 a 中 Rc<List> 的初始引用计数为1，接着每次调用 clone，计数会增加1。当 c 离开作用域时，计数减1。不必像调用 Rc::clone 增加引用计数那样调用一个函数来减少计数；
    //Drop trait 的实现当 Rc<T> 值离开作用域时自动减少引用计数。
    //从这个例子我们所不能看到的是，在 main 的结尾当 b 然后是 a 离开作用域时，此处计数会是 0，同时 Rc<List> 被完全清理。
    //使用 Rc<T> 允许一个值有多个所有者，引用计数则确保只要任何所有者依然存在其值也保持有效。
    //通过不可变引用， Rc<T> 允许在程序的多个部分之间只读地共享数据。
    //如果 Rc<T> 也允许多个可变引用，则会违反第四章讨论的借用规则之一:相同位置的多个可变借用可能造成数据竞争和不一致。
    //不过可以修改数据是非常有用的！在下一部分，我们将讨论内部可变性模式和 RefCell<T> 类型，它可以与 Rc<T> 结合使用来处理不可变性的限制。
}
