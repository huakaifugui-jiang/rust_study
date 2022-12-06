/*
 * @Author: wlj
 * @Date: 2022-12-05 16:26:53
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-06 09:26:36
 * @Description: 结构体的定义和实例化
 * @see:https://kaisery.github.io/trpl-zh-cn/ch05-01-defining-structs.html
 */
//结构体(struct)的定义和实例化
//结构体与元组类似，它们都包含多个相关的值，每一个值都可以是不同的类型。
//但不一样的是结构体 需要 命名 各部分数据以便能清楚的表明其值的意义。
//由于有了这些名字，结构体比元组更灵活：不需要依赖顺序来指定或访问实例中的值。 感觉像ts的接口😀

//定义结构体，需要使用 struct 关键字并为整个结构体提供一个名字。结构体的名字需要描述它所组合的数据的意义。接着，在大括号中，定义每一部分数据的名字和类型，我们称为 字段（field）
//例如下面 展示了一个存储用户账号信息的结构体：
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn learn_struct() {
    //如何使用结构体  通过为每个字段指定具体值来创建这个结构体的 实例。创建一个实例需要以结构体的名字开头，接着在大括号中使用 key: value 键-值对的形式提供字段，其中 key 是字段的名字，value 是需要存储在字段中的数据值。
    //实例中字段的顺序不需要和它们在结构体中声明的顺序一致。换句话说，结构体的定义就像一个类型的通用模板，而实例则会在这个模板中放入特定数据来创建这个类型的值。
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("username"),
        active: true,
        sign_in_count: 1,
    };

    //为了从结构体中获取某个特定的值，可以使用点号。举个例子，想要用户的邮箱地址，可以用 user1.email。如果结构体的实例是可变的，我们可以使用点号并为对应的字段赋值。
    //注意 整个user1 实例必须是可变的；Rust并不允许只将某个字段标记为可变。
    user1.email = String::from("anotheremail@example.com");
    println!("user email : {}", user1.email);

    //我们也可以在函数体的最后一个表达式中构造一个结构体的新实例，来隐式的返回这个实例。
    learn_struct_back(String::from("test@email"), String::from("cxk"));

    //使用结构体更新语法  从其他实例创建实例

    //使用旧实例的大部分值但改变其部分值来创建一个新的结构体实例通常是很有用的。这可以通过 结构体更新语法（struct update syntax）实现。
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    //此处需注意，结构更新语法就像带有 = 的赋值 因为它移动了数据，就像我们上一章节 4.1所有权讲的“变量与数据交互的方式（一）：移动”部分讲到的一样
    //如果我们讲user1的 username或者email 给 user2 ，那么user1的这两个属性都不能再使用 因为他们被移动到了user2 （因为他们是String类型的
    //而 sign_in_count 跟 active 则不会 因为他们的类型是 实现 Copy trait 的类型所以我们在“变量与数据交互的方式（二）：克隆” 部分讨论的行为同样适用。

    // println!("user email : {}", user1.username);所以这里会报错

    let user3 = User {
        username: String::from("another2@example.com"),
        ..user1 // ..语法指定了剩余未显示设置值的字段应由与给定实例对应字段相同的字 他也是结构更新语法 会move 或者 clone
    };
}

fn learn_struct_back(email: String, username: String) -> User {
    // User {
    //     email: email,
    //     username: username,
    //     active: true,
    //     sign_in_count: 1,
    // }
    //我们可以使用 字段初始化简写语法（field init shorthand）来重写 build_user
    User {
        email, //因为 email 字段与 email 参数有着相同的名称，则只需编写 email 而不是 email: email。
        username,
        active: true,
        sign_in_count: 1,
    }
}

//使用没有命名字段的元组结构体来创建不用的类型
//元组结构体有着结构体名称提供的含义，但是没有具体的字段名，只有字段类型。
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn learn_tuple_struct() {
    let black = Color(0, 1, 2);
    //注意 black 和 origin 值的类型不同，因为它们是不同的元组结构体的实例。你定义的每一个结构体有其自己的类型，即使结构体中的字段可能有着相同的类型。
    //例如，一个获取 Color 类型参数的函数不能接受 Point 作为参数，即便这两个类型都由三个 i32 值组成。
    //在其他方面，元组结构体实例类似于元组，你可以将它们解构为单独的部分，也可以使用 . 后跟索引来访问单独的值，等等。
    let Color(a, b, c) = black;
    println!("a : {}", black.0);
    println!("a : {a},b:{b},c:{c}");

    // let i: (i32, i32, i32) = (0, 1, 2);
    // let f = i;
    // let v = i;
    // let d: Color = black;
    // let h: Color = black;
    //疑问为什么 Color 元组 赋值会发生move？ 而 i元组不会呢？
    // println!("a : {}", black.0);
}

//类单元结构体（unit-like structs）因为它们类似（），及元组类型 那边提到的unit类型。
//类单元结构体常常在你想要在某个类型上实现trait但不需要在类型中存储数据的时候发挥作用。（第十章讲trait  有点云里雾里的。没看后面的 现在感觉像是class 类
struct AlwaysEqual; //不需要花括号
fn learn_unit_struct() {
    let subject = AlwaysEqual; //获得 AlwaysEqual 的实例
                               //想象一下，我们将实现这个类型的行为，即每个实例始终等于每一个其他类型的实例，也许是为了获得一个已知的结果以便进行测试。
                               //我们不需要任何数据来实现这种行为，你将在第十章中，看到如何定义特性并在任何类型上实现它们，包括类单元结构体。
}

//结构体的所有权
//在上面的 User结构体中我们使用了自身拥有所有权的String类型而不是&str字符串slice类型。
//这是一个有意而为之的选择，如果我们想要这个结构体拥有它所有的数据，我们必须拥有它所有的数据。
//像下面的User2 我们拥有它所有的数据，为此只要整个结构体是有效的话，其数据也是有效的。
// 可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上 生命周期（lifetimes），这是一个第十章会讨论的 Rust 功能。
// 生命周期确保结构体引用的数据有效性跟结构体本身保持一致。如果你尝试在结构体中存储一个引用而不指定生命周期将是无效的，比如这样：
struct User2 {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}
fn learn_overship_struct() {
    // let user1 = User2 {
    //     email: "someone@example.com",
    //     username: "someusername123",
    //     active: true,
    //     sign_in_count: 1,
    // };
    //报错，expected named lifetime parameter 需要生命周期标识符
}

fn main() {
    println!("Hello, world!");
    learn_struct();
    learn_tuple_struct();
    learn_unit_struct();
    learn_overship_struct();
}
