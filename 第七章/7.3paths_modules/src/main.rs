/*
 * @Author: wlj
 * @Date: 2022-12-08 11:12:24
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-08 11:44:55
 * @Description:
 */
//这章我们了解一下Rust如何在模块树种找到一个项的位置，我们使用路径的方式，就像在文件系统使用路径一样。
//如果我们想要调用一个函数，我们需要它的路径。

//路径有两种形式：
// 1.绝对路径（absolute path)从crate跟开始，以crate名或者字面量crate开头。
// 2.相对路径（relative path）从当前模块开始，以self、super或当前模块的标识符开头。

//绝对路径和相对路径都后跟一个或多个由双冒号（::）分割的标识符。

//

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
        }
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    // 在绝对路径，我们从 crate，也就是 crate 根开始。然后 crate 根中定义了 front_of_house 模块。
    // front_of_house 模块不是公有的，
    // 不过因为 eat_at_restaurant 函数与 front_of_house 定义于同一模块中（即，eat_at_restaurant 和 front_of_house 是兄弟），
    // 我们可以从 eat_at_restaurant 中引用 front_of_house。接下来是使用 pub 标记的 hosting 模块。我们可以访问 hosting 的父模块，
    // 所以可以访问 hosting。最后，add_to_waitlist 函数被标记为 pub ，我们可以访问其父模块，所以这个函数调用是有效的！
    //我们可以理解为 私有只对父模块私有？

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
    //在相对路径，其逻辑与绝对路径相同，除了第一步：不同于从 crate 根开始，路径从 front_of_house 开始。
    // front_of_house 模块与 eat_at_restaurant 定义于同一模块，所以从 eat_at_restaurant 中开始定义的该模块相对路径是有效的。
    // 接下来因为 hosting 和 add_to_waitlist 被标记为 pub，路径其余的部分也是有效的，因此函数调用也是有效的！
}

fn main() {
    println!("Hello, world!");
    eat_at_restaurant();
    let a = 1;
}
