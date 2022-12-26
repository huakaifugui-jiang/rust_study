/*
 * @Author: wlj
 * @Date: 2022-12-26 15:41:15
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-26 17:28:19
 * @Description: 将单线程 server 变为多线程 server
 * @see:https://kaisery.github.io/trpl-zh-cn/ch20-02-multithreaded.html
 */

use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

//目前server会依次处理每一请求，意味着它在完成第一个连接的处理之前不会处理第二个连接。如果server正接收越来越多的请求，这类串行操作会时性能越来越差。
//如果一个请求花费很长实际来处理，随后而来的请求则不等不等待这个长请求结束，即使这些请求可以很快就处理完。我们需要修复这种情况，不过首先让我们实际尝试一下这个问题。
fn main() {
    // let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();

    //     handle_connection(stream);
    // }

    //为每一个请求分配线程的代码结构
    //首先，让我们探索一下为每一个连接都创建一个线程的代码看起来如何。这并不是最终方案，因为正如之前讲到的它会潜在的分配无限的线程，不过这是一个开始。
    // let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     thread::spawn(|| handle_connection(stream));
    // }
    //正如第十六章讲到的，thread::spawn 会创建一个新线程并在其中运行闭包中的代码。如果运行这段代码并在在浏览器中加载 /sleep，
    //接着在另两个浏览器标签页中加载 /，确实会发现 / 请求不必等待 /sleep 结束。
    //不过正如之前提到的，这最终会使系统崩溃因为我们无限制的创建新线程。

    //为有限数量的线程创建一个类似的接口
    //我们期望线程池以类似且熟悉的方式工作，以便从线程切换到线程池并不会对使用该 API 的代码做出较大的修改。
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4); //创建一个新的线程池，它有一个课配置的线程数参数，在这里是4
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            //pool.execute有着类似thread::spawn的接口，它获取一个线程池运行于每一个流的闭包。
            //pool.execute 需要实现为获取闭包并传递给池中的线程运行。这段代码还不能编译，不过通过尝试编译器会指导我们如何修复它。
            handle_connection(stream)
        })
    }
    //报错，报错告诉我们需要一个 ThreadPool 类型或模块，所以我们将构建一个。
    //ThreadPool 的实现会与 web server 的特定工作相独立，所以让我们从 hello crate 切换到存放 ThreadPool 实现的新库 crate。
    //这也意味着可以在任何工作中使用这个单独的线程池库，而不仅仅是处理网络请求。
}

fn handle_connection(mut stream: TcpStream) {
    //在当前 server 实现中模拟慢请求
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        //当接收到这个请求时，在渲染成功 HTML 页面之前会先休眠五秒。
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    //使用 cargo run 启动 server，并接着打开两个浏览器窗口：一个请求 http://127.0.0.1:7878/ 而另一个请求 http://127.0.0.1:7878/sleep
    //如果像之前一样多次请求 /，会发现响应的比较快速。不过如果请求 /sleep 之后在请求 /，就会看到 / 会等待直到 sleep 休眠完五秒之后才出现。
    //这里有多种办法来改变我们的 web server 使其避免所有请求都排在慢请求之后；我们将要实现的一个便是线程池。

    //使用线程池改善吞吐量
    //线程池（thread pool）是一组预先分配的等待或准备处理任务的线程。当程序收到一个新任务，线程池中的一个线程会被分配任务，这个线程会离开并处理任务。
    //其余的线程则可用于处理在第一个线程处理任务的同时处理其他接收到的任务。当第一个线程处理完任务时，它会返回空线程池中等待处理新任务。线程池允许我们并发处理连接，增加server的吞吐量
    //我们会将池中线程限制为较少的数量，以防被拒绝服务（Denial of Service，Dos）攻击；如果程序为每一个接收的请求都新建一个线程，某人向server发起千万级的请求时会耗尽服务器的资源并
    //导致所有请求的处理都被终止。
    //不同于分配无限的线程，线程池中将有固定数量的等待线程。当新进请求时，将有固定数量的等待线程。当新进请求时，将请求发送到线程池 中做处理。
    //线程池会维护一个接收请求的队列。每一个线程会从队列中取出一个请求，处理请求，接着向队列索取另一个请求。通过这种设计，则可以并发处理N个请求，其中N为线程数。如果每一个线程都在
    //响应慢请求，之后的请求仍会阻塞队列，不过相比之前增加了能处理的慢请求的数量。
    //这个设计仅仅是多种改善 web server 吞吐量的方法之一。其他可供探索的方法有 fork/join 模型 和 单线程异步I/O 模型。如果你对这个主题感兴趣，则可以阅读更多关于其他解决方案的内容
    //并尝试用Rust实现他们；对于一个像 Rust 这样的底层语言，所有这些方法都是可能的。
    //在开始之前，让我们讨论一下线程池应用看起来怎样。当尝试设计代码时，首先编写客户端接口确实有助于指导代码设计。以期望的调用方式来构建API代码的结构，接着在这个结构之内实现功能，
    //而不是先实现功能再设计公有API。
    //类似第十二章项目中使用的测试驱动开发。这里将要使用编译器驱动开发（compiler-driven development)。我们将编写调用所期望的函数的代码，接着观察编译器错误告诉我们接下来需要修改什么使得代码可以工作。
}
