/*
 * @Author: wlj
 * @Date: 2022-12-16 10:32:40
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-16 17:39:37
 * @Description: 闭包：可以捕获环境的匿名函数
 * @see:https://kaisery.github.io/trpl-zh-cn/ch13-01-closures.html
 */

//Rust的闭包（closures）是可以保存在一个变量中或作为参数传递给其他函数的匿名函数。可以在一个地方创建闭包，然后再不同的上下文中执行闭包运算。
//不同于函数，闭包允许捕获被定义时所在作用域中的值。我们将展示闭包的这些功能如何复用代码和自定义行为。

//使用闭包创建行为的抽象
//场景：我们在一个通过app生成自定义健身计划的初创企业工作。其后端使用Rust编写，而生成健身计划的算法需要考虑很多不同的因素，比如用户的年龄，身体质量指数(Body Mass Index)、
//用户喜好、最近的健身活动和用户指定的强度系数。本例中实际的算法并不重要，重要的是这个计算只花费几秒钟。我们只希望在需要时调用算法，并且只希望调用一次，这样就不会让用户等太久

//这里将通过调用 simulated_expensive_calculation 函数来模拟调用假定的算法，会打印出 calculating slowly...，等待两秒，
use std::thread;
use std::time::Duration;
/**
 * @description:
 * @param {u32} intensity
 * @return {*}
 */
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

//首先检查用户需要低强度（由小于 25 的系数表示）锻炼还是高强度（25 或以上）锻炼
// fn generate_workout(intensity: u32, random_number: u32) {
//     if intensity < 25 {
//         //低强度锻炼计划会根据由 simulated_expensive_calculation 函数所模拟的复杂算法建议一定数量的俯卧撑和仰卧起坐。
//         println!(
//             "Today,do {} pushups!",
//             simulated_expensive_calculation(intensity)
//         );
//         println!(
//             "Next, do {} situps!",
//             simulated_expensive_calculation(intensity)
//         );
//     } else {
//         //如果用户需要高强度锻炼，这里有一些额外的逻辑：如果 app 生成的随机数刚好是 3，app 相反会建议用户稍做休息并补充水分。
//         //如果不是，则用户会从复杂算法中得到数分钟跑步的高强度锻炼计划。
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 simulated_expensive_calculation(intensity)
//             );
//         }
//     }
// }
//现在这份代码能够应对我们的需求了，但数据科学部门的同学告知我们将来会对调用 simulated_expensive_calculation 的方式做出一些改变。
// 为了在要做这些改动的时候简化更新步骤，我们将重构代码来让它只调用 simulated_expensive_calculation 一次
// 。同时还希望去掉目前多余的连续两次函数调用，
// 并不希望在计算过程中增加任何其他此函数的调用。也就是说，我们不希望在完全无需其结果的情况调用函数，不过仍然希望只调用函数一次。

//使用函数重构
// fn generate_workout(intensity: u32, random_number: u32) {
//     let expensive_result = simulated_expensive_calculation(intensity); //缺点每次进来都得马上调用一次 random_number == 3也要调用

//     if intensity < 25 {
//         println!("Today, do {} pushups!", expensive_result);
//         println!("Next, do {} situps!", expensive_result);
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!("Today, run for {} minutes!", expensive_result);
//         }
//     }
// }

//我们希望在 generate_workout 中只引用 simulated_expensive_calculation 一次，并推迟复杂计算的执行直到我们确实需要结果的时候。这正是闭包的用武之地！

//重构使用闭包储存代码
//不同于总是在if块之前调用 simulated_expensive_calculation 函数并存储其结果，我们可以定义一个闭包将其存储在变量中
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = |num| {
        //为什么闭包不需要类型注解？
        //函数中需要类型注解是因为他们是暴露给用户的显示接口的一部分。
        //严格定义这些接口对于保证所有人都认同函数使用和返回值的类型来说是很重要的。
        //但是闭包并不用于这样暴露在外的接口：他们存储在变量中被使用，不用命名他们或暴露给库的用户调用
        //闭包通常很短，并只关联于小范围的上下文而非任意情境。在这些有限制的上下文中，编译器能可靠的推断参数和返回值的类型，类似于它是如何能够推断大部分变量的类型一样
        //类似于变量，如果相比严格的必要性你更希望增加明确性并变得更啰嗦，可以选择增加类型注解；
        //如果闭包只有一行 let add_one_v4 = |x| x + 1  ;可以简写成这样
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    }; //闭包的定义是在expensive_result赋值的=后面。闭包的定义以一对竖线（|）开始，在竖线中指定闭包的参数；之所以选择这个语法
       //是因为它与Smalltalk和Ruby的闭包定义类似。这个闭包有一个参数num；如果有多于一个参数，可以使用逗号分割，比如|params1,params2|。
       //参数之后是存放闭包的大括号 ———— 如果闭包体中只有一行没有分号（正如函数体一样），所以闭包体最后一行（num）的返回值作为调用闭包时的返回值
       //注意这个let 语句意味着expensive_closure 包含一个匿名函数的 定义，不是调用匿名函数的 返回值。
    if intensity < 25 {
        //此时还是需要调用两次
        //解决办法创建一个变量存储闭包
        println!("Today, do {} pushups!", expensive_result(intensity));
        println!("Next, do {} situps!", expensive_result(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result(intensity));
        }
    }

    //代码中intensity < 25 扔吧 慢计算闭包 调用了比所需更多的次数。解决这个问题的一个方法时在全部代码中的每一个需要多个慢计算闭包结果的地方，可以
    //将结果保存进变量以便复用，这样就可以使用变量而不是再次调用闭包。但是这样就会有很多重复的保存结果变量的地方。

    //幸运的是，还有另一个可用的方案。可以创建一个存放闭包和调用闭包结果的结构体。该结构体只会在需要结果时执行闭包，并会缓存结果，这样余下的代码就
    //不必再负责保存结果并可以复用该值。你可能见过这种模式memoization或lazy evaluation（惰性求值）。

    //为了让结构体存放闭包，我们需要指定闭包的类型，

}

fn main() {
    //一个来自用户的 intensity 数字，请求健身计划时指定，它代表用户喜好低强度还是高强度健身。
    let simulated_user_specified_value = 20;
    //一个随机数，其会在健身计划中生成变化。
    let simulated_random_number = 5;

    //main 函数包含了用于 generate_workout 函数的模拟用户输入和模拟随机数输入
    generate_workout(simulated_user_specified_value, simulated_random_number);
}
