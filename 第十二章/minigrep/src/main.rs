// 第一个任务是让 minigrep 能够接受两个命令行参数：文件名和要搜索的字符串。也就是说我们希望能够使用 cargo run、要搜索的字符串和被搜索的文件的路径来运行程序，像这样：
//cargo run searchstring example-filename.txt

//为了确保 minigrep 能够获取传递给它的命令行参数的值，我们需要一个 Rust 标准库提供的函数，也就是 std::env::args
//这个函数返回一个传递给程序的命令行参数的 迭代器（iterator）。我们会在 第十三章 全面的介绍它们
//但是现在只需理解迭代器的两个细节：迭代器生成一系列的值，可以在迭代器上调用 collect 方法将其转换为一个集合，比如包含所有迭代器产生元素的 vector

use std::env;
use std::fs;
fn main() {
    //获取命令行的输入
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args); //Vector的第一个值是"target\\debug\\minigrep.exe",它是我们二进制文件的名称 这与 C 中的参数列表的行为相匹配，让程序使用在执行时调用它们的名称
                            //如果要在消息中打印它或者根据用于调用程序的命令行别名更改程序的行为，通常可以方便地访问程序名称 (给命令取一个比较短的名字的意思)
    let config = parse_config(&args);

    //读取文件
    let contents = fs::read_to_string(config.filename).expect("Some Error");
    println!("With text:\n{}", contents);
    //二进制项目的关注分离
    //main 函数负责多个任务的组织问题在许多二进制项目中很常见。所以 Rust 社区开发出一类在 main 函数开始变得庞大时进行二进制程序的关注分离的指导性过程。这些过程有如下步骤：
    //将程序拆分成 main.rs 和 lib.rs 并将程序的逻辑放入 lib.rs(还可以做集成测试) 中
    //当命令行解析逻辑比较小时，可以保留在 main.rs 中。
    //当命令行解析开始变得复杂时，也同样将其从 main.rs 提取到 lib.rs 中。
    //经过这些过程之后保留在 main 函数中的责任应该被限制为：
    //使用参数值调用命令行解析逻辑
    //设置任何其他的配置
    //调用 lib.rs 中的 run 函数
    // 如果 run 返回错误，则处理这个错误
    //这个模式的一切就是为了关注分离：main.rs 处理程序运行，而 lib.rs 处理所有的真正的任务逻辑。
    //因为不能直接测试 main 函数，这个结构通过将所有的程序逻辑移动到 lib.rs 的函数中使得我们可以测试他们。
    //仅仅保留在 main.rs 中的代码将足够小以便阅读就可以验证其正确性。
}

struct Config {
    filename: String,
    query: String,
}

fn parse_config(args: &[String]) -> Config {
    Config {
        //由于其运行时消耗，许多 Rustacean 之间有一个趋势是倾向于避免使用 clone 来解决所有权问题。
        //在关于迭代器的第十三章中，我们将会学习如何更有效率的处理这种情况，不过现在，复制一些字符串来取得进展是没有问题的，因为只会进行一次这样的拷贝，而且文件名和要搜索的字符串都比较短。
        //在第一轮编写时拥有一个可以工作但有点低效的程序要比尝试过度优化代码更好一些。
        filename: args[2].clone(),
        query: args[1].clone(),
    }
}
