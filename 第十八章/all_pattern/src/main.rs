/*
 * @Author: wlj
 * @Date: 2022-12-22 16:52:31
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-22 17:25:01
 * @Description: https://kaisery.github.io/trpl-zh-cn/ch18-03-pattern-syntax.html
 */
// 通过本书我们已领略过许多不同类型模式的例子。在本节中，我们收集了模式中所有有效的语法，并讨论为什么以及何时你可能要使用这些语法。
//建议直接看https://kaisery.github.io/trpl-zh-cn/ch18-03-pattern-syntax.html#%E5%BF%BD%E7%95%A5%E6%A8%A1%E5%BC%8F%E4%B8%AD%E7%9A%84%E5%80%BC
fn main() {
    // 匹配字面值
    // 如第六章所示，可以直接匹配字面值模式。如下代码给出了一些例子：
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    // 这段代码会打印 one 因为 x 的值是 1。如果希望代码获得特定的具体值，则该语法很有用。
    // 匹配命名变量
    // 命名变量是匹配任何值的不可反驳模式，这在之前已经使用过数次。然而当其用于 match 表达式时情况会有些复杂。
    // 因为 match 会开始一个新作用域，match 表达式中作为模式的一部分声明的变量会覆盖 match 结构之外的同名变量，与所有变量一样。
    // 在示例 18-11 中，声明了一个值为 Some(5) 的变量 x 和一个值为 10 的变量 y。。接着在值 x 上创建了一个 match 表达式。观察匹配分支中的模式和结尾的 println!，并在运行此代码或进一步阅读之前推断这段代码会打印什么。

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
    // 让我们看看当 match 语句运行的时候发生了什么。第一个匹配分支的模式并不匹配 x 中定义的值，所以代码继续执行。
    // 第二个匹配分支中的模式引入了一个新变量 y，它会匹配任何 Some 中的值。
    // 因为我们在 match 表达式的新作用域中，这是一个新变量，而不是开头声明为值 10 的那个 y。这个新的 y 绑定会匹配任何 Some 中的值，在这里是 x 中的值。
    // 这个值是 5，所以这个分支的表达式将会执行并打印出 Matched, y = 5。如果 x 的值是 None 而不是 Some(5)，头两个分支的模式不会匹配，所以会匹配下划线。
    // 这个分支的模式中没有引入变量 x，所以此时表达式中的 x 会是外部没有被覆盖的 x。在这个假想的例子中，match 将会打印 Default case, x = None。

    // 一旦 match 表达式执行完毕，其作用域也就结束了，同理内部 y 的作用域也结束了。最后的 println! 会打印 at the end: x = Some(5), y = 10。
    // 为了创建能够比较外部 x 和 y 的值，而不引入覆盖变量的 match 表达式，我们需要相应地使用带有条件的匹配守卫（match guard）我们稍后将在 “匹配守卫提供的额外条件” 这一小节讨论匹配守卫。

    // 多个模式
    // 在 match 表达式中，可以使用 | 语法匹配多个模式，它代表 或（or）的意思。
    // 例如，如下代码将 x 的值与匹配分支相比较，第一个分支有 或 选项，意味着如果 x 的值匹配此分支的任一个值，
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    // 通过 ..= 匹配值的范围
    // ..= 语法允许你匹配一个闭区间范围内的值。
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    // 如果 x 是 1、2、3、4 或 5，第一个分支就会匹配。
    // 这相比使用 | 运算符表达相同的意思更为方便；相比 1..=5，使用 | 则不得不指定 1 | 2 | 3 | 4 | 5。相反指定范围就简短的多，特别是在希望匹配比如从 1 到 1000 的数字的时候！
    // 范围只允许用于数字或 char 值，因为编译器会在编译时检查范围不为空。char 和 数字值是 Rust 仅有的可以判断范围是否为空的类型。
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
    // 解构并分解值
    // 也可以使用模式来解构结构体、枚举和元组，以便使用这些值的不同部分。让我们来分别看一看。
    // 解构结构体
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = Point { x: 0, y: 7 };
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
    // 第一个分支通过指定字段 y 匹配字面值 0 来匹配任何位于 x 轴上的点。此模式仍然创建了变量 x 以便在分支的代码中使用。
    // 类似的，第二个分支通过指定字段 x 匹配字面值 0 来匹配任何位于 y 轴上的点，并为字段 y 创建了变量 y。。第三个分支没有指定任何字面值，所以其会匹配任何其他的 Point 并为 x 和 y 两个字段创建变量。
    // 在这个例子中，值 p 因为其 x 包含 0 而匹配第二个分支，因此会打印出 On the y axis at 7。

    // 解构枚举
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}
