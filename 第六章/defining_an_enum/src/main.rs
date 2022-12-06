/*
 * @Author: wlj
 * @Date: 2022-12-06 15:48:27
 * @LastEditors: wulongjiang
 * @LastEditTime: 2022-12-06 21:45:18
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
    let loopback2 = IpAddr2::V6(String::from("::2"));
    dbg!(&home2);
}

fn route(ip_kind: IpAddrKind) {}

fn main() {
    learn_enum()
}
