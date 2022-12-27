/*
 * @Author: wulongjiang
 * @Date: 2022-12-26 21:22:39
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-27 08:53:56
 * @Description: 线程池库
 * @FilePath: \multithreaded\src\lib.rs
 */
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
pub struct ThreadPool {
    //spawn 返回 JoinHandle<T>，其中 T 是闭包返回的类型。尝试使用 JoinHandle 来看看会发生什么。
    //在我们的情况中，传递给线程池的闭包会处理链接并不返回任何值，所以T将会时单元类型()。
    //改变了 ThreadPool 的定义来存放一个 thread::JoinHandle<()> 的 vector 实例
    // threads: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}
//首先，让我们做出如此创建 ThreadPool 时所需的修改。
// 定义 Worker 结构体存放 id 和 JoinHandle<()>
// 修改 ThreadPool 存放一个 Worker 实例的 vector
// 定义 Worker::new 函数，它获取一个 id 数字并返回一个带有 id 和用空闭包分配的线程的 Worker 实例
// 在 ThreadPool::new 中，使用 for 循环计数生成 id，使用这个 id 新建 Worker，并储存进 vector 中
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

//信道将发送这个枚举的两个成员之一而不是 Job 实例 为了停机
enum Message {
    NewJob(Job),
    Terminate,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        Worker {
            id,
            thread: Some(thread::spawn(move || loop {
                // let job = receiver.lock().unwrap().recv().unwrap();
                //这里，首先在 receiver 上调用了 lock 来获取互斥器，接着 unwrap 在出现任何错误时 panic。
                //如果互斥器处于一种叫做 被污染（poisoned）的状态时获取锁可能会失败，这可能发生于其他线程在持有锁时 panic 了且没有释放锁.
                //在这种情况下，调用 unwrap 使其 panic 是正确的行为。请随意将 unwrap 改为包含有意义错误信息的 expect。
                //如果锁定了互斥器，接着调用 recv 从信道中接收 Job。
                //最后的 unwrap 也绕过了一些错误，这可能发生于持有信道发送端的线程停止的情况，类似于如果接收端关闭时 send 方法如何返回 Err 一样。
                //调用 recv 会阻塞当前线程，所以如果还没有任务，其会等待直到有可用的任务。Mutex<T> 确保一次只有一个 Worker 线程尝试请求任务。
                // println!("Worker {} got a job; executing.", id);
                // job();
                //成功了！现在我们有了一个可以异步执行连接的线程池！它绝不会创建超过四个线程，所以当 server 收到大量请求时系统也不会负担过重。
                //如果请求 /sleep，server 也能够通过另外一个线程处理其他请求。

                //注意如果同时在多个浏览器窗口打开 /sleep，它们可能会彼此间隔地加载 5 秒
                //，因为一些浏览器处于缓存的原因会顺序执行相同请求的多个实例。这些限制并不是由于我们的 web server 造成的。

                //
                let messgae = receiver.lock().unwrap().recv().unwrap();

                match messgae {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job();
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            })),
        }
    }
}

//使用信道向线程发送请求
// 下一个需要解决的问题是传递给 thread::spawn 的闭包完全没有做任何工作。
// 目前，我们在 execute 方法中获得期望执行的闭包，不过在创建 ThreadPool 的过程中创建每一个 Worker 时需要向 thread::spawn 传递一个闭包。
// 我们希望刚创建的 Worker 结构体能够从 ThreadPool 的队列中获取需要执行的代码，并发送到线程中执行他们。
// 在第十六章，我们学习了 信道 —— 一个沟通两个线程的简单手段 —— 对于这个例子来说则是绝佳的。
// 这里信道将充当任务队列的作用，execute 将通过 ThreadPool 向其中线程正在寻找工作的 Worker 实例发送任务。如下是这个计划：
// ThreadPool 会创建一个信道并充当发送端。
// 每个 Worker 将会充当信道的接收端。
// 新建一个 Job 结构体来存放用于向信道中发送的闭包。
// execute 方法会在信道发送端发出期望执行的任务。
// 在线程中，Worker 会遍历信道的接收端并执行任何接收到的任务。
// Job 将是一个有着 execute 接收到的闭包类型的 trait 对象的类型别名。
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    //在 new 中验证池中线程数量
    /// 创建线程池
    ///
    /// 线程池中线程的数量。
    ///
    /// # Panics
    ///
    ///  `new`函数会在size 为 0 时 panic
    pub fn new(size: usize) -> ThreadPool {
        //以考虑 new 作为开始。之前选择使用无符号类型作为 size 参数的类型，因为线程数为负的线程池没有意义。
        //然而，线程数为零的线程池同样没有意义，不过零是一个完全有效的 u32 值。
        //让我们增加在返回 ThreadPool 实例之前检查 size 是否大于零的代码，并使用 assert! 宏在得到零时 panic
        assert!(size > 0);
        //使用 size 容量来初始化，并设置一个 for 循环了来运行创建线程的代码，并返回包含这些线程的 ThreadPool 实例
        let mut workers = Vec::with_capacity(size);
        //with_capacity ,它与Vec::new 做了同样的工作，不过有一个重要的区别：它为vector 预先分配空间。
        //因为已经知道了vector中需要size个元素，预先进行分配比仅仅Vec::new 要稍微有效率一点，因为Vec::new 随着插入元素而重新改变大小。

        let (sender, reciver) = mpsc::channel(); //新建了一个信道，并接着让线程池在接收端等待。
        let reciver = Arc::new(Mutex::new(reciver));
        for id in 0..size {
            // create some threads and store them in the vector
            //如何实际创建线程呢？这是一个难题。标准库提供的创建线程的方法，thread::spawn,它期望获取一些一旦创建线程就应该执行的代码。
            //然而我们希望开始线程并使其等待稍后传递的代码。标准库的线程实现并没有包含这么做的方法；我们必须自己实现。
            //我们将要实现的行为是创建线程并稍后发送代码，这会在 ThreadPool 和线程间引入一个新数据类型来管理这种新行为。
            //这个数据结构称为 Worker：这是一个池实现中的常见概念。想象一下在餐馆厨房工作的员工：员工等待来自客户的订单，他们负责接受这些订单并完成它们。
            //不同于在线程池中储存一个 JoinHandle<()> 实例的 vector，我们会储存 Worker 结构体的实例。每一个 Worker 会储存一个单独的 JoinHandle<()> 实例。
            //接着会在Worker上实现一个方法，它会获取需要允许代码的闭包并将其发送给已经允许的线程执行。
            //我们还会赋予每一个 worker id，这样就可以在日志和调试中区别线程池中的不同 worker。
            //这段代码尝试将 receiver 传递给多个 Worker 实例。这是不行的，回忆第十六章：Rust 所提供的信道实现是多 生产者，单 消费者 的。
            //这意味着不能简单的克隆信道的消费端来解决问题。即便可以，那也不是我们希望使用的技术；我们希望通过在所有的 worker 中共享单一 receiver，在线程间分发任务。
            //另外，从信道队列中取出任务涉及到修改 receiver，所以这些线程需要一个能安全的共享和修改 receiver 的方式，否则可能导致竞争状态（参考第十六章）。
            // workers.push(Worker::new(id, reciver));
            //回忆一下第十六章讨论的线程安全智能指针，为了在多个线程间共享所有权并允许线程修改其值，需要使用 Arc<Mutex<T>>。
            //Arc 使得多个 worker 拥有接收端，而 Mutex 则确保一次只有一个 worker 能从接收端得到任务。
            workers.push(Worker::new(id, Arc::clone(&reciver)));
            //在 ThreadPool::new 中，将信道的接收端放入一个 Arc 和一个 Mutex 中。对于每一个新 worker，克隆 Arc 来增加引用计数，如此这些 worker 就可以共享接收端的所有权了。
        }

        ThreadPool { workers, sender }
    }
    //在 ThreadPool 上定义 execute 函数来获取一个闭包参数。
    //回忆第十三章的 “使用带有泛型和 Fn trait 的闭包” 部分，闭包作为参数时可以使用三个不同的 trait：Fn、FnMut 和 FnOnce。
    //我们需要决定这里应该使用哪种闭包。最终需要实现的类似于标准库的 thread::spawn，所以我们可以观察 thread::spawn 的签名在其参数中使用了何种 bound。
    // pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    // where
    //     F: FnOnce() -> T,
    //     F: Send + 'static,
    //     T: Send + 'static,
    //F 是这里我们关心的参数；T 与返回值有关所以我们并不关心。
    //考虑到 spawn 使用 FnOnce 作为 F 的 trait bound，这可能也是我们需要的，因为最终会将传递给 execute 的参数传给 spawn。
    //因为处理请求的线程只会执行闭包一次，这也进一步确认了 FnOnce 是我们需要的 trait，这里符合 FnOnce 中 Once 的意思。
    pub fn execute<F>(&self, f: F)
    where
        //F需要 Send 来将闭包从一个线程转移到另一个线程，而 'static 是因为并不知道线程会执行多久。
        F: FnOnce() + Send + 'static, //FnOnce trait仍然需要后面的(),因为这里的FnOnce代表一个没有参数也没有返回值的闭包。正如函数的定义，返回值类型可以从签名中省略，不过即便没有参数也需要括号。
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
        //在使用 execute 得到的闭包新建 Job 实例之后，将这些任务从信道的发送端发出。
        //这里调用 send 上的 unwrap，因为发送可能会失败，这可能发生于例如停止了所有线程执行的情况，这意味着接收端停止接收新消息了。
        //不过目前我们无法停止线程执行；只要线程池存在他们就会一直执行。使用 unwrap 是因为我们知道失败不可能发生，即便编译器不这么认为。
        //不过到此事情还没有结束！在 worker 中，传递给 thread::spawn 的闭包仍然还只是 引用 了信道的接收端
        //相反我们需要闭包一直循环，向信道的接收端请求任务，并在得到任务时执行他们。
    }
}

//当使用不那么优雅的 ctrl-c 终止主线程时，所有其他线程也会立刻停止，即便它们正处于处理请求的过程中。
//现在我们要为 ThreadPool 实现 Drop trait 对线程池中的每一个线程调用 join，这样这些线程将会执行完他们的请求
//接着会为 ThreadPool 实现一个告诉线程他们应该停止接收新请求并结束的方式。
impl Drop for ThreadPool {
    fn drop(&mut self) {
        //向每个 worker 发送一个 Terminate 消息 如果尝试在同一循环中发送消息并立即 join 线程，则无法保证当前迭代的 worker 是从信道收到终止消息的 worker。
        //为了更好的理解为什么需要两个分开的循环，想象一下只有两个 worker 的场景。如果在一个单独的循环中遍历每个 worker，在第一次迭代中向信道发出终止消息并对第一个 worker 线程调用 join。
        //如果此时第一个 worker 正忙于处理请求，那么第二个 worker 会收到终止消息并停止。我们会一直等待第一个 worker 结束，不过它永远也不会结束因为第二个线程接收了终止消息。死锁！
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            //worker.thread.join().unwrap(); //此时会报错因为join需要的是线程的所有权而不是 可变引用 （所以需要将拿到线程的所有权可以使用Option可以在 Option 上调用 take 方法将值从 Some 成员中移动出来而对 None 成员不做处理。
            //这里遍历线程中的每个workers。 这里使用了&mut 因为self本身是一个可变引用而且也需要能够修改worker
            //对于每一个线程，会打印出说明信息并表明worker正在关闭，接着在 worker 线程上调用 join。如果 join 调用失败，通过 unwrap 使得 panic 并进行不优雅的关闭。

            //如第十七章我们见过的，Option 上的 take 方法会取出 Some 而留下 None。
            //使用if let 解构some并得到了线程，接着在线程上调用join。如果 worker 的线程已然是 None，就知道此时这个 worker 已经清理了其线程所以无需做任何操作。

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
            //向线程发送信息使其停止信号，
            //但是现在代码还不能以我们期待的方式运行，问题是worker中运行闭包的逻辑：调用join并不会关闭线程，因为他们一直loop来寻找任务
            //如果采用这个实现来尝试丢弃ThreadPool，则主线程永远阻塞在等待第一个线程结束上。
            //为了修复这个问题，修改线程既监听是否有 Job 运行也要监听一个应该停止监听并退出无限循环的信号。
        }
    }
}
