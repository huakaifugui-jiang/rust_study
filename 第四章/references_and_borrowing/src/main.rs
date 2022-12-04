/*
 * @Author: wulongjiang
 * @Date: 2022-12-04 17:43:43
 * @LastEditors: wulongjiang
 * @LastEditTime: 2022-12-04 18:28:08
 * @Description:引用与借用
 * @FilePath: \rust_study\第四章\references_and_borrowing\src\main.rs
 * @see:https://kaisery.github.io/trpl-zh-cn/ch04-02-references-and-borrowing.html
 */

//引用
fn overship_fn(s: String) -> String {
    s
}

fn learn_reference(s: &String) -> usize {
    s.len()
}

fn main() {
    let s = String::from("hello,world");
    let s1 = overship_fn(s); //就如上一小节提到的，s的所有权（overship）被移到到了overship_fn 函数中，下面的作用域就不能再 使用s;除非你再创建一个变量，并且返回s。这在实际中是十分繁琐的。
                             //所以为了调用后能再使用String 我们可以提供一个String的值的引用（reference）。引用（reference）像一个指针，因为它是一个地址
                             //我们可以借此访问存储于该地址的属于它的变量的数据。与指针不同的是，引用确保指向某个特定类型的有效值。

    //如何使用引用？  & 符号就是 引用
    let r1 = String::from("hello,reference");
    let len = learn_reference(&r1);
    println!("r1 : {} , len : {}", r1, len);
}
