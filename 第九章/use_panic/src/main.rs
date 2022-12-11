/*
 * @Author: wulongjiang
 * @Date: 2022-12-11 11:57:46
 * @LastEditors: wulongjiang
 * @LastEditTime: 2022-12-11 17:29:48
 * @Description:用panic！处理不可恢复的错误
 * @see：https://kaisery.github.io/trpl-zh-cn/ch09-01-unrecoverable-errors-with-panic.html
 * @FilePath: \rust_study\第九章\use_panic\src\main.rs
 */

//突然有一天，代码出问题了，而你对此束手无策。对于这种情况，Rust 有 panic!宏。当执行这个宏时，程序会打印出一个错误信息，展开并清理栈数据，
//然后接着退出。出现这种情况的场景通常是检测到一些类型的 bug，而且程序员并不清楚该如何处理它

//对应panic时的栈展开或终止。
//当出现panic时，程序默认会开始展开（unwingding），这意味着Rust会回溯(backtrace)并清理它遇到过的每一个函数数据，不过这个回溯
//并清理的过程有很多工作。另一种时选择直接终止(abort)，这会不清理数据就推出程序。那么程序所使用的内存需要由操作系统来清理
//如果你需要项目的最终二进制文件越小越好，panic时通过在Cargo.toml的[profile]部分增加panic='abort'，可以由展开切换为终止。

fn main() {
    //panic!("error");  // 报错 thread 'main' panicked at 'error', src\main.rs:20:5
    //note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    //在这个例子中，我们可以看到报错的指向的行数是 src\main.rs:20:5 是panic！宏的调用
    //而不是我们代码最终导致panic!的那一行。看一下下面的提示，我们可以使用panic!被调用的函数的backtrace(回溯)来寻找出代码中
    //出现问题的地方。下面我们会详细的介绍backtrace是什么

    let v = vec![1, 2, 3];
    v[99]; //报错thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src\main.rs:27:5
           //note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace 这里提示我们可以设置RUST_BACKTRACE环境变量来得到一个回溯
           //windows 系统 可以先允许 set RUST_BACKTRACE=1 来设置环境 然后再允许cargo run

    //访问超越数据结构的元素是十分危险的。 这会造成panic！
    //C语言中，尝试读取数据结构之后的值是未定义行为(undefined behavior)。你会得到任何对应数据结构中的这个元素的内存位置的值，
    //甚至是这些内存并不属于这个数据结构的情况。这被称为缓冲区溢出(buffer overread),并可能会导致安全漏洞，比如攻击者可以像这样操作索引
    //缓冲区溢出维基百科[https://zh.wikipedia.org/wiki/%E7%BC%93%E5%86%B2%E5%8C%BA%E6%BA%A2%E5%87%BA]
    //所以为了保护程序远离这些漏洞，如果尝试读取一个索引不存在的元素，Rust会停止并拒绝继续。报错 造成程序panic
}
