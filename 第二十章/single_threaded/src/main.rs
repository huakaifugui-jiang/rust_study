/*
 * @Author: wlj
 * @Date: 2022-12-26 11:06:59
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-26 15:37:24
 * @Description: 构建单线程 web server
 * @see:https://kaisery.github.io/trpl-zh-cn/ch20-01-single-threaded.html
 */
// 首先让我们创建一个可运行的单线程 web server，不过在开始之前，我们将快速了解一下构建 web server 所涉及到的协议。
//这些协议的细节超出了本书的范畴，不过一个简单的概括会提供我们所需的信息。

//web server 中涉及到的两个主要协议是超文本传输协议(Hypertext Transfer Protocol , HTTP) 和 传输控制协议(Transmission Control Protocol ,TCP)
//这两者都是请求-响应(request-reponse)协议，也就是说，有客户端（client）来初始化请求，并有服务端（server）监听请求并向客户端提供响应。请求与响应的内容由协议本身定义
//TCP是一个底层协议，它描述了信息如何从一个server到另一个的细节，不过其并不指定信息是什么。HTTP构建于TCP之上，它定义了请求和响应的内容。为此，技术上讲可将HTTP用于其他协议之上
//不过对于绝大部分情况，HTTP 通过 TCP 传输。我们将要做的就是处理TCP 和 HTTP 请求与响应的原始字节数据。

//监听TCP连接
//所以我们的web server 所需做的第一件事便是能够监听TCP连接。标准库提供了std::net 模块处理这些功能。
use std::{
    fs,
    io::prelude::*, //这里将 std::io::prelude 引入作用域来获取读写流所需的特定 trait。
    net::{TcpListener, TcpStream},
}; //TcpListener 用于监听 TCP 连接。
fn main() {
    //这段代码会在地址 127.0.0.1:7878 上监听传入的 TCP 流。当获取到传入的流，它会打印出 Connection established!
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); //在这个场景中 bind 函数类似于 new 函数，在这里它返回一个新的 TcpListener 实例。这个函数叫做 bind 是因为，在网络领域，连接到监听端口被称为 “绑定到一个端口”（“binding to a port”）
                                                                 //bind 函数返回 Result<T, E>，这表明绑定可能会失败，例如，连接 80 端口需要管理员权限（非管理员用户只能监听大于 1024 的端口），所以如果不是管理员尝试连接 80 端口，则会绑定失败
                                                                 //另一个例子是如果运行两个此程序的实例这样会有两个程序监听相同的端口，绑定会失败。因为我们是出于学习目的来编写一个基础的 server，将不用关心处理这类错误，使用 unwrap 在出现这些情况时直接停止程序。

    //TcpListener 用于监听 TCP 连接。我们选择监听地址 127.0.0.1:7878。将这个地址拆开，冒号之前的部分是一个代表本机的IP地址（这个地址在每台计算机上都相同）
    //而7878是端口号，选择这个端口出于两个原因：通常HTTP接受这个端口而且7878在电话上打出来就是“rust”（译者注：九宫格键盘上的英文）。

    // /TcpListener 的 incoming 方法返回一个迭代器，它提供了一系列的流（更准确的说是 TcpStream 类型的流）
    //流（stream）代表一个客户端和服务端之间打开的连接。连接（connection）代表客户端连接服务端、服务端生成响应以及服务端关闭连接的全部请求/响应过程。为此
    //TcpStream 允许我们读取它来查看客户端发送了什么，并可以编写响应。总体来说，这个for循环会依次处理每个连接并产生一系列的流提供我们处理。
    for stream in listener.incoming() {
        let stream = stream.unwrap(); //目前为止，处理流的过程包含 unwrap 调用，如果出现任何错误会终止程序，如果没有任何错误，则打印出信息。
                                      //下一个示例我们将为成功的情况增加更多功能。当客户端连接到服务端时 incoming 方法返回错误是可能的，因为我们实际上没有遍历连接，而是遍历 连接尝试（connection attempts）。
                                      //连接可能会因为很多原因不能成功，大部分是操作系统相关的。例如，很多系统限制同时打开的连接数；新连接尝试产生错误，直到一些打开的连接关闭为止。
                                      // println!("Connection established!");

        //读取请求
        handle_connection(stream)
    }
    //让我们试试这段代码！首先在终端执行 cargo run，接着在浏览器中加载 127.0.0.1:7878。浏览器会显示出看起来类似于“连接重置”（“Connection reset”）的错误信息，，
    //因为 server 目前并没响应任何数据。但是如果我们观察终端，会发现当浏览器连接 server 时会打印出一系列的信息！
    //有时会看到对于一次浏览器请求会打印出多条信息；这可能是因为浏览器在请求页面的同时还请求了其他资源，比如出现在浏览器 tab 标签中的 favicon.ico。
    //这也可能是因为浏览器尝试多次连接 server，因为 server 没有响应任何数据。当 stream 在循环的结尾离开作用域并被丢弃，其连接将被关闭，作为 drop 实现的一部分。浏览器有时通过重连来处理关闭的连接，因为这些问题可能是暂时的。现在重要的是我们成功的处理了 TCP 连接！
    //记得当运行完特定版本的代码后使用 ctrl-C 来停止程序。并在做出最新的代码修改之后执行 cargo run 重启服务。
}

//读取Tcp流 stream 参数是可变的。这是因为TcpStream实例在内部记录了所返回的数据。它可能读取了多余我们请求的数据并保存它们以备下一次请求数据。
//因此它需要是 mut 的 因为其内部状态可能会发生改变；通常我们认为“读取” 不需要可变性，不过这个例子中需要mut 关键字。
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; //首先在栈上声明一个buffer 来存放需要读取到的数据。这里创建了一个1024字节的缓冲区
                                //它足以存放基本的数据并满足本章的目的需要。如果希望处理任意大小的请求，缓冲区管理将更为复杂，不过现在一切从简。

    stream.read(&mut buffer).unwrap(); //接着将缓冲区传递给stream.read，它会从TcpStream中读取字节并放入缓冲区

    // println!("Request : {}", String::from_utf8_lossy(&buffer[..])); //将缓冲区中的字节转换为字符串打印出来。
    //String::from_utf8_lossy函数获取一个&[u8]并产生一个string。函数名的 “lossy” 部分来源于当其遇到无效的UTF-8序列的行为：它使用 �，U+FFFD REPLACEMENT CHARACTER，来代替无效序列。
    //你可能会在缓冲区的剩余部分看到这些替代字符，因为他们没有被请求数据填满。

    //打印出来看起来像这样（可能也有不同
    // Request: GET / HTTP/1.1
    // Host: 127.0.0.1:7878
    // User-Agent: Mozilla/5.0 (Windows NT 10.0; WOW64; rv:52.0) Gecko/20100101
    // Firefox/52.0
    // Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
    // Accept-Language: en-US,en;q=0.5
    // Accept-Encoding: gzip, deflate
    // Connection: keep-alive
    // Upgrade-Insecure-Requests: 1
    // ������������������������������������

    //根据使用的浏览器的不同可能会出现稍微不同的数据。现在我们打印出了请求数据，可以通过观察Request:Get 之后的路径来解释为何会从浏览器中得到多个连接。
    //如果重复的连接都是请求/，就知道了浏览器尝试重复获取/因为它没有从程序得到响应。

    //仔细观察 HTTP 请求
    //HTTP是一个基于文本的协议，同时有一个请求有如下格式：
    //Method Request-URL HTTP-Version CRLF
    //headers CRLF
    //message-body
    //第一行叫做请求行（request line），它存放了客户端请求了什么的信息。请求行第一部分是所使用的method，比如GET 或者 POST ，这描述了客户端如何进行请求。这里客户端使用了GET请求
    //请求行接下来的部分是/，它代表客户端请求的统一资源标识符（Uniform Resource Identifier，URI）--URI大体上类似，但也不完全类似于URL（统一资源定位符，Uniform Resource Locators）。
    //URL 和 URI 之间的区别对于本章的墓地来说并不重要，不过HTTP规范 使用术语 URI，所以这里可以简单的将URL理解为URI。

    //最后以部分是客户端使用的HTTP版本，然后请求行以CRLF序列(CRLF代表回车和换行，carriage return line feed，这是打字机时代的术语！)结束。CRLF序列也可以写成\r\n,气质\r是回车符，\n是换行符
    //CRLF序列将请求行与其余请求数据分开。 请注意，打印CRLF时，我们会看到一个新行，而不是\r\n。

    //观察目前运行程序所接收到的数据的请求行，可以看到GET是method 。 / 是请求URI， 而HTTP/1.1是版本
    //从Host:开始的其余行是headers；GET请求没有body。
    //如果你希望的话，尝试用不同的浏览器发送请求，或请求不同的地址，比如 127.0.0.1:7878/test，来观察请求数据如何变化。
    //现在我们知道了浏览器请求了什么。让我们返回一些数据！
    //编写响应
    //我们将实现在客户端请求的响应中发生数据的功能。响应有如下格式：
    //HTTP-Version Status-Code Reason-Phrase CRLF
    //headers CRLF
    //message-body
    //第一行叫做状态行（status line），它包含响应的HTTP版本、一个数字状态码用以总结请求的结果和一个描述之前状态码的文本原因短语。CRLF 序列之后是任意 header，另一个 CRLF 序列，和响应的 body。
    // 这里是一个使用 HTTP 1.1 版本的响应例子，其状态码为 200，原因短语为 OK，没有 header，也没有 body：
    // HTTP/1.1 200 OK\r\n\r\n
    // 状态码 200 是一个标准的成功响应。这些文本是一个微型的成功 HTTP 响应。让我们将这些文本写入流作为成功请求的响应！

    // let response = "HTTP/1.1 200 OK\r\n\r\n"; //定义变量response来存放将要返回的成功响应的数据。

    // stream.write(response.as_bytes()).unwrap(); //接着在response上调用as_bytes，因为stream的write方法获取一个&[u8]并直接将这些字节发送给连接。
    //                                             //因为 write 操作可能会失败，所以像之前那样对任何错误结果使用 unwrap。同理，在真实世界的应用中这里需要添加错误处理。
    // stream.flush().unwrap(); //最后flush 会等待并阻塞程序执行直到所有字节被写入连接中；TcpStream包含一个内部缓冲区来最小化对底层操作系统的调用。
    //有了这些修改，运行我们的代码并进行请求！我们不再向终端打印任何数据，所以不会再看到除了 Cargo 以外的任何输出。不过当在浏览器中加载 127.0.0.1:7878 时，会得到一个空页面而不是错误。
    //太棒了！我们刚刚手写了一个 HTTP 请求与响应。
    //返回真正的HTML
    //让我们实现不只是返回空页面的功能。在项目根目录创建一个新文件，hello.html —— 也就是说，不是在 src 目录。在此可以放入任何你期望的 HTML；
    //~/hello.html这是一个极小化的HTML5文档，他有一个标题和一小段文本。为了在server接受请求时返回它，需要：将其加入响应的body中

    // let contents = fs::read_to_string("hello.html").unwrap();
    // let response = format!(
    //     "HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}",
    //     contents.len(),
    //     contents
    // ); //使用format!将文件内容加入到将要写入流的成功响应的body中。

    // stream.write(response.as_bytes()).unwrap();
    // stream.flush().unwrap()
    //使用 cargo run 运行程序，在浏览器加载 127.0.0.1:7878，你应该会看到渲染出来的 HTML 文件！
    //目前忽略了 buffer 中的请求数据并无条件的发送了 HTML 文件的内容。
    //这意味着如果尝试在浏览器中请求 127.0.0.1:7878/something-else 也会得到同样的 HTML 响应。
    //如此其作用是非常有限的，也不是大部分 server 所做的；让我们检查请求并只对格式良好（well-formed）的请求 / 发送 HTML 文件。

    //验证请求并有选择的进行响应
    //目前我们的web server 不管客户端请求什么都会返回相同的HTML文件。让我们增加在返回HTML文件前检查浏览器是否请求/，
    //并在其请求任何其他内容时返回错误的功能。
    // let get = b"GET / HTTP/1.1\r\n"; //首先，将与 / 请求相关的数据硬编码进变量 get 因为我们将原始字节读取进了缓冲区，所以在 get 的数据开头增加 b"" 字节字符串语法将其转换为字节字符串。

    // //接着检查 buffer 是否以 get 中的字节开头。如果是，这就是一个格式良好的 / 请求，也就是 if 块中期望处理的成功情况，并会返回 HTML 文件内容的代码。
    // if buffer.starts_with(get) {
    //     let contents = fs::read_to_string("hello.html").unwrap();

    //     let response = format!(
    //         "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    //         contents.len(),
    //         contents
    //     );

    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // } else {
    //     //如果 buffer 不 以 get 中的字节开头，就说明接收的是其他请求。之后会在 else 块中增加代码来响应所有其他请求。
    //     //some other request
    //     let status_line = "HTTP/1.1 404 NOT FOUND";
    //     let contents = fs::read_to_string("404.html").unwrap();

    //     let response = format!(
    //         "{}\r\nContent-Length: {}\r\n\r\n{}",
    //         status_line,
    //         contents.len(),
    //         contents
    //     );

    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // }

    //少量代码重构
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
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

    //现在 if 和 else 块所做的唯一的事就是在一个元组中返回合适的状态行和文件名的值；接着使用第十八章讲到的使用模式的 let 语句通过解构元组的两部分为 filename 和 header 赋值。
    //之前读取文件和写入响应的冗余代码现在位于 if 和 else 块之外，并会使用变量 status_line 和 filename。
    //这样更易于观察这两种情况真正有何不同，还意味着如果需要改变如何读取文件或写入响应时只需要更新一处的代码。
    //目前 server 运行于单线程中，它一次只能处理一个请求。让我们模拟一些慢请求来看看这如何会成为一个问题，并进行修复以便 server 可以一次处理多个请求。
}
