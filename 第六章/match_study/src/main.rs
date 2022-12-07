/*
 * @Author: wlj
 * @Date: 2022-12-07 11:34:23
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-07 16:04:52
 * @Description: match 控制流结构
 * @see:https://kaisery.github.io/trpl-zh-cn/ch06-02-match.html
 */

//match 是 RUST 的极为强大的控制流运算符,它允许我们将一个值与一系列的模式 一 一进行对比，并且根据相匹配的模式执行相应的代码。
//模式可以由字面量、变量、通配符和许多其他内容构成；第十八章会涉及到所有不同种类的模式以及它们的作用
//match 的力量来源于模式的表现力以及编译器检查，它确保了所有可能的情况都得到处理。
//可以把match表达式想象成某种硬币分类器：硬币滑入有着不同大小孔洞的轨道，每一个硬币都会掉入符合它大小的孔洞。
//同样地，值也会通过 match 的每一个模式，并且在遇到第一个 “符合” 的模式时，值会进入相关联的代码块并在执行中被使用。
//个人感觉很像switch 写法，但是不同的它是一个个按顺序对比下来的

//！！！！注意 match分支时一个表达式
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn learn_match(coin: Coin) -> u8 {
    //match是表达式 所以可以直接返回
    match coin {
        Coin::Penny => {
            println!("Penny");
            1
        }
        Coin::Nickel => {
            println!("Nickel");
            5
        }
        Coin::Dime => {
            println!("Dime");
            10
        }
        Coin::Quarter => {
            println!("Quarter");
            25
        }
    }
}

//----------绑定值的模式
//匹配分支的另一个有用的功能是可以绑定匹配的模式的部分值。这也就是如何从枚举成员中提取值的。
#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
}
// 作为一个例子，让我们修改枚举的一个成员来存放数据。1999 年到 2008 年间，美国在 25 美分的硬币的一侧为 50 个州的每一个都印刷了不同的设计。其他的硬币都没有这种区分州的设计，
// 所以只有这些 25 美分硬币有特殊的价值。可以将这些信息加入我们的 enum，通过改变 Quarter 成员来包含一个 State 值
enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn learn_bind_match(coin: Coin2) -> u8 {
    //想象一下如果我们要收集所有50个州的25美分硬币。在根据硬币类型分类零钱的同时，也可以报告出每个25美分硬币所对应的州的名称
    match coin {
        Coin2::Penny => {
            println!("Penny");
            1
        }
        Coin2::Nickel => {
            println!("Nickel");
            5
        }
        Coin2::Dime => {
            println!("Dime");
            10
        }
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

//---------匹配Option<T>
fn learn_option_match(x: Option<i32>) -> Option<i32> {
    match x {
        //匹配是穷尽（exhaustive）的 这些分支必须覆盖了所有的可能性
        //如果将None => None,注释掉 那么Rust将会报错  pattern `None` not covered 所以我们必须穷举到最后的可能性来使代码有效
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let coin = learn_match(Coin::Penny);
    println!("coin:{}", coin);

    //----------绑定值的模式
    let coin2 = learn_bind_match(Coin2::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = learn_option_match(five);
    let none = learn_option_match(None);
    dbg!(six, none);

    //---------通配模式和_占位符

    //我们希望对一些特定的值采取特殊操作，而对其他的值采取默认操作。想象我们正在玩一个游戏，如果你掷出骰子的值为 3，角色不会移动，而是会得到一顶新奇的帽子。如果你掷出了 7，你的角色将失去新奇的帽子。对于其他的数值，你的角色会在棋盘上移动相应的格子。
    //这是一个实现了上述逻辑的 match，骰子的结果是硬编码而不是一个随机值，其他的逻辑部分使用了没有函数体的函数来表示，实现它们超出了本例的范围
    let dice_roll = 9;
    match dice_roll {
        3 => println!("3"),
        7 => println!("7"),
        //other => println!("other : {}", other), //通配分支  最后一个分支则涵盖了所有其他可能的值，模式是我们命名为 other 的一个变量 而且我们必须将这个分支放到最后
        //因为match是按顺序匹配的，如果我们在通配分支后添加其他分支，Rust会警告我们，此后的分支永远不会被匹配到
        // _ => println("_"), //当我们不想使用这个变量的时候 我们可以用_代替other
        _ => (), //可以使用空元组作为_分支的代码代表 无事发生
    }
}
