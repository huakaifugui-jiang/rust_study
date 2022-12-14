/*
 * @Author: wulongjiang
 * @Date: 2022-12-14 21:07:45
 * @LastEditors: wulongjiang
 * @LastEditTime: 2022-12-14 23:04:39
 * @Description: 如何编写测试
 * @see:https://kaisery.github.io/trpl-zh-cn/ch11-01-writing-tests.html
 * @FilePath: \rust_study\第十一章\write_test\src\main.rs
 */

//如何编写测试
//Rust 中的测试函数是用来验证非测试代码是否按照期望的方式运行的。测试函数体通常执行如下三种操作：
//1.设置任何所需的数据或状态
//2.运行需要测试的代码
//3.断言其结果是我们所期望的

//让我们看看 Rust 提供的专门用来编写测试的功能：test 属性、一些宏和 should_panic 属性。

//测试函数剖析

//作为最简单的例子，Rust中的测试就是一个带有test属性注解的函数。属性（attribute）是关于Rust代码片段的元数据；
//第五章中的结构体用到的derive属性就是一个例子。为了将一个函数变成测试函数需要在 fn 行之前加上 #[test]
//当使用 cargo test 命令运行测试时，Rust 会构建一个测试执行程序用来调用标记了 test 属性的函数，并报告每一个测试是通过还是失败。

// 让我们创建一个新的库项目 adder
fn main() {
    println!("Hello, world!");
}
