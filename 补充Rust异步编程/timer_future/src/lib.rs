/*
 * @Author: wulongjiang
 * @Date: 2022-12-29 02:54:28
 * @LastEditors: wulongjiang
 * @LastEditTime: 2022-12-29 03:30:12
 * @Description: 构建一个简单的定时器Future
 * @FilePath: \timer_future\src\lib.rs
 */
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

//为了让例子尽量简单，当计时器创建时，我们会启动一个线程接着让该线程进入睡眠，等睡眠结束后再通知给 Future 。
//定时器是异步的所以要有Future
#[derive(Debug)]
pub struct Timer {
    Duration: Duration,
    status: Mutex<bool>, //true是完成，false是未完成
}

impl Timer {
    pub fn new(duration: Duration) {
        let timer = Timer {
            Duration: duration,
            status: Mutex::new(false),
        };
        thread::spawn(move || {
            thread::sleep(timer.Duration);
            let mut status = timer.status.lock().unwrap();
            *status = true;
        });
    }
}

impl Future for Timer {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let timer_status = self.status.lock().unwrap();
        if *timer_status {
            Poll::Ready(())
        } else {
            cx.waker();
            Poll::Pending
        }
    }
}
