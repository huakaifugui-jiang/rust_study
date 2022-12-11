/*
 * @Author: wulongjiang
 * @Date: 2022-12-11 17:33:36
 * @LastEditors: wulongjiang
 * @LastEditTime: 2022-12-11 20:13:57
 * @Description:用Result处理可恢复的错误
 * @see：https://kaisery.github.io/trpl-zh-cn/ch09-02-recoverable-errors-with-result.html
 * @FilePath: \use_result\src\main.rs
 */

//  大部分错误并没有严重到需要程序完全停止执行。有时，一个函数会因为一个容易理解并做出反应的原因失败。
//  例如，如果因为打开一个并不存在的文件而失败，此时我们可能想要创建这个文件，而不是终止进程。

//Reust所示枚举它的定义如下：
// enum result<T, E> {
//     Ok(T),
//     Err(E),
// }

//T和E是泛型类型参数；第十章会详细介绍泛型。现在你需要知道的就是T代表成功时返回的Ok成员中的数据类型
//而E代表失败时返回的Err成员中的错误的类型。 因为Result由这些泛型类型参数，我我们可以将 Result 类型和标准库中为其定义的函数用于很多不同的场景，这些情况中需要返回的成功值和失败值可能会各不相同。
use std::fs::File;
use std::io::ErrorKind;
fn learn_result() {
    //我们如何知道File::open返回的是什么类型呢？
    //1.我们可以查看标准库API文档https://doc.rust-lang.org/std/index.html
    //2.我们可以let f: u32 = File::open("hello.txt"); 然后允许，让编译器告诉我们，因为类型不匹配，错误信息会告诉我们f的类型应该是什么
    //通过报错expected `u32`, found enum `Result` 我们可以看到它返回的是一个Result类型。
    //note提示found enum `Result<File, std::io::Error>` 这里泛型参数T放入了成功值的类型std::fs::File，它是一个文件句柄。E被用在失败值上时E的类型时std::io::Error
    let f = File::open("hello.txt");
    //这个返回值说明File::open 调用可能会成功并返回一个可以进行读写的文件句柄。这个函数也可能会失败：例如，文件可能并不存在，或者可能没有访问文件的权限。
    // File::open 需要一个方式告诉我们是成功还是失败，并同时提供给我们文件句柄或错误信息。而这些信息正是 Result 枚举可以提供的

    //处理Result match
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),//当目录没有hello.txt文件的时候我们就会看到报错 thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }', src\main.rs:36:23
    // }

    //匹配不同的错误
    //如果我们想如果File::open 因为文件不存在而失败，我们希望创建这个文件并返回新文件的句柄。
    //如果File::open 因为任何其他原因失败，例如没有打开文件的权限，我们仍然希望panic
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            //File::open返回的Err成员中的值类型io::Error，它是一个标准库提供的结构体，这个结构体有一个返回 io::ErrorKind 值的 kind 方法可供调用。
            //io::ErrorKind 是一个标准库提供的枚举，它的成员对应 io 操作可能导致的不同错误类型。
            //我们感兴趣的成员是 ErrorKind::NotFound，它代表尝试打开的文件并不存在。这样，match 就匹配完 f 了，不过对于 error.kind() 还有一个内层 match。
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

//失败时 panic 的简写：unwrap 和 expect
fn learn_unwrap() {
    //记得注释learn_result();
    //其中之一叫做 unwrap，它的实现就类似于示例 9-4 中的 match 语句。如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。
    //如果 Result 是成员 Err，unwrap 会为我们调用 panic!。
    // let f = File::open("hello.txt").unwrap(); //此时我们可以看到 像上面例子的一个panic报错

    //expect 类似于unwrap的方法。它允许我们选择panic!的错误信息。它提供一个好的错误信息可以表名你的意图并更易于追踪panic的根域
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // expect 与 unwrap 的使用方式一样：返回文件句柄或调用 panic! 宏。
    // expect 在调用 panic! 时使用的错误信息将是我们传递给 expect 的参数，而不像 unwrap 那样使用默认的 panic! 信息。
}

//传播错误
fn read_username_from_file() {}

fn main() {
    // learn_result();
    // learn_unwrap();
    read_username_from_file()
}
