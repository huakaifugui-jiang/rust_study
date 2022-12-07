/*
 * @Author: wlj
 * @Date: 2022-12-06 15:48:27
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-07 11:31:26
 * @Description: 枚举的定义
 * @see:https://kaisery.github.io/trpl-zh-cn/ch06-01-defining-an-enum.html
 */

//结构体给予你将字段和数据聚合在一起的方法，像Rectangle结构体有width和height两个字段。而枚举给予你将一个值成为一个集合之一的方法。
//比如，我们想让 Rectangle 是一些形状的集合，包含 Circle 和 Triangle 。为了做到这个，Rust提供了枚举类型。
//让我们看看一个需要诉诸于代码的场景，来考虑为何此时使用枚举更为合适且实用。假设我们要处理 IP 地址。目前被广泛使用的两个主要 IP 标准：IPv4（version four）和 IPv6（version six）。
//这是我们的程序可能会遇到的所有可能的 IP 地址类型：所以可以 枚举 出所有可能的值，这也正是此枚举名字的由来。

//任何地址要么是IPv4的要么是IPv6的，而且不能两者都是IP地址的这个特性使得枚举数据结构非常适合这个场景，
//因为枚举值只可能是其中一个成员。IPv4和IPv6从根本上讲仍是IP地址，所以当代码再处理适用于任何类型的IP地址时应该把他们当作相同的类型。

//可以通过在代码中定义一个 IpAddrKind 枚举来表现这个概念并列出可能的 IP 地址类型，V4 和 V6。这被称为枚举的 成员（variants）：
#[derive(Debug)]
enum IpAddrKind {
    V4, //这是一个值
    V6,
}

//但是现在还没有一个能存储实际ip地址 数据 的方法；所以我们可以创建一个结构体
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

//我们可以使用一种更简洁的方式来表达相同的概念 用枚举代替结构体
#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String),
}

//如果我们想要将V4地址存储为四个u8值而V6地址仍然表现为一个String，这就不能使用结构体了。枚举则可以轻易处理这个情况。
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn learn_enum() {
    //创建 IpAddrKind 实例
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    //注意枚举的成员位于其标识符的命名空间中，并使用两个冒号分开。这么设计的益处时现在 IpAddrKind::V4 和 IpAddrKind::V6 都是 IpAddrKind 类型的
    //例如，接着可以定义一个函数来获取任何 IpAddrKind：
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    dbg!(four);

    //创建结构体实例
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    dbg!(&home);
    //我们也可以使用另一种更加简洁的方式来表达相同的概念
    let home2 = IpAddr2::V4(String::from("127.0.0.2"));
    let loopback2 = IpAddr2::V6(String::from("::2")); //这是一个获取String参数并返回IpAddr类型实例的函数调用
    route2(&home2);
    dbg!(&home2);
}

fn route(ip_kind: IpAddrKind) {}
fn route2(ip_kind: &IpAddr2) {}

//这些代码展示了使用枚举来存储两种不同 IP 地址的几种可能的选择。然而，事实证明存储和编码 IP 地址实在是太常见了以致标准库提供了一个开箱即用的定义！让我们看看标准库是如何定义 IpAddr 的：
//它正有着跟我们定义和使用的一样的枚举和成员，不过它将成员中的地址数据嵌入到了两个不同形式的结构体中，它们对不同的成员的定义是不同的：
// struct Ipv4Addr {
//     // --snip--
// }

// struct Ipv6Addr {
//     // --snip--
// }

// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

//内嵌了许多多种多样类型的枚举：
#[derive(Debug)]
enum Message {
    Quit,                       //没有关联任何数据
    Move { x: i32, y: i32 },    //类似结构体包含命名字段
    Write(String),              //包含一个单独的String
    ChangeColor(i32, i32, i32), //包含三个i32
}

//这相当于定义了许多个结构体
// struct QuitMessage; // 类单元结构体
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // 元组结构体
// struct ChangeColorMessage(i32, i32, i32); // 元组结构体

//如果我们使用不同的结构体，由于它们都有不同的类型，我们将不能像使用示例 6-2 中定义的 Message 枚举那样，
//轻易的定义一个能够处理这些不同类型的结构体的函数，因为枚举是单独一个类型。
//我们也可以用impl 定义方法。
impl Message {
    fn call(&self) {
        dbg!(&self);
    }
}

fn learn_enum_impl() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

//Option枚举和其相对于空值的优势
//Option是标准库定义的一个枚举。Option类型应用广泛因为它编码了一个非常普遍的场景，即一个值要么有值要么没值。并且它被包含在了prelude（所以你不必显示得引入作用域就可以直接使用）。
//例如，如果请求一个包含项的列表的第一个值，会得到一个值，如果请求一个空的列表，就什么也不会得到。
//从类型系统的角度来表达这个概念就意味着编译器需要检查是否处理了所有应该处理的情况，这样就可以避免在其他编程语言中非常常见的 bug。
//Rust并没有很多其他语言中有的空值功能。空值(Null)是一个值，它代表没有值。在有空值的语言中，变量总是这两种状态之一：空值和非空值。
//空值的问题在于当你尝试像一个非空值那样使用一个空值，会出现某种形式的错误。因为空和非空的属性无处不在，非常容易出现这类错误。
//然而，空值尝试表达的概念仍然是有意义的：空值是一个因为某种原因目前无效或缺失的值。
//但是没有空值的Rust 是如何表示它的呢？ 它通过Option<T> 这是一个可以编码存在或者不存在的的概念的枚举。，它位于标准库中。
// enum Option<T> {
//     None,
//     Some(T),
// }
//Option<T> 枚举是如此有用以至于它甚至被包含在了 prelude 之中，你不需要将其显式引入作用域。另外，它的成员也是如此，可
//以不需要 Option:: 前缀来直接使用 Some 和 None。即便如此 Option<T> 也仍是常规的枚举，Some(T) 和 None 仍是 Option<T> 的成员。
//<T> 语法是一个我们还未讲到的 Rust 功能。它是一个泛型类型参数，第十章会更详细的讲解泛型。
//所有你需要知道的就是 <T> 意味着 Option 枚举的 Some 成员可以包含任意类型的数据，同时每一个用于 T 位置的具体类型使得 Option<T> 整体作为不同的类型。
fn learn_option_enum() {
    let some_number = Some(5); //Option<i32>类型
    let some_char = Some('e'); //Option<char>类型
    let mut absent_numer: Option<i32> = None; //Option<i32>类型
                                              // println!("{} {} ",some_number,some_char);
    dbg!(some_number, some_char, absent_numer);
    test(some_number);
    //那么 Option<T> 为什么就比空值要好呢？比如下面
    absent_numer = some_number;
    // absent_numer = 5//报错expected enum `Option`, found integer
    //let sum = some_number + 8;//报错 cannot add `{integer}` to `Option<i32>`

    //所以在对Option<T>进行T的换算前 必须将其转换为T。通常这能帮助我们捕获到空值最常见的问题之一：假设某值不为空实际上不为空的情况。 有点抽象
    //为了拥有一个可鞥为空的值，你必须要显示得将其放入对应类型的Option<T>中。接着当使用这个值时，必须明确的处理值为空的情况。只要一个值不是Option<T>类型
    //你就可以安全的认定它的值不为空。

    dbg!(some_number, some_char, absent_numer);

    //那么当有一个 Option<T> 的值时，如何从 Some 成员中取出 T 的值来使用它呢？Option<T> 枚举拥有大量用于各种情况的方法：
    //文档：https://doc.rust-lang.org/std/option/enum.Option.html
    //你可以查看它的文档。熟悉 Option<T> 的方法将对你的 Rust 之旅非常有用。
    //总的来说，为了使用 Option<T> 值，需要编写处理每个成员的代码。你想要一些代码只当拥有 Some(T) 值时运行，允许这些代码使用其中的 T。
    //也希望一些代码在值为 None 时运行，这些代码并没有一个可用的 T 值。
    //match 表达式就是这么一个处理枚举的控制流结构：它会根据枚举的成员运行不同的代码，这些代码可以使用匹配到的值中的数据。
}

fn test(params: Option<i32>) {}

fn main() {
    learn_enum();
    learn_enum_impl();
    learn_option_enum()
}
