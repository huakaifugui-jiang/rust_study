/*
 * @Author: wlj
 * @Date: 2022-12-15 08:32:45
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-15 10:34:14
 * @Description:
 */

// 控制测试如何运行

//就像 cargo run 会编译代码并运行生成的二进制文件一样，cargo test 在测试模式下编译代码并运行生成的测试二进制文件。可以指定命令行参数来改变 cargo test 的默认行为。
//例如，cargo test 生成的二进制文件的默认行为是 并行 的运行所有测试，并截获测试运行过程中产生的输出，阻止他们被显示出来，使得阅读测试结果相关的内容变得更容易。

//可以将一部分命令行参数传递给 cargo test，而将另外一部分传递给生成的测试二进制文件。为了分隔这两种参数，需要先列出传递给 cargo test 的参数，接着是分隔符 --，再之后是传递给测试二进制文件的参数。
//运行 cargo test --help 会提示 cargo test 的有关参数，而运行 cargo test -- --help 可以提示在分隔符 -- 之后使用的有关参数

//并行或连续的运行测试
//当运行多个测试时， Rust 默认使用线程来并行运行。这意味着测试会更快地运行完毕，所以你可以更快的得到代码能否工作的反馈。
//因为测试是在同时运行的(并行)，你应该确保测试不能相互依赖，或依赖任何共享的状态，包括依赖共享的环境，比如当前工作目录或者环境变量。

//举个例子，每一个测试都运行一些代码，假设这些代码都在硬盘上创建一个 test-output.txt 文件并写入一些数据。
//接着每一个测试都读取文件中的数据并断言这个文件包含特定的值，而这个值在每个测试中都是不同的。因为所有测试都是同时运行的，一个测试可能会在另一个测试读写文件过程中修改了文件。
//那么第二个测试就会失败，并不是因为代码不正确，而是因为测试并行运行时相互干扰。一个解决方案是使每一个测试读写不同的文件；另一个解决方案是一次运行一个测试。

//如果你不希望测试并行运行，或者想要更加精确的控制线程的数量，可以传递 --test-threads 参数和希望使用线程的数量给测试二进制文件。
//cargo test -- --test-threads=1 设置运行的线程数量
pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;
    //显示函数输出
    //默认情况下，当测试通过时，Rust 的测试库会截获打印到标准输出的所有内容。比如在测试中调用了 println!
    //而测试通过了，我们将不会在终端看到 println! 的输出：只会看到说明测试通过的提示行。如果测试失败了，则会看到所有标准输出和其他错误信息。
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
    //注意输出中不会出现测试通过时打印的内容，即 I got the value 4。因为当测试通过时，这些输出会被截获。失败测试的输出 I got the value 8 ，则出现在输出的测试摘要部分，同时也显示了测试失败的原因。
    // 如果你希望也能看到通过的测试中打印的值，也可以在结尾加上 --show-output 告诉 Rust 显示成功测试的输出。

    //通过指定名字来运行部分测试
    //有时运行整个测试集会耗费很长时间。如果你负责特定位置的代码，你可能会希望只运行与这些代码相关的测试。你可以向 cargo test 传递所希望运行的测试名称的参数来选择运行哪些测试
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test] 
    #[ignore]//忽略某些测试 如果我们只希望运行被忽略的测试，可以使用 cargo test -- --ignored
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    //运行单个测试
    //可以向 cargo test 传递任意测试的名称来只运行这个测试：cargo test one_hundred
}
