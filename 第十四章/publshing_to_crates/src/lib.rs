/*
 * @Author: wlj
 * @Date: 2022-12-19 10:33:59
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-19 11:42:53
 * @Description: 将 crate 发布到 Crates.io
 */

// 注释包含项的结构
// 还有另一种风格的文档注释，//!，这为包含注释的项，而不是位于注释之后的项增加文档。这通常用于 crate 根文件（通常是 src/lib.rs）或模块的根文件为 crate 或模块整体提供文档。
// 作为一个例子，如果我们希望增加描述包含 add_one 函数的 my_crate crate 目的的文档，可以在 src/lib.rs 开头增加以 //! 开头的注释，如示例 14-2 所示：
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// 如果运行 cargo doc --open，将会发现这些注释显示在 my_crate 文档的首页，位于 crate 中公有项列表之上

//我们曾经在项目中使用 crates.io 上的包作为依赖，不过你也可以通过发布自己的包来向他人分享代码。crates.io 用来分发包的源代码，所以它主要托管开源代码。
//Rust 和 Cargo 有一些帮助他人更方便找到和使用你发布的包的功能。我们将介绍一些这样的功能，接着讲到如何发布一个包。

//编写有用的文档注释
//准确的包文档有助于其他用户理解如何以及何时使用他们，所以花一些时间编写文档是值得的。第三章中我们讨论了如何使用两斜杠 // 注释 Rust 代码。
// //Rust 也有特定的用于文档的注释类型，通常被称为 文档注释（documentation comments）
// 他们会生成 HTML 文档。这些 HTML 展示公有 API 文档注释的内容，他们意在让对库感兴趣的程序员理解如何 使用 这个 crate，而不是它是如何被 实现 的。
// 文档注释使用三斜杠 /// 而不是两斜杆以支持 Markdown 注解来格式化文本。文档注释就位于需要文档的项的之前。

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = publshing_to_crates::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
// 这里，我们提供了一个 add_one 函数工作的描述，接着开始了一个标题为 Examples 的部分，和展示如何使用 add_one 函数的代码。
// 。可以运行 cargo doc 来生成这个文档注释的 HTML 文档
// 这个命令运行由 Rust 分发的工具 rustdoc 并将生成的 HTML 文档放入 target/doc 目录

//为了方便起见，运行 cargo doc --open 会构建当前 crate 文档（同时还有所有 crate 依赖的文档）的 HTML 并在浏览器中打开。导航到 add_one 函数将会发现文档注释的文本是如何渲染的

//常用（文档注释）部分
// 示例 14-1 中使用了 # Examples Markdown 标题在 HTML 中创建了一个以 “Examples” 为标题的部分。其他一些 crate 作者经常在文档注释中使用的部分有：
// Panics：这个函数可能会 panic! 的场景。并不希望程序崩溃的函数调用者应该确保他们不会在这些情况下调用此函数。
// Errors：如果这个函数返回 Result，此部分描述可能会出现何种错误以及什么情况会造成这些错误，这有助于调用者编写代码来采用不同的方式处理不同的错误。
// Safety：如果这个函数使用 unsafe 代码（这会在第十九章讨论），这一部分应该会涉及到期望函数调用者支持的确保 unsafe 块中代码正常工作的不变条件（invariants）。
// 大部分文档注释不需要所有这些部分，不过这是一个提醒你检查调用你代码的人有兴趣了解的内容的列表。

// 文档注释作为测试

// 在文档注释中增加示例代码块是一个清楚的表明如何使用库的方法，这么做还有一个额外的好处：cargo test 也会像测试那样运行文档中的示例代码！
// 没有什么比有例子的文档更好的了，但最糟糕的莫过于写完文档后改动了代码，而导致例子不能正常工作。尝试 cargo test 运行像示例 14-1 中 add_one 函数的文档；应该在测试结果中看到像这样的部分：
// Doc-tests publshing_to_crates 证明cargo test 会运行 add_one doc注释的代码

// 使用 pub use 导出合适的公有 API
// 第七章介绍了如何使用 mod 关键字来将代码组织进模块中，如何使用 pub 关键字将项变为公有，和如何使用 use 关键字将项引入作用域
// 然而你开发时候使用的文件架构可能并不方便用户。你的结构可能是一个包含多个层级的分层结构，不过这对于用户来说并不方便
// 这是因为想要使用被定义在很深层级中的类型的人可能很难发现这些类型的存在
// 他们也可能会厌烦要使用 use my_crate::some_module::another_module::UsefulType; 而不是 use my_crate::UsefulType; 来使用类型。

// 公有 API 的结构是你发布 crate 时主要需要考虑的。crate 用户没有你那么熟悉其结构，并且如果模块层级过大他们可能会难以找到所需的部分。
// 好消息是，即使文件结构对于用户来说 不是 很方便，你也无需重新安排内部组织：你可以选择使用 pub use 重导出（re-export）项来使公有结构不同于私有结构。
// 重导出获取位于一个位置的公有项并将其公开到另一个位置，好像它就定义在这个新位置一样。

// 例如，假设我们创建了一个描述美术信息的库 art。这个库中包含了一个有两个枚举 PrimaryColor 和 SecondaryColor 的模块 kinds，以及一个包含函数 mix 的模块 utils

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) {}
}

//运行 cargo doc--open查看 所生成的 crate 文档首页
//注意 PrimaryColor 和 SecondaryColor 类型、以及 mix 函数都没有在首页中列出。我们必须点击 kinds 或 utils 才能看到他们。
// 另一个依赖这个库的 crate 需要 use 语句来导入 art 中的项，这包含指定其当前定义的模块结构。示例 14-4 展示了一个使用 art crate 中 PrimaryColor 和 mix 项的 crate 的例子
// use art::kinds::PrimaryColor;
// use art::utils::mix;

// fn main() {
//     let red = PrimaryColor::Red;
//     let yellow = PrimaryColor::Yellow;
//     mix(red, yellow);
// }
// 使用 art crate 代码的作者不得不搞清楚 PrimaryColor 位于 kinds 模块而 mix 位于 utils 模块。
// art crate 的模块结构相比使用它的开发者来说对编写它的开发者更有意义。其内部的 kinds 模块和 utils 模块的组织结构并没有对尝试理解如何使用它的人提供任何有价值的信息。
// art crate 的模块结构因不得不搞清楚所需的内容在何处和必须在 use 语句中指定模块名称而显得混乱和不便。
// 为了从公有 API 中去掉 crate 的内部组织，我们可以采用示例 14-3 中的 art crate 并增加 pub use 语句来重导出项到顶层结构
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;
// 现在此 crate 由 cargo doc 生成的 API 文档会在首页列出重导出的项以及其链接，如图 14-4 所示，这使得 PrimaryColor 和 SecondaryColor 类型和 mix 函数更易于查找
// use art::mix;
// use art::PrimaryColor;

// fn main() {
//     // --snip--
// }
//对于有很多嵌套模块的情况，使用 pub use 将类型重导出到顶级结构对于使用 crate 的人来说将会是大为不同的体验。
//创建一个有用的公有 API 结构更像是一门艺术而非科学，你可以反复检视他们来找出最适合用户的 API。
//pub use 提供了解耦组织 crate 内部结构和与终端用户体现的灵活性。观察一些你所安装的 crate 的代码来看看其内部结构是否不同于公有 API。

//创建 Crates.io 账号
//在你可以发布任何 crate 之前，需要在 crates.io 上注册账号并获取一个 API token。为此，访问位于 crates.io 的首页并使用 GitHub 账号登录。
//（目前 GitHub 账号是必须的，不过将来该网站可能会支持其他创建账号的方法）
//一旦登录之后，查看位于 https://crates.io/me/ 的账户设置页面并获取 API token。接着使用该 API token 运行 cargo login 命令，像这样：
//cargo login abcdefghijklmnopqrstuvwxyz012345(你的API TOKEN)
//这个命令会通知 Cargo 你的 API token 并将其储存在本地的 ~/.cargo/credentials 文件中。
//注意这个 token 是一个 秘密（secret）且不应该与其他人共享。如果因为任何原因与他人共享了这个信息，应该立即到 crates.io 重新生成这个 token。

//发布新 crate 之前
//有了账号之后，比如说你已经有一个希望发布的 crate。在发布之前，你需要在 crate 的 Cargo.toml 文件的 [package] 部分增加一些本 crate 的元信息（metadata）
// 首先 crate 需要一个唯一的名称。虽然在本地开发 crate 时，可以使用任何你喜欢的名称。不过 crates.io 上的 crate 名称遵守先到先得的分配原则。
// 一旦某个 crate 名称被使用，其他人就不能再发布这个名称的 crate 了
// 请在网站上搜索你希望使用的名称来找出它是否已被使用。如果没有，修改 Cargo.toml 中 [package] 里的名称为你希望用于发布的名称，像这样
// [package]
// name = "guessing_game"

