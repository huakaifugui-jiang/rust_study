/*
 * @Author: wlj
 * @Date: 2022-12-07 16:38:59
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-07 17:04:47
 * @Description:if let 简洁控制流
 * @see:https://kaisery.github.io/trpl-zh-cn/ch06-03-if-let.html
 */
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    //像上一小节所讲
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    //简洁写法
    //if let 语法可以让我们以一种不那么冗长 的方式结合if 和 let ，来处理只匹配一个模式的值而忽略其他模式的情况。
    let config_max2 = Some(4u8);
    if let Some(max) = config_max2 {
        println!("if let {}", max);
    }
    //就不需要有_=>()这种写法了。
    //后面可以搭配else
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
