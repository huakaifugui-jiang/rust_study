/*
 * @Author: wulongjiang
 * @Date: 2022-12-29 03:34:21
 * @LastEditors: wulongjiang
 * @LastEditTime: 2022-12-31 21:44:59
 * @Description:
 * @FilePath: \timer_future\src\main.rs
 */

use std::collections::HashMap;
use std::vec;

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
    timer_future::TimerFuture,
};

fn main() {
    // Rust 的 Future 是惰性的：只有屁股上拍一拍，它才会努力动一动。其中一个推动它的方式就是在 async 函数中使用 .await 来调用另一个 async 函数，但是这个只能解决 async 内部的问题，
    //那么这些最外层的 async 函数，谁来推动它们运行呢？答案就是我们之前多次提到的执行器 executor 。
    //执行器会管理一批 Future (最外层的 async 函数)，然后通过不停地 poll 推动它们直到完成。
    //最开始，执行器会先 poll 一次 Future ，后面就不会主动去 poll 了，而是等待 Future 通过调用 wake 函数来通知它可以继续，它才会继续去 poll
    //那要怎么poll呢？ 消息通道
    //这种wake 通知然后 poll的方式会不断重复，直到 Future 完成。

    //构建执行器， 需要用到futures包的ArcWake特征，它可以提供一个方便的途径去构建一个Waker。
    //执行器需要一个消息通道（channel）中拉取事件，然后允许它们。当一个任务准备好后（可继续执行），它会将自己放入消息通道中，然后等待执行器poll

    ///任务执行器（接收者 消费者），负责从通道中执行任务然后执行
    struct Executor {
        ready_queue: Receiver<Arc<Task>>,
    }

    /// Spawner (发送者 生产者) 负责创建新的Future然后把他发送到任务通道中
    struct Spawner {
        task_sender: SyncSender<Arc<Task>>,
    }

    /// 一个Future，它可以调度自己（将自己放入任务通道中，因为执行器只会进行一次poll），然后等待执行器去poll
    struct Task {
        /// 进行中的Future，在未来的某个时间点会被完成
        /// 按道理来说 mutex在这是多余的，因为我们只有一个线程来执行任务。但是由于
        /// Rust并不聪明，它无法知道`Future`只会在一个线程内被修改，并不会被跨线程修改。因此我们需要使用`Mutex`来满足这个笨笨的编译器对线程安全的执着。
        /// 如果是生产级的执行器实现，不会使用`Mutex`，因为会带来性能上的开销，取而代之的是使用`UnsafeCell`
        future: Mutex<Option<BoxFuture<'static, ()>>>,
        ///可以将该任务放回到任务通道中，等待执行器的poll
        task_sender: SyncSender<Arc<Task>>,
    }

    fn new_executor_and_spawner() -> (Executor, Spawner) {
        //（同步）任务通道允许的最大缓冲数（任务队列的最大长度）
        // 当前的实现仅仅是为了简单，在实际的执行中，并不会这么使用
        const MAX_QUEUED_TASKS: usize = 10_000;
        let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
        (Executor { ready_queue }, Spawner { task_sender })
    }
    //下面再来添加一个方法用于生成Future，然后将它放入任务通道中
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

    //在执行器poll一个Future之前，首先需要调用wake方法进行唤醒，然后再由Waker
    //负责调度该任务并将其放入任务通道中。创建Waker的最简单的方式就是实现ArcWake特征
    //先来为我们的任务实现ArcWake特征，这样他们就能被转变为Waker然后被唤醒：
    impl ArcWake for Task {
        fn wake_by_ref(arc_self: &Arc<Self>) {
            //通过发送任务到任务管道的方式来实现wake，，这样`wake`后，任务就能被执行器`poll`
            let tast_clone = arc_self.clone();
            arc_self.task_sender.send(tast_clone).expect("任务队列已满");
        }
    }

    //当任务实现了ArcWake特征后，它就变成了Waker，再调用wake()对其唤醒后会将
    //任务复制一份，然后将其发送到任务通道中。最后我们的执行器将从通道中获取任务，然后进行 poll 执行
    impl Executor {
        fn run(&self) {
            while let Ok(task) = self.ready_queue.recv() {
                //获取一个future，如果它还没有完成（仍然是Some，不是None）则对它进行一次poll并尝试完成它
                let mut future_slot = task.future.lock().unwrap();
                if let Some(mut future) = future_slot.take() {
                    //基于任务自身创建一个 `LocalWaker`(进行下次poll)
                    let waker = waker_ref(&task);
                    let context = &mut Context::from_waker(&*waker);
                    // `BoxFuture<T>`是`Pin<Box<dyn Future<Output = T> + Send + 'static>>`的类型别名
                    //通过调用`as_mut`方法，可以将上面的类型转换成`Pin<&mut dyn Future + Send + 'static>`
                    if future.as_mut().poll(context).is_pending() {
                        //Future还没执行完就放回任务中，等待下次被poll
                        *future_slot = Some(future);
                    }
                }
            }
        }
    }
    //恭喜！我们终于拥有了自己的执行器，下面再来写一段代码使用该执行器去运行之前的定时器 Future ：

    let (executor, spawner) = new_executor_and_spawner();
    // 生成一个任务
    spawner.spawn(async {
        println!("howdy!");
        // 创建定时器Future，并等待它完成
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    //drop 掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来
    drop(spawner);
    // 运行执行器直到任务队列为空
    // 任务运行后，会先打印`howdy!`, 暂停2秒，接着打印 `done!`
    executor.run();
    println!("main thread");
    let nums = vec![32];
    let mut map = HashMap::new();
    let target = 9;
    for i in nums {
        if map.contains_key(&(target - nums[i])) {
            return vec![map.get(&(target - i)),i]
        };
        map.insert(nums[i], i);
    }
}
