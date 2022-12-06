/*
 * @Author: wlj
 * @Date: 2022-12-06 10:10:38
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-06 15:35:48
 * @Description: 方法语法
 * @see:https://kaisery.github.io/trpl-zh-cn/ch05-03-method-syntax.html
 */
//方法语法
//方法（method）与函数类似：他们使用fn关键字和名称生命，可以拥有参数和返回值，同时包含在某处调用该方法时会执行的代码。
//不过方法与函数是不同的，因为它们在结构体的上下文中被定义（或者是枚举（第6章讲解） 或trait对象的上下文）。
//并且他们的第一个参数总是self，它代表调用该方法的结构体实例。

//让我们把前面实现的获取一个 Rectangle 实例作为参数的 area 函数，改写成一个定义于 Rectangle 结构体上的 area 方法。
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// 为了使函数定义于 Rectangle 的上下文中。 我们开始了一个impl块（impl是implementation （中文：实施，实现）的缩写）
//这个impl块中的所有内容都将与Rectangle类型相关联。
//他的第一个参数总是self，它代表调用该方法的结构体实例。
impl Rectangle {
    //使用 &self 来替代 rectangle: &Rectangle，&self 实际上是 self: &Self 的缩写。在一个 impl 块中，Self 类型是 impl 块的类型的别名。
    //方法的第一个参数必须有一个名为 self 的Self 类型的参数，所以 Rust 让你在第一个参数位置上只用 self 这个名字来缩写
    fn area(&self) -> u32 {
        println!("area方法被调用了！");
        self.width * self.height
    }

    //我们可以选择将方法的名称与结构中的一个字段相同。例如
    fn width(&self) -> bool {
        self.width > 0
        //在这里，我们选择让 width 方法在实例的 width 字段的值大于 0 时返回 true，等于 0 时则返回 false：我们可以出于任何目的，在同名的方法中使用同名的字段。在 main 中，当我们在 rect1.width 后面加上括号时。
        //Rust 知道我们指的是方法 width。当我们不使用圆括号时，Rust 知道我们指的是字段 width。

        //通常，但并不总是如此，与字段同名的方法将被定义为只返回字段中的值，而不做其他事情。这样的方法被称为 getters，Rust 并不像其他一些语言那样为结构字段自动实现它们。Getters 很有用，因为你可以把字段变成私有的，
        //但方法是公共的，这样就可以把对字段的只读访问作为该类型公共 API 的一部分。我们将在第七章中讨论什么是公有和私有，以及如何将一个字段或方法指定为公有或私有
    }

    //带有更多参数的方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //关联函数
    //所有在impl 块中定义的函数都被称为 关联函数（associated functions），因为它们与impl后面命名的类型相关。
    //我们可以定义一个不以 self 为第一参数的关联函数（因此它不能被称作方法），因为它们并不作用于一个结构体的实例。
    //我们已经使用过了这样的一个函数：在String类型上定义的String::from函数。
    //不是方法的关联函数通常被用作返回一个结构体新实例的构造函数。 这些函数的名称通常为new，但new并不是一个关键词。
    //例如我们可以提供一个叫做 square 关联函数，它接受一个维度参数并且同时作为宽和高，这样可以更轻松的创建一个正方形 Rectangle 而不必指定两次同样的值：

    //关键字 Self 在函数的返回类型中代指在 impl 关键字后出现的类型，在这里是 Rectangle
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

//多个 impl 块
// 每个结构体都允许拥有多个 impl 块。例如。
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
    if rect.width() {
        println!("The rectangle has a nonzero width; it is {}", rect.width);
    }
    println!("rect.width fn : {}", rect.width());
    println!("rect.width : {}", rect.width);

    //在C/C++ 语言中，有两个不同的运算符来调用方法：. 直接在对象中调用方法，而 -> 在一个对象的指针上调用方法，这时需要先解引用（dereference)指针。
    //换句话说，如果object是一个指针，那么object->something() 就像 (*object).something() (ps:*是解引用，意思就是拿到指针所指向的地址的值 也就是object) 一样。

    //Rust并没有一个与 -> 等效的运算符；相反，Rust 有一个叫 自动引用和解引用（automatic referencing and dereferencing）的功能。方法调用是Rsut中少数几个拥有这种行为的地方。
    //它是这样工作的：当使用object.something() 调用方法时，Rust会自动为object添加 & 、 &mut 或 * 以便使 object 与方法签名匹配。
    //也就是说，这些代码是等价的：
    rect.area();
    (&rect).area();
    //第一行看起来简洁了许多，因为&self是一个引用 所以正常来说 参数也是引用，但是参数是结构体本身 所以需要&rect；
    //但是因为 方法有一个很明确的 接收者 ----也就是sele类型 。 在明确了接收者和方法名的前提下，Rust可以很明确的计算出 方法是仅仅读取（&self），做出修改（&mut self）或者是获取所有权（self）。事实上，Rust 对方法接收者的隐式借用让所有权在实践中更友好。

    //补充到这边看来 struct有点像 class  不过是拆开的。

    //带有更多参数的方法
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect.can_hold(&rect3));

    //关联函数。
    //使用结构体名和 :: 语法来调用这个关联函数
    //这个函数唯一结构体的命名空间中：:: 语法用于 关联函数和模块创建的命名空间。七章会讲到模块。
    let sq = Rectangle::square(20);
    dbg!(sq);
}
