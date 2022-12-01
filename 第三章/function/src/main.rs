/*
 * @Author: wulongjiang
 * @Date: 2022-12-01 22:19:14
 * @LastEditors: wulongjiang
 * @LastEditTime: 2022-12-01 23:13:24
 * @Description:函数部分
 * @see:https://kaisery.github.io/trpl-zh-cn/ch03-03-how-functions-work.html
 */

//函数
//Rust 种的函数名和变量名使用snake case 风格规范 所有字母都是小写并使用下划线分隔单词。
//主函数
fn main() {
    println!("Hello, world!");

    //注意，源码中 another_function 定义在 main 函数 之后；也可以定义在之前。
    //Rust 不关心函数定义所在的位置，只要函数被调用时出现在调用之处可见的作用域内就行。
    another_function(5);
    user_info(18, '男');
    learn_difference();

    let five = five();
    let six = six();
    println!("five:{five},six:{six}")
}

//函数可以定义参数（parameters），参数是特殊的变量是函数签名的一部分。当函数拥有参数时（形参），可以为这些参数提供具体的值（实参）。
//这些具体值被称为参数（arguments），但是在日常交流中，人们倾向于不区分使用 parameter 和 argument 来表示函数定义中的变量或调用函数时传入的具体值。

//在函数签名中，必须 声明每个参数的类型。 当定义多个参数时，使用逗号分隔
//我们增加了一个i32类型的参数x
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn user_info(age: i32, gender: char) {
    println!("userinfo: age:{age},gender:{gender}");
}

//语句和表达式
//！！！！函数体由一系列的语句和一个可选的结尾表达式构成。 目前为止我们提到的函数还不包含结尾表达式。
//Rust时一门基于表达式(expression-based)的语言，这是一个需要理解的（不同于其他语言）的重要区别。
//其他语言并没有这样的区别，所以让我们看看语句与表达式有什么区别以及这些区别是如何影响函数体的。

//语句（Statements）是执行一些操作但不返回值的指令。 函数定义也是语句，
//表达式（expressions）计算并产生一个值。

fn learn_difference() {
    //使用let 关键字创建变量并绑定一个值是一个语句。
    let y = 9; //表达式可以是语句的一个部分 其中9就是表达式 ,它计算出来的值是9

    // 语句不返回值 所以不能将let语句赋值给另一个变量。
    // let x = (let z = 3);//报错expected expression, found `let` statement（预期一个表达式，但是却发现是个let语句）
    //let y = 6 语句并不返回值，所以没有可以绑定到 x 上的值。这与其他语言不同，例如 C 和 Ruby，它们的赋值语句会返回所赋的值。
    //在这些语言中，可以这么写 x = y = 6，这样 x 和 y 的值都是 6；Rust 中不能这样写。

    //表达式可以是语句的一个部分
    //函数调用是一个表达式。宏调用是一个表达式。用大括号创建的一个新的块作用域也是一个表达式
    let scope = {
        let a = 3;
        a + 1 //！！！！！没有;注意 如果有;就是语句了
    };
    //是一个代码块，它的值是 4。这个值作为 let 语句的一部分被绑定到 y 上。注意 x+1 这一行在结尾没有分号，
    //与你见过的大部分代码行不同。表达式的结尾没有分号。
    //如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。在接下来探索具有返回值的函数和表达式时要谨记这一点。
    println!("The value of scope is: {scope}");
}

//具有返回值的函数
//函数可以向调用它的代码返回值。我们并不对返回值命名，但要在箭头（->）后声明它的类型。
//在 Rust 中，函数的返回值等同于函数体最后一个表达式的值。
//使用 return 关键字和指定值，可从函数中提前返回；但大部分函数隐式的返回最后的表达式。
fn five() -> i32 {
    println!("five函数运行");
    5
}

fn six() -> i32 {
    println!("six函数运行");
    return 6;
}
