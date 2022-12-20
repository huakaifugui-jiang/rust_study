/*
 * @Author: wlj
 * @Date: 2022-12-20 08:13:59
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-20 09:24:45
 * @Description: 通过 Deref trait 将智能指针当作常规引用处理
 * @see:https://kaisery.github.io/trpl-zh-cn/ch15-02-deref.html
 */
//实现 Deref trait 允许我们重载 解引用运算符（dereference operation） * （与乘法运算符或通配符 或通配符相区别）。
//通过这种方式实现 Deref trait 的智能指针可以被当作常规引用来对待，可以编写操作引用的代码并用于智能指针。
use std::ops::Deref;
fn main() {
    //通过解引用运算符追踪指针的值
    let x = 5; // 存放了一个 i32 值 5。
    let y = &x; //y 等于 x 的一个引用。
    let y2 = Box::new(x); //将 y 设置为一个指向 x 值拷贝的 box 实例
    assert_eq!(5, x); //可以断言 x 等于 5
                      // assert_eq!(x, y);报错 no implementation for `{integer} == &{integer}`
    assert_eq!(x, *y); //然而，如果希望对 y 的值做出断言，必须使用 *y 来追踪引用所指向的值（也就是 解引用）一旦解引用了 y，就可以访问 y 所指向的整型值并可以与 5 做比较
    assert_eq!(x, *y2);

    // 自定义智能指针
    //为了体会默认情况下智能指针与引用的不同，让我们创建一个类似于标准库提供的 Box<T> 类型的智能指针。接着学习如何增加使用解引用运算符的功能。
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    //如第十章 “为类型实现 trait” 部分所讨论的，为了实现 trait，需要提供 trait 所需的方法实现。Deref trait，由标准库提供，
    impl<T> Deref for MyBox<T> {
        type Target = T; //type Target = T; 语法定义了用于此 trait 的关联类型。关联类型是一个稍有不同的定义泛型参数的方式，现在还无需过多的担心它；第十九章会详细介绍。
        fn deref(&self) -> &Self::Target {
            //返回内部数据的引用 也就是T 这这里就是&i32
            //要求实现名为 deref 的方法，其借用 self 并返回一个内部数据的引用。
            &self.0 //因为  struct MyBox<T>(T);是一个元组 .0 访问第一个元素
        } //现在可以编译并能通过断言
    }

    let x1 = 5;
    let y3 = MyBox::new(x1);
    assert_eq!(x1, *y3); //MyBox<T> 类型不能解引用，因为我们尚未在该类型实现这个功能。为了启用 * 运算符的解引用功能，需要实现 Deref trait。
                         //没有 Deref trait 的话，编译器只会解引用 & 引用类型。
                         //deref 方法向编译器提供了获取任何实现了 Deref trait 的类型的值，并且调用这个类型的 deref 方法来获取一个它知道如何解引用的 & 引用的能力
                         //当我们在运行*y3的时候实际上运行了*(y3.deref())
    assert_eq!(x1, *(y3.deref()));
    //Rust 将 * 运算符替换为先调用 deref 方法再进行普通解引用的操作，如此我们便不用担心是否还需手动调用 deref 方法了。
    //Rust 的这个特性可以让我们写出行为一致的代码，无论是面对的是常规引用还是实现了 Deref 的类型。
    //deref 方法返回值的引用，以及 *(y.deref()) 括号外边的普通解引用仍为必须的原因在于所有权。
    //如果 deref 方法直接返回值而不是值的引用，其值（的所有权）将被移出 self。
    //在这里以及大部分使用解引用运算符的情况下我们并不希望获取 MyBox<T> 内部值的所有权
    let d = *y3;
    let c = *y3;
    //可以看出并没有获取所有权
    //注意，每次当我们在代码中使用 * 时， * 运算符都被替换成了先调用 deref 方法再接着使用 * 解引用的操作，且只会发生一次
    //不会对 * 操作符无限递归替换，解引用出上面 i32 类型的值就停止了

    //函数和方法的隐式 Deref 强制转换
    //Deref 强制转换(deref coercions)是Rust在函数或方法上传参上的一种便利。
    //Deref 强制转换只能作用于实现了 Deref trait 的类型。
    //Deref 强制转换将这样一个类型的引用转换为另一个类型的引用。例如，Deref 强制转换 可以将 &String 转换为 &str
    //因为 String 实现了 Deref trait 因此可以返回 &str
    //当这种特定类型的引用作为实参传递给和形参类型不同的函数或方法时，Deref 强制转换将自动发生。
    //时会有一系列的 deref 方法被调用，把我们提供的类型转换成了参数所需的类型
    //Deref 强制转换的加入使得 Rust 程序员编写函数和方法调用时无需增加过多显式使用 & 和 * 的引用和解引用。这个功能也使得我们可以编写更多同时作用于引用或智能指针的代码。
    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }
    let m = MyBox::new(String::from("Rust"));
    let h = &m; //h的类型是 &MyBox<String>
    hello(h); //因为示例 15-10 中在 MyBox<T> 上实现了 Deref trait，Rust 可以通过 deref 调用将 &MyBox<String> 变为 &String。
              //标准库中提供了 String 上的 Deref 实现，其会返回字符串 slice，这可以在 Deref 的 API 文档中看到。
              //Rust 再次调用 deref 将 &String 变为 &str，这就符合 hello 函数的定义了。
              //如果 Rust 没有实现 Deref 强制转换，为了使用 &MyBox<String> 类型的值调用 hello，则不得不编写示
    let h2 = &(*h)[..]; //(*h) 将 MyBox<String> 解引用为 String。接着 & 和 [..] 获取了整个 String 的字符串 slice 来匹配 hello 的签名。
                        //当所涉及到的类型定义了 Deref trait，Rust 会分析这些类型并使用任意多次 Deref::deref 调用以获得匹配参数的类型。这些解析都发生在编译时，所以利用 Deref 强制转换并没有运行时损耗！
    hello(h2);

    //Deref 强制转换如何与可变性交互
    //类似于如何使用 Deref trait 重载不可变引用的 * 运算符，Rust 提供了 DerefMut trait 用于重载可变引用的 * 运算符。
    // Rust 在发现类型和 trait 实现满足三种情况时会进行 Deref 强制转换：
    //当 T: Deref<Target=U> 时从 &T 到 &U。
    //当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
    //当 T: Deref<Target=U> 时从 &mut T 到 &U。
    //头两个情况除了可变性之外是相同的：第一种情况表明如果有一个 &T，而 T 实现了返回 U 类型的 Deref，则可以直接得到 &U。第二种情况表明对于可变引用也有着相同的行为。
    //第三个情况有些微妙：Rust 也会将可变引用强转为不可变引用。
    //但是反之是 不可能 的：不可变引用永远也不能强转为可变引用。
    //因为根据借用规则，如果有一个可变引用，其必须是这些数据的唯一引用（否则程序将无法编译）。
    //将一个可变引用转换为不可变引用永远也不会打破借用规则。将不可变引用转换为可变引用则需要初始的不可变引用是数据唯一的不可变引用，而借用规则无法保证这一点。因此，Rust 无法假设将不可变引用转换为可变引用是可能的。
    let mut h3 = &m;
    hello(h3);
}
