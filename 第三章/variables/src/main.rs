/*
 * @Author: wlj
 * @Date: 2022-12-01 09:43:57
 * @LastEditors: wulongjiang
 * @LastEditTime: 2022-12-02 22:03:08
 * @Description: 变量和可变性
 * @see:https://kaisery.github.io/trpl-zh-cn/ch03-01-variables-and-mutability.html
 */

//在Rust中数据默认是不可变的(immutable).
// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 8;
//     println!("The value of x is: {x}");
//     //此时我们运行cargo run 会看到 报错警告 cannot assign twice to immutable variable (不能对一个不可变数据赋值两次)
// }

fn main() {
    let mut x = 5; //通过mut（mutable 可变的） 我们可以将x(不可变变量) 转为 可变变量 此时 运行代码就不会报错
    println!("The value of x is: {x}");
    //因为是静态强类型语言 所以变量重新赋值也必须是数字类型
    x = 8;
    println!("The value of x is: {x}");

    //常量(constants) 是绑定到一个名称的不允许改变的值,它总是不能变的。
    //Rust 对常量的命名约定是在单词之间使用全大写加下划线。
    //常量与变量的区别
    // 1.不允许对常量使用mut
    // 2.常量的声明用const关键字而不是let关键字。
    // 3.常量必须声明值的类型。
    // 4.常量可以在任何作用域中声明，包括全局作用域.
    // 5.常量只能被设置为常量表达式，而不可以是其他任何只能在运行时计算出的值。
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    //常量可以是一个数值计算表达式 使我们可以选择以更容易理解和验证的方式写出此值，而不是将此常量设置为值10,800。
    //有关声明常量时可以使用哪些操作的详细信息，请参阅 Rust Reference 的常量求值部分 https://doc.rust-lang.org/reference/const_eval.html。
    //将遍布于应用程序中的硬编码(直接编译成二进制于可执行文件中)（可以理解为常量是硬编码的）值声明为常量，能帮助后来的代码维护人员了解值的意图。如果将来需要修改硬编码值，也只需修改汇聚于一处的硬编码值。
    println!("The value of x is: {THREE_HOURS_IN_SECONDS}");

    //隐藏（Shadowing）
    let y = 5; //首先将5 赋值给x
    let y = y + 1; //通过let 创建了一个新的变量(之前的y被隐藏了) 并获取初始值加1 ， 这样y就变成了6
    {
        let y = y * 2; //隐藏了之前的值 并将之前的值乘2
        println!("The value of y in the inner scope is: {y}"); //12
    }

    println!("the value of y is:{y}"); //6

    //隐藏与mut的区别
    //1.当我们对变量重新进行赋值时如果没有使用 let 关键字，就会导致编译时错误。通过使用 let，我们可以用这个值进行一些计算，不过计算完之后变量仍然是不可变的。
    //2.隐藏可以改变变量的类型
    let spaces = "   "; //字符串类型
    let spaces = spaces.len(); //获取字符串的长度 （数字类型）
    println!("The value of spaces is:{spaces}");
}
