/*
 * @Author: wlj
 * @Date: 2022-12-28 08:11:00
 * @LastEditors: wulongjiang
 * @LastEditTime: 2022-12-29 01:56:38
 * @Description: 底层探秘: Future 执行器与任务调度
 * @see:https://course.rs/async-rust/async/future-excuting.htmlss
 */

/// 异步编程背后到底藏有什么秘密？究竟是哪只幕后之手在操纵这一切？如果你对这些感兴趣，就继续看下去，否则可以直接跳过，因为本章节的内容对于一个 API 工程师并没有太多帮助。
/// 但是如果你希望能深入理解 Rust 的 async/.await 代码是如何工作、理解运行时和性能，甚至未来想要构建自己的 async 运行时或相关工具，那么本章节终究不会辜负于你。
/// # Future 特征
/// Future 特征是 Rust 异步编程的核心，毕竟异步函数是异步编程的核心，而 Future 恰恰是异步函数的返回值和被执行的关键。
///
/// 首先，来给出 Future 的定义：它是一个能产出值的异步计算(虽然该值可能为空，例如 () )。光看这个定义，可能会觉得很空洞，我们来看看一个简化版的 Future 特征:
/// ```
/// trait SimpleFuture {
/// type Output;
/// fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
/// }
/// enum Poll<T> {
/// Ready(T),
/// Pending,
/// }
/// ```
///我们提到过 Future 需要被执行器poll(轮询)后才能运行，诺，这里 poll 就来了，通过调用该方法，可以推进 Future 的进一步执行，直到被切走为止。
///这里不好理解，但是你只需要知道 Future 并不能保证在一次 poll 中就被执行完，后面会详解介绍
///
/// 若在当前 poll 中， Future 可以被完成，则会返回 Poll::Ready(result) ，反之则返回 Poll::Pending，
/// 并且安排一个 wake 函数：当未来 Future 准备好进一步执行时， 该函数会被调用，然后管理该 Future 的执行器(例如上一章节中的block_on函数)会再次调用 poll 方法，此时 Future 就可以继续执行了。)
///
/// 如果没有 wake 方法，那执行器无法知道某个Future是否可以继续被执行，除非执行器定期的轮询每一个 Future ，确认它是否能被执行，但这种作法效率较低。
/// 而有了 wake，Future 就可以主动通知执行器，然后执行器就可以精确的执行该 Future。这种“事件通知 -> 执行”的方式要远比定期对所有 Future 进行一次全遍历来的高效。
///
/// 也许大家还是迷迷糊糊的，没事，我们用一个例子来说明下。考虑一个需要从 socket 读取数据的场景：
/// 如果有数据，可以直接读取数据并返回 Poll::Ready(data)，  但如果没有数据，Future 会被阻塞且不会再继续执行，此时它会注册一个 wake 函数，
/// 当 socket 数据准备好时，该函数将被调用以通知执行器：我们的 Future 已经准备好了，可以继续执行。
// pub struct SocketRead<'a> {
//     socket: &'a Socket,
// }

// impl SimpleFuture for SocketRead<'_> {
//     type Output = Vec<u8>;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

use futures::future::{self, BoxFuture};

//     fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
//         if self.socket.has_data_to_read() {
//             // socket有数据，写入buffer中并返回
//             Poll::Ready(self.socket.read_buf())
//         } else {
//             // socket中还没数据
//             //
//             // 注册一个`wake`函数，当数据可用时，该函数会被调用，
//             // 然后当前Future的执行器会再次调用`poll`方法，此时就可以读取到数据
//             self.socket.set_readable_callback(wake);
//             Poll::Pending
//         }
//     }
// }
fn main() {
    //这种 Future 模型允许将多个异步操作组合在一起，同时还无需任何内存分配。不仅仅如此，如果你需要同时运行多个 Future或链式调用多个 Future ，也可以通过无内存分配的状态机实现，例如：
    // 一个SimpleFuture，它会并发地运行两个Future直到它们完成
    //
    // 之所以可以并发，是因为两个Future的轮询可以交替进行，一个阻塞，另一个就可以立刻执行，反之亦然
    // trait SimpleFuture {
    //     type Output;
    //     fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
    // }

    // enum Poll<T> {
    //     Ready(T),
    //     Pending,
    // }

    // pub struct Join<FutureA, FutureB> {
    //     // 结构体的每个字段都包含一个Future，可以运行直到完成.
    //     // 如果Future完成后，字段会被设置为 `None`. 这样Future完成后，就不会再被轮询
    //     a: Option<FutureA>,
    //     b: Option<FutureB>,
    // }

    // impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
    // where
    //     FutureA: SimpleFuture<Output = ()>,
    //     FutureB: SimpleFuture<Output = ()>,
    // {
    //     type Output = ();
    //     fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
    //         // 尝试去完成一个 Future `a`
    //         if let Some(a) = &mut self.a {
    //             if let Poll::Ready(()) = a.poll(wake) {
    //                 self.a.take();
    //             }
    //         }

    //         // 尝试去完成一个 Future `b`
    //         if let Some(b) = &mut self.b {
    //             if let Poll::Ready(()) = b.poll(wake) {
    //                 self.b.take();
    //             }
    //         }
    //         if self.a.is_none() && self.b.is_none() {
    //             // 两个 Future都已完成 - 我们可以成功地返回了
    //             Poll::Ready(())
    //         } else {
    //             // 至少还有一个 Future 没有完成任务，因此返回 `Poll::Pending`.
    //             // 当该 Future 再次准备好时，通过调用`wake()`函数来继续执行
    //             Poll::Pending
    //         }
    //     }
    // }
    //上面代码展示了如何同时运行多个 Future， 且在此过程中没有任何内存分配，让并发编程更加高效。 类似的，多个Future也可以一个接一个的连续运行：
    // trait SimpleFuture {
    //     type Output;
    //     fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
    // }

    // enum Poll<T> {
    //     Ready(T),
    //     Pending,
    // }

    // /// 一个SimpleFuture，它会并发地运行两个Future直到它们完成
    // ///
    // /// 之所以可以并发，是因为两个Future的轮询可以交替进行，一个阻塞，另一个就可以立刻执行，反之亦然
    // pub struct Join<FutureA, FutureB> {
    //     // 结构体的每个字段都包含一个Future，可以运行直到完成.
    //     // 如果Future完成后，字段会被设置为 `None`. 这样Future完成后，就不会再被轮询
    //     a: Option<FutureA>,
    //     b: Option<FutureB>,
    // }

    // impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
    // where
    //     FutureA: SimpleFuture<Output = ()>,
    //     FutureB: SimpleFuture<Output = ()>,
    // {
    //     type Output = ();
    //     fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
    //         // 尝试去完成一个 Future `a`
    //         if let Some(a) = &mut self.a {
    //             if let Poll::Ready(()) = a.poll(wake) {
    //                 self.a.take();
    //             }
    //         }

    //         // 尝试去完成一个 Future `b`
    //         if let Some(b) = &mut self.b {
    //             if let Poll::Ready(()) = b.poll(wake) {
    //                 self.b.take();
    //             }
    //         }

    //         if self.a.is_none() && self.b.is_none() {
    //             // 两个 Future都已完成 - 我们可以成功地返回了
    //             Poll::Ready(())
    //         } else {
    //             // 至少还有一个 Future 没有完成任务，因此返回 `Poll::Pending`.
    //             // 当该 Future 再次准备好时，通过调用`wake()`函数来继续执行
    //             Poll::Pending
    //         }
    //     }
    // }
    // 这些例子展示了在不需要内存对象分配以及深层嵌套回调的情况下，该如何使用 Future 特征去表达异步控制流。 在了解了基础的控制流后，我们再来看看真实的 Future 特征有何不同之处。
    // trait Future {
    //     type Output;
    //     fn poll(
    //         // 首先值得注意的地方是，`self`的类型从`&mut self`变成了`Pin<&mut Self>`:
    //         self: Pin<&mut Self>,
    //         // 其次将`wake: fn()` 修改为 `cx: &mut Context<'_>`:
    //         cx: &mut Context<'_>,
    //     ) -> Poll<Self::Output>;
    // }
    // 首先这里多了一个 Pin ，关于它我们会在后面章节详细介绍，现在你只需要知道使用它可以创建一个无法被移动的 Future ，因为无法被移动，因此它将具有固定的内存地址，
    // 意味着我们可以存储它的指针(如果内存地址可能会变动，那存储指针地址将毫无意义！)也意味着可以实现一个自引用数据结构: struct MyFut { a: i32, ptr_to_a: *const i32 }。 而对于 async/await 来说，Pin 是不可或缺的关键特性。
    // 其次，从 wake: fn() 变成了 &mut Context<'_> 。意味着 wake 函数可以携带数据了，为何要携带数据？
    // 考虑一个真实世界的场景，一个复杂应用例如 web 服务器可能有数千连接同时在线，那么同时就有数千 Future 在被同时管理着，
    // 如果不能携带数据，当一个 Future 调用 wake 后，执行器该如何知道是哪个 Future 调用了 wake ,然后进一步去 poll 对应的 Future ？没有办法！那之前的例子为啥就可以使用没有携带数据的 wake ？ 因为足够简单，不存在歧义性。
    // 总之，在正式场景要进行 wake ，就必须携带上数据。 而 Context 类型通过提供一个 Waker 类型的值，就可以用来唤醒特定的的任务。

    // 使用 Waker 来唤醒任务
    // 对于 Future 来说，第一次被 poll 时无法完成任务是很正常的。但它需要确保在未来一旦准备好时，可以通知执行器再次对其进行 poll 进而继续往下执行，该通知就是通过 Waker 类型完成的。
    // Waker 提供了一个 wake() 方法可以用于告诉执行器：相关的任务可以被唤醒了，此时执行器就可以对相应的 Future 再次进行 poll 操作。

    // 构建一个定时器
    //下面一起来实现一个简单的定时器 Future 。为了让例子尽量简单，当计时器创建时，我们会启动一个线程接着让该线程进入睡眠，等睡眠结束后再通知给 Future 。
    use std::{
        future::Future,
        pin::Pin,
        sync::{Arc, Mutex},
        task::{Context, Poll, Waker},
        thread,
        time::Duration,
    };

    //继续实现Future定时器，之前提到：新建线程在睡眠结束后悔需要将状态同步给Future，由于是多线程的环境，我们需要一个Arc<Mutex<T>>来作为一个共享状态，用于在新线程和Future定时器间共享

    pub struct TimerFuture {
        shared_state: Arc<Mutex<SharedState>>,
    }

    /// 在Future和等待的线程间共享状态
    struct SharedState {
        ///定时（睡眠）是否结束
        completed: bool,
        /// 当睡眠结束后，线程可以用`waker`通知`TimeFuture`来唤醒任务
        waker: Option<Waker>,
    }

    impl Future for TimerFuture {
        type Output = ();
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            //通过检查共享状态，来确定定时器是否已经完成
            let mut shared_state = self.shared_state.lock().unwrap();
            if shared_state.completed {
                Poll::Ready(())
            } else {
                // 设置`waker`，这样新线程在睡眠（计时）结束后可以唤醒当前任务，接着再次对`Future`进行`poll`操作，
                // 下面的`clone`每次都被 `poll`一次，实际上，应该只是`clone`一次更加合理
                // 每次都选择`clone`的原因是：`TimerFuture` 可以在执行器的不同任务间移动，如果只克隆一次，
                // 那么获取到的`waker`可能已经被篡改并指向了其他任务，最终导致执行器运行了错误的任务
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
        //只要新线程设置了shared_state.completed = true ，那任务就能顺利结束。如果没有设置，会为当前任务克隆一份Waker，这样新线程就可以用它来唤醒当前任务
    }
    //创建一个API用于构建定时器和启动定时器线程：
    impl TimerFuture {
        fn new(duration: Duration) -> Self {
            let shared_state = Arc::new(Mutex::new(SharedState {
                completed: false,
                waker: None,
            }));

            //创建新线程
            let thread_shared_state = shared_state.clone();
            thread::spawn(move || {
                thread::sleep(duration);
                let mut shared_state = thread_shared_state.lock().unwrap();
                //通知定时器已经完成，可以继续`poll`对应的 Future 了
                shared_state.completed = true;
                if let Some(waker) = shared_state.waker.take() {
                    waker.wake()
                }
            });
            TimerFuture { shared_state }
        }
    }
    //最后。我们需要创建一个执行器，才能让程序动起来
    //Rust 的 Future 是惰性的：只有屁股上拍一拍，它才会努力动一动。其中一个推动它的方式就是在 async 函数中使用 .await 来调用另一个 async 函数
    //，但是这个只能解决 async 内部的问题，那么这些最外层的 async 函数，谁来推动它们运行呢？答案就是我们之前多次提到的执行器 executor 。
    //执行器会管理一批 Future (最外层的 async 函数)，然后通过不停地 poll 推动它们直到完成。 最开始，执行器会先poll 一次 Future ，后面就不会主动去 poll 了
    //而是等待 Future 通过调用 wake 函数来通知它可以继续，它才会继续去 poll 。这种wake 通知然后 poll的方式会不断重复，直到 Future 完成。

    //构建执行器
    //执行器需要从一个消息通道（channel）中拉取事件，然后运行它们。当一个任务准备好后（可以继续执行），它会将自己放入消息通道中，然后等待执行器poll；
    //(相当于执行器是消费者)
    //任务执行器，负责从通道中接受任务然后执行
    struct Executor {
        ready_queue: Receiver<Arc<Task>>,
    }

    //spawner 负责创建新的 future 然后将它发送到任务通道中
    struct Spawner {
        task_sender: SyncSender<Arc<Task>>,
    }
    use futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    };
    //用于生成Future，并把它放入任务通道中
    impl Spawner {
        fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
            let future = future.boxed();
            let task = Arc::new(Task{
                future,
                task_sender:self.task_sender.clone()
            });
        }
    }
    //一个Future 它可以调度自己（将自己放入任务通道中）然后等待执行器去poll
    struct Task {
        /// 进行中的Future，在未来的某个时间点会被完成
        /// 按道理来说Mutex 在这边是多余的，因为我们只有一个线程来执行任务。但是由于Rust并不聪明，它无法知道future只会在一个线程内被修改
        /// 并不会垮线程被修改。因此我们需要mutex 来满足这个笨笨的编译器对线程安全的执着
        /// 如果是生产级的执行器实现，不会使用mutex 因为会带来性能上的开销，取而代之的是使用 unsafeCell
        future: BoxFuture<'static, ()>,
        //可以将任务自身放回到任务通道中，等待执行器的poll
        task_sender: SyncSender<Arc<Task>>,
    }
     //在执行器poll一个Future之前，需要调用wake方法进行唤醒，然后再由Waker负责调度该任务并将其放入任务通道中。
    //  创建 Waker 的最简单的方式就是实现 ArcWake 特征，先来为我们的任务实现 ArcWake 特征，这样它们就能被转变成 Waker 然后被唤醒:

    fn new_executor_and_spawner() -> (Executor, Spawner) {
        //任务通道允许的最大缓冲数(任务队列的最大长度)
        // 当前的实现仅仅是为了简单，在实际的执行中，并不会这么使用
        const MAX_QUEUE_TASKS: usize = 10_000;
        let (task_sender, ready_queue) = sync_channel(MAX_QUEUE_TASKS); //同步通道

        (Executor { ready_queue }, Spawner { task_sender })
    }


       
}
