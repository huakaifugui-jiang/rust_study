/*
 * @Author: wlj
 * @Date: 2022-12-02 08:28:04
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-02 09:50:33
 * @Description:控制流学习
 * @see:https://kaisery.github.io/trpl-zh-cn/ch03-05-control-flow.html
 */

//根据条件是否为真来决定是否执行某些代码，以及根据条件是否为真来重复运行一段代码的能力是大部分编程语言的基本组成部分。
//Rust 代码中最常见的用来控制执行流的结构是 if 表达式和循环。

fn main() {
    //if 表达式 （！！！注意 这是一个表达式
    //if 表达式中与条件关联的代码块有时被叫做 arms 和大多数的语言一样 if可以和else一起使用
    //但是要注意的是rust不像JavaScript会自动的将非布尔值专为布尔值，必须显示地使用布尔值作为if的条件
    //因为他是表达式 所以我们可以在let语句的右侧使用它。
    let condition = true;
    let num = if condition { 5 } else { 6 }; //条件不用()包裹起来. 两个代码块表达式的值类型需一致

    //需要记住的是： 代码块的值是其最后一个表达式的值，而数字本身就是一个表达式。
    //这也就意味着 if和else语句的代码块 的 最后一个表达式的值的类型要一致。
    println!("The number of value is :{num}");

    //循环重复执行
    //Rust 有三种循环：loop、while和for。
    //loop 可以用break 表达式（！！！表达式）来停止循环的返回 ，！！并且它会被停止的循环返回
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("The result is {result}");

    //循环标签 ：在多个循环之间消除歧义。
    let mut count = 0;
    'counting_up: loop {
        let mut remain = 10;

        loop {
            println!("remain = {remain}");
            if remain == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remain -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    // 打印结果
    // remain = 10 首先第一次counting_up循环 第一次内部循环内部循环 打印后 -1
    // remain = 9 第一次counting_up循环 第二次内部循环 打印后 发现=9 跳出内部循环后 count+1 =1
    // remain = 10  因为没有跳出counting_up循环 进行第二次counting_up循环 后进入第一次内循环 打印10后 remain-1
    // remain = 9   进行第二次counting_up循环的第二次内循环 打印9后 跳出内循环 后count+1 =2
    // remain = 10  进入第三次counting_up循环 进入第一次内循环 打印10后 发现 count =2 跳出外循环
    // End count = 2    打印count

    //while条件循环 在程序中计算循环的条件也很常见。当条件为真，执行循环。当条件不再为真，调用 break 停止循环。这个循环类型可以通过组合 loop、if、else 和 break 来实现；
    //然而，这个模式太常用了，Rust 为此内置了一个语言结构，它被称为 while 循环

    let mut while_num = 0;
    while while_num <= 3 {
        println!("while_num:{while_num}!");
        while_num += 1;
    }
    println!("End While!!! while_num:{while_num}");

    //for 循环 遍历集合
    let arr = [10, 11, 12, 13, 14];

    for element in arr {
        println!("the value is: {element}");
    }
    //大部分 Rustacean(使用rust的人) 也会使用 for 循环。这么做的方式是使用 Range，它是标准库提供的类型，用来生成从一个数字开始到另一个数字之前结束的所有数字的序列
    //(1..4)是Range； 它还使用了一个我们还未讲到的方法，rev，用来反转 range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
