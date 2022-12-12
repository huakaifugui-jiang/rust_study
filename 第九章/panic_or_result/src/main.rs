/*
 * @Author: wlj
 * @Date: 2022-12-12 09:19:12
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-12 09:30:45
 * @Description: 要不要panic！
 * @see：https://kaisery.github.io/trpl-zh-cn/ch09-03-to-panic-or-not-to-panic.html
 */

// 那么，该如何决定何时应该 panic! 以及何时应该返回 Result 呢？如果代码 panic，就没有恢复的可能。你可以选择对任何错误场景都调用 panic!，
// 不管是否有可能恢复，不过这样就是你代替调用者决定了这是不可恢复的。
// 选择返回 Result 值的话，就将选择权交给了调用者，而不是代替他们做出决定。调用者可能会选择以符合他们场景的方式尝试恢复，或者也可能干脆就认为 Err 是不可恢复的
// ，所以他们也可能会调用 panic! 并将可恢复的错误变成了不可恢复的错误。因此返回 Result 是定义可能会失败的函数的一个好的默认选择。

// 在一些类似示例、原型代码（prototype code）和测试中， panic 比返回 Result 更为合适，不过他们并不常见。
// 让我们讨论一下为何在示例、代码原型和测试中，以及那些人们认为不会失败而编译器不这么看的情况下， panic 是合适的。
// 章节最后会总结一些在库代码中如何决定是否要 panic 的通用指导原则。

//这一章建议直接看文档
fn main() {
    println!("Hello, world!");
}
