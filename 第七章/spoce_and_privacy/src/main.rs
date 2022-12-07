/*
 * @Author: wulongjiang
 * @Date: 2022-12-07 22:35:20
 * @LastEditors: wulongjiang
 * @LastEditTime: 2022-12-07 23:14:18
 * @Description:定义模块来控制作用域与私有性
 * @see:https://kaisery.github.io/trpl-zh-cn/ch07-02-defining-modules-to-control-scope-and-privacy.html
 * @FilePath: \rust_study\第七章\spoce_and_privacy\src\main.rs
 */
//在本节，我们将讨论模块和其它一些关于模块系统的部分，如允许你命名项的 路径（paths）；用来将路径引入作用域的 use 关键字；
//以及使项变为公有的 pub 关键字。我们还将讨论 as 关键字、外部包和 glob 运算符。现在，让我们把注意力放在模块上！

//模块小抄
//我们这里提供一个简单的参考，用来解释模块、路径、use关键词和pub关键词 如何再编译器中工作，和大部门开发者如何组织他们的代码。
//1. 从crate根节点开始 ：
//当编译一个crate,编译器首先在crate根文件（通常，对于一个库crate而言是src/lib.rs 大伙可以查看一下（rand库https://github.com/rust-random/rand/tree/master/src），
//对于一个二进制crate而言是src/main.rs(也就是我们的程序))中寻找需要被编译的代码。
//2.声明模块：
//在crate根文件中，你可以声名一个模块；比如你用 mod(备注：modules缩写) garden 声名了一个叫做 garden的模块。编译器会在下列路径中寻找模块代码：
//内联，在大括号中，当mod garden后方不是一个分号而是一个大括号
//在文件src/garden.rs
//在文件src/garden/mod.rs
//3.声明子模块：
//在除了crate根节点以外的其他文件中，你可以定义子模块。比如你可能在src/garden.rs中定义了mod vegetables;编译器会在以父模块命名的目录中寻找子模块代码：
//内联，在大括号中，当mod vegetables 后方不是一个分号而是一个大括号
//在文件 src/garden/vegetables.rs
//在文件 src/garden/vegetables/mod.rs

fn main() {
    println!("Hello, world!");
}
