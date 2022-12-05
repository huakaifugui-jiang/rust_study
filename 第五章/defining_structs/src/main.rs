/*
 * @Author: wlj
 * @Date: 2022-12-05 16:26:53
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-05 17:49:13
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
    let user2 = User{
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count, 
    };
    //此处需注意，结构更新语法就像带有 = 的赋值 因为它移动了数据，就像我们上一章节 4.1所有权讲的“变量与数据交互的方式（一）：移动”部分讲到的一样
    //如果我们讲user1的 username或者email 给 user2 ，那么user1的这两个属性都不能再使用 因为他们被移动到了user2 （因为他们是String类型的
    //而 sign_in_count 跟 active 则不会 因为他们的类型是 实现 Copy trait 的类型所以我们在“变量与数据交互的方式（二）：克隆” 部分讨论的行为同样适用。
    println!("user email : {}", user1.username);
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
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    println!("Hello, world!");
    learn_struct();
}
