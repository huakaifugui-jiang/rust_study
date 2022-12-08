/*
 * @Author: wulongjiang
 * @Date: 2022-12-07 22:35:20
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-08 10:01:02
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
//4.模块中代码路径：
//一旦一个模块是你crate的一部分，你可以在隐私规则运行的前提下，从一个crate内的任意地方，通过代码路径引用该模块的代码。
//举例而言，一个garden vegetables模块下的Asparagus类型可以在crate::garden::vegetables::Asparagus被找到。
//5.私有vs公用：
//一个模块里的代码默认对其父模块私有。为了使一个模块公用，应当在声明的时使用pub mod 代替 mod。为了使一个公用模块内部的成员公用，应当在声明前使用pub。
//6.use关键词
//在一个作用域内，use关键词创建了一个成员的快捷方式，用来减少长路径的重复。在任何可以引用crate::garden::vegetables::Asparagus的作用域，你可以通过
//use crate::garden::vegetables::Asparagus; 创建一个快捷方式，然后你就可以在作用域中只写Asparagus来使用该类型。 可以看看第二章的猜游戏案例

//我们可以看看我们创建的这个项目 spoce_and_privacy 这是一个名为 spoce_and_privacy 的二进制crate（其实也是一个package）来说明这些规则。
// spoce_and_privacy
// ├── Cargo.lock
// ├── Cargo.toml
// └── src
//     ├── garden
//     │   └── vegetables.rs
//     ├── garden.rs
//     └── main.rs //这是根文件

use crate::garden::vegetables::Asparagus;

pub mod garden; //告诉编译器应该在src/garden.rs文件中发现代码

fn main() {
    garden::garden();

    let plant = Asparagus {};
    dbg!(plant);
}

//-------在模块中对相关代码进行分组
//模块让我们可以将一个crate中的代码进行分组，以提高可读性与重用性。模块还可以控制项的私有性，即项是可以被外部代码使用的(public)，
//还是作为一个内部实现的内容，不能被外部代码使用（private）。
//例如：餐馆中会有一些地方被称之为 前台（front of house），还有另外一些地方被称之为 后台（back of house）。前台是招待顾客的地方，
//在这里，店主可以为顾客安排座位，服务员接受顾客下单和付款，调酒师会制作饮品。后台则是由厨师工作的厨房，洗碗工的工作地点，以及经理做行政工作的地方组成。

//我们可以将函数放置到嵌套的模块中，来使我们的crate结构与实际的餐厅结构相同。通过执行cargo new --lib restaurant，来创建一个新的名为 restaurant的库
