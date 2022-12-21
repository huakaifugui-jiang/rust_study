/*
 * @Author: wlj
 * @Date: 2022-12-21 09:33:31
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-21 10:54:20
 * @Description: 使用线程同时运行代码
 * @see:https://kaisery.github.io/trpl-zh-cn/ch16-01-threads.html
 */

//在大部分现代操作系统中，已执行程序的代码在一个进程（process）中运行，操作系统负责管理多个进程。在程序内部，也可以拥有多个同时运行的独立部分。运行这些独立部分的功能被称为线程
//将程序中的计算拆分为多个线程可以改善性能，因为程序可以同时进行多个任务，不过这也会增加复杂性。因为线程是同时运行的，所以无法预先保证不同线程中代码的执行顺序。这会导致诸如此类的问题：

//竞态条件（Race conditions），多个线程以不一致的顺序访问数据或资源
//死锁（Deadlocks），两个线程互相等待对方停止使用其所拥有的资源，这会阻止它们继续运行
//只会发生在特定情况且难以稳定重现和修复的bug

//Rust 尝试减轻使用线程的负面影响。不过在多线程上下文中编程仍需格外小心，同时其所要求的代码结构也不同于运行于单线程的程序。
//编程语言有一些不同的方法来实现线程。很多操作系统提供了创建新线程的 API。
//这种由编程语言调用操作系统 API 创建线程的模型有时被称为 1:1，一个 OS 线程对应一个语言线程。
//Rust 标准库只提供了 1:1 线程实现；有一些 crate 实现了其他有着不同取舍的线程模型。

use std::{thread, time::Duration, vec};
//使用 spawn（产卵） 创建新线程
fn use_spawn_create_threads() {
    //为了创建一个新线程，需要调用thread::spawn函数并传递一个闭包（第十三章学习了闭包），并在其中包含希望在新线程中运行的代码。
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         //spawned 线程 新开辟的线程
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1)); //调用强制线程停止执行一小段时间，这会允许不同的线程运行。这些线程可能会轮流运行
    //     }
    // });

    // for i in 1..5 {
    //     //主线程
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    //注意这个函数编写的方式，当主线程结束时，新线程也会结束，而不管其是否执行完毕。这个程序的输出可能每次都略有不同，不过它大体上看起来像这样：（自己运行看看）
    //sleep调用强制线程停止执行一小段时间，这会允许其他不同的线程运行。这些线程可能会轮流运行，不过并不保证如此：这依赖于操作系统如何调度线程。在这里，主线程首先打印
    //即便创建线程的打印语句位于程序的开头，甚至即使我们告诉新建的线程打印知道i 等于9，它在主线程结束之前也就只打印到了5。

    //使用 join 等待所有线程结束
    //由于主线程结束 示例中的代码大部分时候不光会提早结束新建线程，甚至不能实际保证新建线程会被执行。其原因在于无法保证线程运行的顺序！
    //可以通过将 thread::spawn 的返回值储存在变量中来修复新建线程部分没有执行或者完全没有执行的问题。
    //thread::spawn 的返回值类型是 JoinHandle。JoinHandle 是一个拥有所有权的值，当对其调用 join 方法时，它会等待其线程结束。

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    //  handle.join().unwrap();会阻塞for循环
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap(); //通过调用handle的join 会阻塞当前线程(main)直到handle所代表的线程结束。阻塞（Blocking）线程意味着阻止该线程执行工作或退出
                            //因为我们将join放在了主线程的for循环后，所以不会对for循环进行阻塞
}

//线程与move闭包
//move 关键字经常用于传递给 thread::spawn的闭包，因为闭包会获取从环境中取得的值的所有权，因此会将这些值的所有权从一个线程传递到另一个线程。
//在第十三章 “闭包会捕获其环境” 部分讨论了闭包上下文中的 move。现在我们会更专注于 move 和 thread::spawn 之间的交互。
fn move_keyword_in_threads() {
    // let x = vec![1, 2, 3];

    // let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x);//报错 因为使用了move关键字被移动到闭包里面了 x的所有权

    // let y = vec![1, 2, 3];

    // assert!(equal_to_x(y));
    //在第十三章中，我们讲到可以在参数列表前使用 move 关键字强制闭包获取其使用的环境值的所有权。这个技巧在创建新线程将值的所有权从一个线程移动到另一个线程时最为实用。
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        //所以需要添加move关键字 将v的所有权移动进来
        //报错 may outlive borrowed value `v`
        println!("Here's a vector: {:?}", v); //如果使用了外部的变量 报错- `v` is borrowed here
                                              //Rust会推断 如何捕获v，因为println 只需要v的引用，闭包尝试借用v。然而这有一个问题：Rust不知道这个新建线程会执行多久，所以无法知晓v的引用是否一直有效。
    });

    //drop(v); //回收v 新建线程内部有一个 v 的引用，不过主线程立刻就使用第十五章讨论的 drop 丢弃了 v。接着当新建线程开始执行，v 已不再有效，所以其引用也是无效的。
    handle.join().unwrap();
}

fn main() {
    use_spawn_create_threads();
    move_keyword_in_threads();
}
