/*
 * @Author: wulongjiang
 * @Date: 2022-12-14 21:40:23
 * @LastEditors: wulongjiang
 * @LastEditTime: 2022-12-14 22:59:19
 * @Description: 如何编写测试
 * @see:https://kaisery.github.io/trpl-zh-cn/ch11-01-writing-tests.html
 * @FilePath: \adder\src\lib.rs
 */
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*; //因为这是一个内部模块，要测试外部模块中的代码，需要将其引入到内部模块的作用域中。
    #[test] //表明这是一个测试函数,这样测试执行者就知道将其作为测试处理
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4); //函数体通过使用 assert_eq! 宏来断言 2 加 2 等于 4。
                               //test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
                               //1 passed; 0 failed 表示通过或失败的测试数量
                               //因为之前我们并没有将任何测试标记为忽略，所以摘要中会显示 0 ignored。
                               //我们也没有过滤需要运行的测试，所以摘要中会显示0 filtered out
                               //0 measured 统计是针对性能测试的。性能测试（benchmark tests）在编写本书时，仍只能用于 Rust 开发版（nightly Rust）。

        //Doc-tests adder  开头的这一部分是所有文档测试的结果。
        //我们现在并没有任何文档测试，不过 Rust 会编译任何在 API 文档中的代码示例。这个功能帮助我们使文档和代码保持同步！
        //在第十四章的 “文档注释作为测试” 部分会讲到如何编写文档测试。现在我们将忽略 Doc-tests 部分的输出
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    //使用 assert! 宏来检查结果
    //assert! 宏由标准库提供，在希望确保测试中一些条件为 true 时非常有用。需要向 assert! 宏提供一个求值为布尔值的参数。
    //如果值是 true，assert! 什么也不做，同时测试会通过。如果值为 false，assert! 调用 panic! 宏，这会导致测试失败。
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            //判断是否能够包容下第二个长方形
            self.width > other.width && self.height > other.height
        }
    }
    //can_hold 方法返回一个布尔值，这意味着它完美符合 assert! 宏的使用场景。
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    //使用 assert_eq! 和 assert_ne! 宏来测试相等
    //测试功能的一个常用方法是将需要测试代码的值与期望值做比较，并检查是否相等。可以通过向 assert! 宏传递一个使用 == 运算符的表达式来做到。
    //不过这个操作实在是太常见了，以至于标准库提供了一对宏来更方便的处理这些操作 —— assert_eq! 和 assert_ne!
    //这两个宏分别比较两个值是相等还是不相等。当断言失败时他们也会打印出这两个值具体是什么，以便于观察测试 为什么 失败，
    //而 assert! 只会打印出它从 == 表达式中得到了 false 值，而不是导致 false 的两个值。

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        // assert_eq!(4, add_two(3), "自定义失败信息 {} {}", 4, add_two(3));
        // assert_eq!(4, add_two(3));//thread 'tests::it_adds_two' panicked at 'assertion failed: `(left == right)`  left: `4`,right: `5`', src\lib.rs:93:9
    }
    //assert_ne! 宏在传递给它的两个值不相等时通过，而在相等时失败。在代码按预期运行，我们不确定值 会 是什么，不过能确定值绝对 不会 是什么的时候，

    //assert_eq! 和 assert_ne! 宏在底层分别使用了 == 和 !=。当断言失败时，这些宏会使用调试格式打印出其参数，这意味着被比较的值必需实现了 PartialEq 和 Debug trait。所有的基本类型和大部分标准库类型都实现了这些 trait。对于自定义的结构体和枚举，需要实现 PartialEq 才能断言他们的值是否相等。需要实现 Debug 才能在断言失败时打印他们的值。
    //因为这两个 trait 都是派生 trait，如第五章示例 5-12 所提到的，通常可以直接在结构体或枚举上添加 #[derive(PartialEq, Debug)] 注解。

    //使用 should_panic 检查 panic
    //除了检查代码是否返回期望的正确的值之外，检查代码是否按照期望处理错误也是很重要的。
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess { value }
        }
    }
    //其他使用 Guess 的代码都是基于 Guess 实例仅有的值范围在 1 到 100 的前提。可以编写一个测试来确保创建一个超出范围的值的 Guess 实例会 panic
    #[test]
    #[should_panic]
    //可以通过对函数增加另一个属性 should_panic 来实现这些。这个属性在函数中的代码 panic 时会通过，而在其中的代码没有 panic 时失败。
    // #[should_panic(expected = "Guess value must be less than or equal to 100")] //设置预期的值
    fn greater_than_100() {
        Guess::new(200);
    }

    // 将 Result<T, E> 用于测试
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
