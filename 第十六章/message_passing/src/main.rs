/*
 * @Author: wlj
 * @Date: 2022-12-21 10:56:33
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-21 14:34:28
 * @Description: 使用消息传递在线程间传送数据
 * @see:https://kaisery.github.io/trpl-zh-cn/ch16-02-message-passing.html#%E4%BD%BF%E7%94%A8%E6%B6%88%E6%81%AF%E4%BC%A0%E9%80%92%E5%9C%A8%E7%BA%BF%E7%A8%8B%E9%97%B4%E4%BC%A0%E9%80%81%E6%95%B0%E6%8D%AE
 */

//使用消息传递在线程间传送数据

//一个日益流行的确保安全并发的方式是消息传递(message passing),这里 线程 或 actor 通过发送包含数据的消息来互相沟通。这个思想来源于 Go编程语言文档中的口号：“不要通过共享内存来通讯”
//而是通过通讯来共享内存（Do not commitnicate by sharing memory;instead,share memory by communicating）
//Rust 中实现消息传递并发的主要工具是信道(channel)，Rust标准库提供了其实现的编程概念。
//你可以将其想象为一个水流的渠道，比如河流或小溪。如果你将诸如橡皮鸭或小船之类的东西放入其中，它们会顺流而下到达下游。

//编程中的信息渠道(信道)有两部分组成，一个发送者(transmitter) 和一个接收者(receiver)。发送者位于上游位置，在这里可以将橡皮鸭放入河中，接收者则位于下游，橡皮鸭最终会漂流至此。
//代码中的一部分 调用发送者的方法以及希望发送的数据，另一部分则检查接收端收到的消息。当发送者或接收者任一被丢弃时可以认为信道被关闭了。

//这里，我们将开发一个程序，它会在一个线程生成值向信道发送，而在另一个线程会接收值并打印出来。这里会通过信道在线程间发送简单值来演示这个功能。
//一旦你熟悉了这项技术，就能使用信道来实现聊天系统,或利用很多线程进行分布式计算并将部分计算结果发送给一个线程进行聚合。
use std::time::Duration;
use std::{sync::mpsc, thread}; //mpsc是多个生产者，单个消费者(multiple producer,single consumer)的缩写。
                               //简而言之Rust标准库实现信道的方式意味着一个信道可以有多个产生值的发送(sending)端,但只能由一个消费这些值的接收(receiving)端
                               //想象一下 多条小河小溪最终汇聚成大河：所有通过这些小河发出的东西最后都会来到下游的大河。目前我们从单个生产者开始，但是当示例可以工作后会增加多个生产者。

fn learn_message_passing() {
    let (tx, rx) = mpsc::channel(); //函数返回一个元组：第一个元素是发送端，第二个元素是接收端。由于历史原因,tx和rx通常作为
                                    //发送者(transmitter)和接收者(receiver)的缩写，所以这就是我们将用来绑定这两端变量的名字。这里使用了一个 let 语句和模式来解构了此元组；

    //将发送端移动到一个新建线程中并发送一个字符串，这样新建线程就可以和主线程通讯了，如下，这类似于在河的上游扔下一只橡皮鸭或从一个线程向另一个线程发送聊天信息：
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); //send方法用来获取需要放入信道的值。send方法返回一个Result类型，所以如果接收端已经被丢弃了，将没有发送值的目标，所以发送操作会返回错误。
                               //在这个例子中，出错的时候调用 unwrap 产生 panic。不过对于一个真实程序，需要合理地处理它：回到第九章复习正确处理错误的策略。

        //所有权规则在消息传递中扮演了重要角色，其有助于我们编写安全的并发代码。防止并发编程中的错误是在Rust程序中考虑所有权的一大优势
        // println!("val is {}", val);//send 函数获取其参数的所有权并移动这个值归接收者所有
    });

    //在主线程中从信道的接收端获取值。这类似于在河的下游捞起橡皮鸭或接收聊天信息
    let received = rx.recv().unwrap(); //信道的接收端有两个有用的方法：recv和try_recv。这里，我们使用了recv，它是receive的缩写。这个方法会阻塞主线程执行直到从信道中
                                       //接收一个值。一旦发送了一个值，recv 会在一个 Result<T, E> 中返回它。当信道发送端关闭，recv 会返回一个错误表明不会再有新的值到来了。
                                       //try_recv 不会阻塞，相反它立刻返回一个 Result<T, E>：Ok 值包含可用的信息，而 Err 值代表此时没有任何消息。
                                       // 如果线程在等待消息过程中还有其他工作时使用 try_recv 很有用：可以编写一个循环来频繁调用 try_recv，在有可用消息时进行处理，其余时候则处理一会其他工作直到再次检查。
                                       //出于简单的考虑，这个例子使用了 recv；主线程中除了等待消息之外没有任何其他工作，所以阻塞主线程是合适的。
    println!("Got: {}", received);
}
fn main() {
    learn_message_passing();

    //发送多个值并观察接收者的等待
    let (tx, rx) = mpsc::channel();
    // 通过克隆发送者来创建多个生产者
    let tx1 = tx.clone(); //这一次，在创建新线程之前，我们对信道的发送端调用了 clone 方法。这会给我们一个可以传递给第一个新建线程的发送端句柄。

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1)); //遍历他们，单独的发送每一个字符串，后暂停1s
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //主线程中，不再显式调用 recv 函数：而是将 rx 当作一个迭代器。对于每一个接收到的值，我们将其打印出来。当信道被关闭时，迭代器也将结束。
    for received in rx {
        println!("Got vals: {}", received);
        //因为主线程中的 for 循环里并没有任何暂停或等待的代码，所以可以说主线程是在等待从新建线程中接收值。
    }
}
