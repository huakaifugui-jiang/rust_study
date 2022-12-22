/*
 * @Author: wlj
 * @Date: 2022-12-22 15:58:00
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-22 16:35:00
 * @Description: 所有可能会用到模式的位置
 * @see:https://kaisery.github.io/trpl-zh-cn/ch18-01-all-the-places-for-patterns.html
 */

fn main() {
    //match 分支
    //如第六章所讨论的，一个模式常用的位置是 match 表达式的分支。
    //在形式上 match 表达式由 match 关键字、用于匹配的值和一个或多个分支构成，这些分支包含一个模式和在值匹配分支的模式时运行的表达式：
    // match VALUE {
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    // }
    //match 表达式必须是 穷尽（exhaustive）的，意为 match 表达式所有可能的值都必须被考虑到。一个确保覆盖每个可能值的方法是在最后一个分支使用捕获所有的模式
    // 比如，一个匹配任何值的名称永远也不会失败，因此可以覆盖所有匹配剩下的情况。
    // 有一个特定的模式 _ 可以匹配所有情况，不过它从不绑定任何变量。这在例如希望忽略任何未指定值的情况很有用。

    // if let 条件表达式
    // 第六章讨论过了 if let 表达式，以及它是如何主要用于编写等同于只关心一个情况的 match 语句简写的。
    // if let 可以对应一个可选的带有代码的 else 在 if let 中的模式不匹配时运行。

    //下面代码展示了一系列针对不同条件的检查来决定背景颜色应该是什么。为了达到这个例子的目的，我们创建了硬编码值的变量，在真实程序中则可能由询问用户获得。

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color); //如果用户指定了中意的颜色，将使用其作为背景颜色。
    } else if is_tuesday {
        println!("Tuesday is green day!"); //如果今天是星期二，背景颜色将是绿色
    } else if let Ok(age) = age {
        //如果用户指定了他们的年龄字符串并能够成功将其解析为数字的话，我们将根据这个数字使用紫色或者橙色。
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color"); //最后，如果没有一个条件符合，背景颜色将是蓝色：
    }
    //这个条件结构允许我们支持复杂的需求。使用这里硬编码的值，例子会打印出 Using purple as the background color。
    //注意 if let 也可以像 match 分支那样引入覆盖变量：if let Ok(age) = age 引入了一个新的覆盖变量 age，它包含 Ok 成员中的值。
    //这意味着 if age > 30 条件需要位于这个代码块内部；不能将两个条件组合为 if let Ok(age) = age && age > 30
    //因为我们希望与 30 进行比较的被覆盖的 age 直到大括号开始的新作用域才是有效的。
    //if let 表达式的缺点在于其穷尽性没有为编译器所检查，而 match 表达式则检查了。如果去掉最后的 else 块而遗漏处理一些情况，编译器也不会警告这类可能的逻辑错误。

    //while let 条件循环
    //一个与 if let 结构类似的是 while let 条件循环，它允许只要模式匹配就一直进行 while 循环

    //示例 18-2 展示了一个使用 while let 的例子，它使用 vector 作为栈并以先进后出的方式打印出 vector 中的值
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    //这个例子会打印出 3、2 接着是 1。pop 方法取出 vector 的最后一个元素并返回 Some(value)。如果 vector 是空的，它返回 None。
    //while 循环只要 pop 返回 Some 就会一直运行其块中的代码。一旦其返回 None，while 循环停止。我们可以使用 while let 来弹出栈中的每一个元素。

    //for 循环
    //如同第三章所讲的，for 循环是 Rust 中最常见的循环结构，不过还没有讲到的是 for 可以获取一个模式。
    //在 for 循环中，模式是 for 关键字直接跟随的值，正如 for x in y 中的 x。
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        //这里使用 enumerate 方法适配一个迭代器来产生一个值和其在迭代器中的索引,他们位于一个元组中。
        println!("{} is at index {}", value, index);
        //第一个产生的值是元组 (0, 'a')。
    }

    //let 语句
    //在本章之前，我们只明确的讨论过通过 match 和 if let 使用模式，不过事实上也在别的地方使用过模式，包括 let 语句。
    //。例如，考虑一下这个直白的 let 变量赋值：
    let x = 5;
    // 本书进行了不下百次这样的操作，不过你可能没有发觉，这正是在使用模式！let 语句更为正式的样子如下：
    // let PATTERN = EXPRESSION;
    //像 let x = 5; 这样的语句中变量名位于 PATTERN 位置，变量名不过是形式特别朴素的模式。
    // 我们将表达式与模式比较，并为任何找到的名称赋值。所以例如 let x = 5; 的情况，x 是一个代表 “将匹配到的值绑定到变量 x” 的模式
    // 同时因为名称 x 是整个模式 这个模式实际上等于 “将任何值绑定到变量 x，不管值是什么”。
    // 为了更清楚的理解 let 的模式匹配方面的内容，考虑示例 18-4 中使用 let 和模式解构一个元组：
    let (x, y, z) = (1, 2, 3);
    // 这里将一个元组与模式匹配。Rust 会比较值 (1, 2, 3) 与模式 (x, y, z) 并发现此值匹配这个模式。
    // 在这个例子中，将会把 1 绑定到 x，2 绑定到 y 并将 3 绑定到 z
    // 你可以将这个元组模式看作是将三个独立的变量模式结合在一起。
    // 如果模式中元素的数量不匹配元组中元素的数量，则整个类型不匹配，并会得到一个编译时错误。例如，示例 18-5 展示了尝试用两个变量解构三个元素的元组，这是不行的：
    let (x, y, _) = (1, 2, 3);
    // 函数参数
    // 函数参数也可以是模式。列表 18-6 中的代码声明了一个叫做 foo 的函数，它获取一个 i32 类型的参数 x，现在这看起来应该很熟悉：

    fn foo(x: i32) {
        // code goes here
    }
    // x 部分就是一个模式！类似于之前对 let 所做的，可以在函数参数中匹配元组。列表 18-7 将传递给函数的元组拆分为值：
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);

    // 这会打印出 Current location: (3, 5)。值 &(3, 5) 会匹配模式 &(x, y)，如此 x 得到了值 3，而 y得到了值 5。
    // 因为如第十三章所讲闭包类似于函数，也可以在闭包参数列表中使用模式。
    // 现在我们见过了很多使用模式的方式了，不过模式在每个使用它的地方并不以相同的方式工作；在一些地方，模式必须是 irrefutable 的，意味着他们必须匹配所提供的任何值。
    // 在另一些情况，他们则可以是 refutable 的。接下来让我们讨论这两个概念。
}
