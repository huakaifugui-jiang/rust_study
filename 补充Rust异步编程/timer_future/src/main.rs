/*
 * @Author: wulongjiang
 * @Date: 2022-12-29 03:34:21
 * @LastEditors: wulongjiang
 * @LastEditTime: 2022-12-29 18:43:20
 * @Description:
 * @FilePath: \timer_future\src\main.rs
 */

use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        future::Future,
        sync::mpsc::{sync_channel, Receiver, SyncSender},
        sync::{Arc, Mutex},
        task::{Context, Poll},
        time::Duration,
    },
    // 引入之前实现的定时器模块
    timer_future::Timer,
};

fn main() {
    //构建执行器，它可以同时并发允许多个Future。例子中，需要用到 futures 包的 ArcWake 特征，它可以提供一个方便的途径去构建一个 Waker 。
    //执行器需要从一个消息通道( channel )中拉取事件，然后运行它们。当一个任务准备好后（可以继续执行），它会将自己放入消息通道中，然后等待执行器 poll 。
    /// 任务执行器，负责从通道中接收任务然后执行
    struct Executor {
        ready_queue: Receiver<Arc<Task>>,
    }

    /// `Spawner`负责创建新的`Future`然后将它发送到任务通道中
    #[derive(Clone)]
    struct Spawner {
        task_sender: SyncSender<Arc<Task>>,
    }

    /// 一个Future，它可以调度自己(将自己放入任务通道中)，然后等待执行器去`poll`
    struct Task {
        /// 进行中的Future，在未来的某个时间点会被完成
        ///
        /// 按理来说`Mutex`在这里是多余的，因为我们只有一个线程来执行任务。但是由于
        /// Rust并不聪明，它无法知道`Future`只会在一个线程内被修改，并不会被跨线程修改。因此
        /// 我们需要使用`Mutex`来满足这个笨笨的编译器对线程安全的执着。
        ///
        /// 如果是生产级的执行器实现，不会使用`Mutex`，因为会带来性能上的开销，取而代之的是使用`UnsafeCell`
        future: Mutex<Option<BoxFuture<'static, ()>>>,

        /// 可以将该任务自身放回到任务通道中，等待执行器的poll
        task_sender: SyncSender<Arc<Task>>,
    }
    fn new_executor_and_spawner() -> (Executor, Spawner) {
        // 任务通道允许的最大缓冲数(任务队列的最大长度)
        // 当前的实现仅仅是为了简单，在实际的执行中，并不会这么使用
        const MAX_QUEUED_TASKS: usize = 10_000;
        let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
        (Executor { ready_queue }, Spawner { task_sender })
    }
    // 下面再来添加一个方法用于生成 Future , 然后将它放入任务通道中:
    impl Spawner {
        fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
            let future = future.boxed();
            let task = Arc::new(Task {
                future: Mutex::new(Some(future)),
                task_sender: self.task_sender.clone(),
            });
            self.task_sender.send(task).expect("任务队列已满");
        }
    }
}
