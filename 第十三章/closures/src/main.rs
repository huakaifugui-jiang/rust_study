/*
 * @Author: wlj
 * @Date: 2022-12-16 10:32:40
 * @LastEditors: wulongjiang
 * @LastEditTime: 2022-12-18 11:44:06
 * @Description: 闭包：可以捕获环境的匿名函数
 * @see:https://kaisery.github.io/trpl-zh-cn/ch13-01-closures.html
 */

//Rust的闭包（closures）是可以保存在一个变量中或作为参数传递给其他函数的匿名函数。可以在一个地方创建闭包，然后再不同的上下文中执行闭包运算。
//不同于函数，闭包允许捕获被定义时所在作用域中的值。我们将展示闭包的这些功能如何复用代码和自定义行为。

//使用闭包创建行为的抽象
//场景：我们在一个通过app生成自定义健身计划的初创企业工作。其后端使用Rust编写，而生成健身计划的算法需要考虑很多不同的因素，比如用户的年龄，身体质量指数(Body Mass Index)、
//用户喜好、最近的健身活动和用户指定的强度系数。本例中实际的算法并不重要，重要的是这个计算只花费几秒钟。我们只希望在需要时调用算法，并且只希望调用一次，这样就不会让用户等太久

use std::collections::HashMap;
//这里将通过调用 simulated_expensive_calculation 函数来模拟调用假定的算法，会打印出 calculating slowly...，等待两秒，
use std::cmp::Eq;
use std::hash::Hash;
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
// fn generate_workout(intensity: u32, random_number: u32) {
//     let expensive_result = |num| {
//         //为什么闭包不需要类型注解？
//         //函数中需要类型注解是因为他们是暴露给用户的显示接口的一部分。
//         //严格定义这些接口对于保证所有人都认同函数使用和返回值的类型来说是很重要的。
//         //但是闭包并不用于这样暴露在外的接口：他们存储在变量中被使用，不用命名他们或暴露给库的用户调用
//         //闭包通常很短，并只关联于小范围的上下文而非任意情境。在这些有限制的上下文中，编译器能可靠的推断参数和返回值的类型，类似于它是如何能够推断大部分变量的类型一样
//         //类似于变量，如果相比严格的必要性你更希望增加明确性并变得更啰嗦，可以选择增加类型注解；
//         //如果闭包只有一行 let add_one_v4 = |x| x + 1  ;可以简写成这样
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     }; //闭包的定义是在expensive_result赋值的=后面。闭包的定义以一对竖线（|）开始，在竖线中指定闭包的参数；之所以选择这个语法
//        //是因为它与Smalltalk和Ruby的闭包定义类似。这个闭包有一个参数num；如果有多于一个参数，可以使用逗号分割，比如|params1,params2|。
//        //参数之后是存放闭包的大括号 ———— 如果闭包体中只有一行没有分号（正如函数体一样），所以闭包体最后一行（num）的返回值作为调用闭包时的返回值
//        //注意这个let 语句意味着expensive_closure 包含一个匿名函数的 定义，不是调用匿名函数的 返回值。
//     if intensity < 25 {
//         //此时还是需要调用两次
//         //解决办法创建一个变量存储闭包
//         println!("Today, do {} pushups!", expensive_result(intensity));
//         println!("Next, do {} situps!", expensive_result(intensity));
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!("Today, run for {} minutes!", expensive_result(intensity));
//         }
//     }

//     //代码中intensity < 25 扔吧 慢计算闭包 调用了比所需更多的次数。解决这个问题的一个方法时在全部代码中的每一个需要多个慢计算闭包结果的地方，可以
//     //将结果保存进变量以便复用，这样就可以使用变量而不是再次调用闭包。但是这样就会有很多重复的保存结果变量的地方。

//     //幸运的是，还有另一个可用的方案。可以创建一个存放闭包和调用闭包结果的结构体。该结构体只会在需要结果时执行闭包，并会缓存结果，这样余下的代码就
//     //不必再负责保存结果并可以复用该值。你可能见过这种模式memoization或lazy evaluation（惰性求值）。

//     //为了让结构体存放闭包，我们需要指定闭包的类型，因为结构体定义需要知道每一个字段的类型。每一个闭包实例有其自己独有的匿名类型：也就是说
//     //即便两个闭包有着相同的签名，他们的类型仍然可以被认为是不同的。为了定义使用闭包的结构体、枚举或函数参数，需要像第十章讨论的那样使用泛型和trait bound

//     //Fn 系列 trait由标准库提供。所有的闭包都实习了trait Fn、FnMut或FnOnce中的一个。
//     //为了满足Fn trait bound 我们增加了代表了闭包所必须的参数和返回值类型的类型。
// }

//在这个例子中，闭包由一个u32的参数并返回一个u32这样所指定的trait bound 就是Fn(u32)->u32.
//如下展示了存放了闭包和一个Option结果值的Cacher结构体的定义
// struct Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     calculation: T,
//     value: Option<u32>, //字段value是Option类型的。在执行闭包之前，value将是none。如果使用了Cacher的代码请求闭包的结果，这时会执行闭包并结果存储在Value字段的
//                         //Some成员中。接着如果代码再次请求闭包的结果，这时不再执行闭包，而是会返回从存放在Some成员中的结果
// }

// impl<T> Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     //Cacher::new函数获取一个泛型参数T，它定义与impl块上下文中并与Cacher结构体有着相同的trait bound。
//     //Cacher::new 返回一个在 calculation 字段中指定闭包和在value中存放了None值的Cacher实例，因为我们还未执行闭包。
//     fn new(calculation: T) -> Cacher<T> {
//         Cacher {
//             calculation,
//             value: None,
//         }
//     }

//     //Cacher的value缓存逻辑
//     //当调用代码需要闭包的执行结果时，不同于直接调用闭包，它会调用value方法。这个方法会检查self.value是否已经有了一个Some的结果值；
//     //如果有，它返回Some中的值并不会再次执行闭包。如果self.value是None，则会调用self.calculation中存储的闭包，将结果保存到self.value以便将来使用
//     //同时返回结果值
//     fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.calculation)(arg); //此时有缺点如果arg的值变了，但是还是不会更新
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }

struct Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Hash + Eq + Copy, //为了使用get方法 说实话还没太明白 可能是有一些类型是不能用get方法的 所以加了个限制
{
    calculation: T,
    value: HashMap<E, E>,
}

impl<T, E> Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Hash + Eq + Copy,
{
    fn new(calculation: T) -> Cacher<T, E> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: E) -> E {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(random_number));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    //一个来自用户的 intensity 数字，请求健身计划时指定，它代表用户喜好低强度还是高强度健身。
    let simulated_user_specified_value = 20;
    //一个随机数，其会在健身计划中生成变化。
    let simulated_random_number = 5;

    //main 函数包含了用于 generate_workout 函数的模拟用户输入和模拟随机数输入
    generate_workout(simulated_user_specified_value, simulated_random_number);

    //闭包会捕获其环境
    //在健身计划生成器的例子中，我们只将闭包作为内联匿名函数来使用。不过闭包还有另一个函数所没有的功能：他们可以捕获其环境并访问其被定义的作用域的变量
    let x = 4;
    let equal_to_x = |z| z == x; //x并不是equal_to_x这个闭包的一个参数，但是却被允许使用变量x，因为它与qual_to_x定义于相同的作用域

    // fn equal_to_x(z: i32) -> bool {
    //     z == x //如果是函数则会报错
    // }
    let y = 4;
    assert!(equal_to_x(y));

    //当闭包从环境捕获一个值，闭包会在闭包体中存储这个值以供使用。这会使用内存并产生额外的开销，在更一般的场景中，当我们不需要闭包来捕获环境时，
    //我们并不希望产生这些开销。因为函数从未允许捕获环境，定义和使用函数也就从不会有这些额外的开销。
    //闭包可以通过三种方式捕获其环境，他们直接对应函数的三种获取参数的方式：获取所有权，可变借用和不可变借用。这三种捕获值的方式被编码为如下三个Fn trait:
    //1. FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其环境（environment）。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。
    //其名称的Once部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。

    //2. FnMut 获取可变的借用值所以可以改变其环境

    //3. Fn从其环境获取不可变的借用值。

    //当创建一个闭包时，Rust根据其如何使用环境中变量来推断我们希望如何引用环境。由于所有闭包都可以被调用至少一次，所以所有闭包都实现了FnOnce。
    //那些并没有移动被捕获变量的所有权到闭包内的闭包也实现了FnMut，而不需要对被捕获的变量进行可变访问的闭包也是实现了Fn。
    //例如上面的equal_to_x 闭包不可变的借用了x 所以具有Fn trait 因为闭包只需读取x的值

    //如果你希望强制闭包获取其使用的环境值的所有权，可以在参数列表前使用move关键字。这个技巧在将闭包传递给新线程以便将数据移动到新线程中时最为实用。
    //注意：即使其捕获的值已经被移动了，move闭包仍需要实现Fn 或 FnMut。这是因为闭包所实现的trait是由闭包所捕获了什么值而不是如何捕获所决定的。而move关键字代表了后者。

    //第十六章讨论并发时会展示更多 move 闭包的例子，不过现在这里修改了示例 13-12 中的代码（作为演示），
    // 在闭包定义中增加 move 关键字并使用 vector 代替整型，因为整型可以被拷贝而不是移动；注意这些代码还不能编译：

    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;
    //x 被移动进了闭包，因为闭包使用 move 关键字定义。接着闭包获取了 x 的所有权，同时 main 就不再允许在 println! 语句中使用 x 了。去掉 println! 即可修复问题。
    //大部分需要指定一个 Fn 系列 trait bound 的时候，可以从 Fn 开始，而编译器会根据闭包体中的情况告诉你是否需要 FnMut 或 FnOnce。
    
    // println!("can't use x here: {:?}", x); 报错

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
