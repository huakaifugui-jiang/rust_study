/*
 * @Author: wlj
 * @Date: 2022-12-23 16:49:03
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-23 17:38:33
 * @Description: 高级类型
 * @see:https://kaisery.github.io/trpl-zh-cn/ch19-04-advanced-types.html
 */

// Rust 的类型系统有一些我们曾经提到但没有讨论过的功能。首先我们从一个关于为什么 newtype 与类型一样有用的更宽泛的讨论开始。
//  接着会转向类型别名（type aliases），一个类似于 newtype 但有着稍微不同的语义的功能。我们还会讨论 ! 类型和动态大小类型
// 为了类型安全和抽象而使用 newtype 模式

// newtype 模式可以用于一些其他我们还未讨论的功能，包括静态的确保某值不被混淆，和用来表示一个值的单元。
// 实际上一章中已经有一个这样的例子：Millimeters 和 Meters 结构体都在 newtype 中封装了 u32 值。如果编写了一个有 Millimeters 类型参数的函数，不小心使用 Meters 或普通的 u32 值来调用该函数的程序是不能编译的。

// 另一个 newtype 模式的应用在于抽象掉一些类型的实现细节：例如，封装类型可以暴露出与直接使用其内部私有类型时所不同的公有 API，以便限制其功能。

// newtype 也可以隐藏其内部的泛型类型。例如，可以提供一个封装了 HashMap<i32, String> 的 People 类型，用来储存人名以及相应的 ID。
// 使用 People 的代码只需与提供的公有 API 交互即可，比如向 People 集合增加名字字符串的方法，这样这些代码就无需知道在内部我们将一个 i32 ID 赋予了这个名字了。
// newtype 模式是一种实现第十七章 “封装隐藏了实现细节” 部分所讨论的隐藏实现细节的封装的轻量级方法。

fn main() {
    // 类型别名用来创建类型同义词
    // 连同 newtype 模式，Rust 还提供了声明 类型别名（type alias）的能力，使用 type 关键字来给予现有类型另一个名字。例如，可以像这样创建 i32 的别名 Kilometers：
    type Kilometers = i32;
    //这意味着 Kilometers 是 i32 的 同义词（synonym）
    // 不同于示例 19-15 中创建的 Millimeters 和 Meters 类型。Kilometers 不是一个新的、单独的类型。Kilometers 类型的值将被完全当作 i32 类型值来对待：
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    //因为 Kilometers 是 i32 的别名，他们是同一类型，可以将 i32 与 Kilometers 相加，也可以将 Kilometers 传递给获取 i32 参数的函数。但通过这种手段无法获得上一部分讨论的 newtype 模式所提供的类型检查的好处。
    //类型别名的主要用途是减少重复。
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    //类型别名也经常与 Result<T, E> 结合使用来减少重复。考虑一下标准库中的 std::io 模块。I/O 操作通常会返回一个 Result<T, E>，因为这些操作可能会失败。
    //标准库中的 std::io::Error 结构体代表了所有可能的 I/O 错误。std::io 中大部分函数会返回 Result<T, E>，其中 E 是 std::io::Error，比如 Write trait 中的这些函数

    use std::fmt;
    use std::io::Error;

    // pub trait Write {
    //     fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    //     fn flush(&mut self) -> Result<(), Error>;

    //     fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    //     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
    // }

    //这里出现了很多的 Result<..., Error>。为此，std::io 有这个类型别名声明：
    type Result<T> = std::result::Result<T, std::io::Error>;

    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;

        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
    }
    //类型别名在两个方面有帮助：易于编写 并 在整个 std::io 中提供了一致的接口。因为这是一个别名，它只是另一个 Result<T, E>，这意味着可以在其上使用 Result<T, E> 的任何方法，以及像 ? 这样的特殊语法。

    //从不返回的 never type
    //Rust 有一个叫做 ! 的特殊类型。在类型理论术语中，它被称为 empty type，因为它没有值。我们更倾向于称之为 never type。这个名字描述了它的作用：在函数从不返回的时候充当返回值。

    fn bar() -> ! {
        // --snip--
        panic!();
    }

    //这读 “函数 bar 从不返回”，而从不返回的函数被称为 发散函数（diverging functions）。不能创建 ! 类型的值，所以 bar 也不可能返回值。
    //不过一个不能创建值的类型有什么用呢？如果你回想一下示例 2-5 中的代码，曾经有一些看起来像这样的代码，
    // let guess: u32 = match guess.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => continue,
    // };
    //当时我们忽略了代码中的一些细节。在第六章 “match 控制流运算符” 部分，我们学习了 match 的分支必须返回相同的类型。
    // 这里的 guess 必须既是整型 也是 字符串，而 Rust 要求 guess 只能是一个类型。那么 continue 返回了什么呢？
    // 正如你可能猜到的，continue 的值是 ! 。也就是说，当 Rust 要计算 guess 的类型时，它查看这两个分支。前者是 u32 值，而后者是 ! 值。因为 ! 并没有一个值，Rust 决定 guess 的类型是 u32。
    // 描述 ! 的行为的正式方式是 never type 可以强转为任何其他类型。允许 match 的分支以 continue 结束是因为 continue 并不真正返回一个值；
    // 相反它把控制权交回上层循环，所以在 Err 的情况，事实上并未对 guess 赋值。
    // never type 的另一个用途是 panic!。还记得 Option<T> 上的 unwrap 函数吗？它产生一个值或 panic。

    // impl<T> Option<T> {
    //     pub fn unwrap(self) -> T {
    //         match self {
    //             Some(val) => val,
    //             None => panic!("called `Option::unwrap()` on a `None` value"),
    //         }
    //     }
    // }

    // 这里与示例 19-34 中的 match 发生了相同的情况：Rust 知道 val 是 T 类型，panic! 是 ! 类型，所以整个 match 表达式的结果是 T 类型。
    // 这能工作是因为 panic! 并不产生一个值；它会终止程序。对于 None 的情况，unwrap 并不返回一个值，所以这些代码是有效的。

    // 最后一个有着 ! 类型的表达式是 loop：
    print!("forever ");

    loop {
        print!("and ever ");
    }
    // 这里，循环永远也不结束，所以此表达式的值是 !。但是如果引入 break 这就不为真了，因为循环在执行到 break 后就会终止。

    // 动态大小类型和 Sized trait
    // 因为 Rust 需要知道例如应该为特定类型的值分配多少空间这样的信息其类型系统的一个特定的角落可能令人迷惑：这就是 动态大小类型（dynamically sized types）的概念
    // 这有时被称为 “DST” 或 “unsized types”，这些类型允许我们处理只有在运行时才知道大小的类型。
    // 让我们深入研究一个贯穿本书都在使用的动态大小类型的细节：str。没错，不是 &str，而是 str 本身。
    // str 是一个 DST；直到运行时我们都不知道字符串有多长。
    // 因为直到运行时都不能知道其大小，也就意味着不能创建 str 类型的变量，也不能获取 str 类型的参数。考虑一下这些代码，他们不能工作：

    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";

    // Rust 需要知道应该为特定类型的值分配多少内存，同时所有同一类型的值必须使用相同数量的内存。如果允许编写这样的代码，也就意味着这两个 str 需要占用完全相同大小的空间，不过它们有着不同的长度。
    // 这也就是为什么不可能创建一个存放动态大小类型的变量的原因。
    // 那么该怎么办呢？你已经知道了这种问题的答案：s1 和 s2 的类型是 &str 而不是 str。
    // 如果你回想第四章 “字符串 slice” 部分，slice 数据结构储存了开始位置和 slice 的长度。

    // 所以虽然 &T 是一个储存了 T 所在的内存位置的单个值，&str 则是 两个 值：str 的地址和其长度。
    // 这样，&str 就有了一个在编译时可以知道的大小：它是 usize 长度的两倍。也就是说，我们总是知道 &str 的大小，而无论其引用的字符串是多长。
    // 这里是 Rust 中动态大小类型的常规用法：他们有一些额外的元信息来储存动态信息的大小。这引出了动态大小类型的黄金规则：必须将动态大小类型的值置于某种指针之后。
    // 可以将 str 与所有类型的指针结合：比如 Box<str> 或 Rc<str>。事实上，之前我们已经见过了，不过是另一个动态大小类型：trait
    // 每一个 trait 都是一个可以通过 trait 名称来引用的动态大小类型。在第十七章 “为使用不同类型的值而设计的 trait 对象” 部分，我们提到了为了将 trait 用于 trait 对象，
    // 必须将他们放入指针之后，比如 &dyn Trait 或 Box<dyn Trait>（Rc<dyn Trait> 也可以）。
    // 为了处理 DST，Rust 有一个特定的 trait 来决定一个类型的大小是否在编译时可知：这就是 Sized trait。
    // 这个 trait 自动为编译器在编译时就知道大小的类型实现。另外，Rust 隐式的为每一个泛型函数增加了 Sized bound。。也就是说，对于如下泛型函数定义：
    // fn generic<T>(t: T) {
    //     // --snip--
    // }

    // 实际上被当作如下处理：
    // fn generic<T: Sized>(t: T) {
    //     // --snip--
    // }

    // 泛型函数默认只能用于在编译时已知大小的类型。然而可以使用如下特殊语法来放宽这个限制：
    fn generic<T: ?Sized>(t: &T) {
        // --snip--
    }
    // ?Sized 上的 trait bound 意味着 “T 可能是也可能不是 Sized” 同时这个注解会覆盖泛型类型必须在编译时拥有固定大小的默认规则。这种意义的 ?Trait 语法只能用于 Sized ，而不能用于任何其他 trait。
    // 另外注意我们将 t 参数的类型从 T 变为了 &T：因为其类型可能不是 Sized 的，所以需要将其置于某种指针之后。在这个例子中选择了引用。



}
