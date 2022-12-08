/*
 * @Author: wulongjiang
 * @Date: 2022-12-08 22:10:05
 * @LastEditors: wulongjiang
 * @LastEditTime: 2022-12-08 22:35:01
 * @Description:使用 use 关键字将路径引入作用域
 * @see:https://kaisery.github.io/trpl-zh-cn/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html
 * @FilePath: \rust_study\第七章\7.4use_keyword\src\main.rs
 */
use std::fmt;
use std::io::Result as IoResult; //我们使用 as 指定一个新的本地名称或者别名

// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io}; //通过嵌套路径来消除大量的 use 行

//use std::io;
// use std::io::Write;
// 可以缩写成/*  */
// use std::io::{self, Write};

//如果希望将一个路径下 所有 公有项引入作用域，可以指定路径后跟 *，glob 运算符：
use std::collections::*;
//这个 use 语句将 std::collections 中定义的所有公有项引入当前作用域。
//使用 glob 运算符时请多加小心！Glob 会使得我们难以推导作用域中有什么名称和它们是在何处定义的。
//glob 运算符经常用于测试模块 tests 中，这时会将所有内容引入作用域；我们将在第十一章 “如何编写测试” 部分讲解。
//glob 运算符有时也用于 prelude 模式；查看 标准库中的文档 了解这个模式的更多细节。

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
        }
    }
}

//在作用域中增加 use 和路径类似于在文件系统中创建软连接（符号连接，symbolic link）
//use crate::front_of_house::hosting;
//通过在 crate 根增加 use crate::front_of_house::hosting，现在 hosting 在作用域中就是有效的名称了，
//如同 hosting 模块被定义于 crate 根一样。通过 use 引入作用域的路径也会检查私有性，同其它路径一样。

//相对路径
// use self::front_of_house::hosting; //最好不要将add_to_waitlist直接用这样引入

//使用pub use重导出（re-exporting）名称 。 这样相当于（mod 了 hosting 外部作用域就可以访问了
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

//这样就可以区分不同的Result 而不是使用  use std::io::Result引入
fn function1() -> fmt::Result {
    Ok(())
}

// fn function2() -> io::Result<()> {
//     // --snip--
//     Ok(())
// }

//或者使用use关键词来区分Result
fn function2() -> IoResult<()> {
    Ok(())
}
fn main() {
    eat_at_restaurant();
    hosting::add_to_waitlist();
}
