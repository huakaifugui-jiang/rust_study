/*
 * @Author: wlj
 * @Date: 2022-12-22 10:50:19
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-22 15:47:57
 * @Description:
 */
//面向对象设计模式的实现
//状态模式（state pattern）是一个面向对象设计模式。该模式的关键在于一个值有某些内部状态，体现为一系列的 状态对象，同时值的行为随着其内部状态而改变。
//状态对象共享功能：当然，在 Rust 中使用结构体和 trait 而不是对象和继承。每一个状态对象负责其自身的行为，以及该状态何时应当转移至另一个状态。
//持有一个状态对象的值对于不同状态的行为以及何时状态转移毫不知情。

//使用状态模式意味着当程序的业务需求改变时，无需改变值持有状态或者使用值的代码。我们只需更新某个状态对象中的代码来改变其规则，或者是增加更多的状态对象。让我们看看一个有关状态模式和如何在 Rust 中使用它的例子。

//为了探索这个概念，我们将实现一个增量式的发布博文的工作流。这个博客的最终功能看起来像这样：
// 1.博文从空白的草案开始。
// 2.一旦草案完成，请求审核博文。
// 3.一旦博文过审，它将被发表。
// 4.只有被发表的博文的内容会被打印，这样就不会意外打印出没有被审核的博文的文本。
//任何其他对博文的修改尝试都是没有作用的。例如，如果尝试在请求审核之前通过一个草案博文，博文应该保持未发布的状态。
// 示例 17-11 展示这个工作流的代码形式：这是一个我们将要在一个叫做 blog 的库 crate 中实现的 API 的示例。这段代码还不能编译，因为还未实现 blog。
use blog::Post;
fn main() {
    let mut post = Post::new(); //我们希望允许用户使用 Post::new 创建一个新的博文草案。

    post.add_text("I ate a salad for lunch today"); //也希望能在草案阶段为博文编写一些文本
    assert_eq!("", post.content()); //如果在审批之前尝试立刻获取博文的内容，不应该获取到任何文本因为博文仍然是草案。一个好的单元测试将是断言草案博文的 content 方法返回空字符串

    post.request_review(); //接下来，我们希望能够请求审核博文
    assert_eq!("", post.content()); //而在等待审核的阶段 content 应该仍然返回空字符串。

    post.approve(); //最后当博文审核通过，它应该被发表
    assert_eq!("I ate a salad for lunch today", post.content()); //这意味着当调用 content 时博文的文本将被返回。

    //注意我们与 crate 交互的唯一的类型是 Post。这个类型会使用状态模式并会存放处于三种博文所可能的状态之一的值 —— 草案，等待审核和发布。
    //状态上的改变由 Post 类型内部进行管理。状态依库用户对 Post 实例调用的方法而改变，但是不能直接管理状态变化。
    //这也意味着用户不会在状态上犯错，比如在过审前发布博文。
    //接下来实现Post库

    //状态模式的权衡取舍
    //我们展示了 Rust 是能够实现面向对象的状态模式的，以便能根据博文所处的状态来封装不同类型的行为。Post 的方法并不知道这些不同类型的行为。
    //通过这种组织代码的方式，要找到所有已发布博文的不同行为只需查看一处代码：Published 的 State trait 的实现。
    //如果要创建一个不使用状态模式的替代实现，则可能会在 Post 的方法中，或者甚至于在 main 代码中用到 match 语句，来检查博文状态并在这里改变其行为。
    //这意味着需要查看很多位置来理解处于发布状态的博文的所有逻辑！这在增加更多状态时会变得更糟：每一个 match 语句都会需要另一个分支。
    //对于状态模式来说，Post 的方法和使用 Post 的位置无需 match 语句，同时增加新状态只涉及到增加一个新 struct 和为其实现 trait 的方法。
    //状态模式的一个缺点是因为状态实现了状态之间的转换，一些状态会相互联系。如果在 PendingReview 和 Published 之间增加另一个状态，比如 Scheduled，则不得不修改 PendingReview 中的代码来转移到 Scheduled。
    //如果 PendingReview 无需因为新增的状态而改变就更好了，不过这意味着切换到另一种设计模式。
    //另一个缺点是我们会发现一些重复的逻辑。为了消除他们，可以尝试为 State trait 中返回 self 的 request_review 和 approve 方法增加默认实现，
    //不过这会违反对象安全性，因为 trait 不知道 self 具体是什么。我们希望能够将 State 作为一个 trait 对象，所以需要其方法是对象安全的。
    //另一个重复是 Post 中 request_review 和 approve 这两个类似的实现。他们都委托调用了 state 字段中 Option 值的同一方法，并在结果中为 state 字段设置了新值。
    //如果 Post 中的很多方法都遵循这个模式，我们可能会考虑定义一个宏来消除重复（查看第十九章的 “宏” 部分）。
    //完全按照面向对象语言的定义实现这个模式并没有尽可能地利用 Rust 的优势。让我们看看一些代码中可以做出的修改，来将无效的状态和状态转移变为编译时错误。

    //将状态和行为编码为类型
    //我们将展示如何稍微反思状态模式来进行一系列不同的权衡取舍。不同于完全封装状态和状态转移使得外部代码对其毫不知情，我们将状态编码进不同的类型。
    //如此，Rust 的类型检查就会将任何在只能使用发布博文的地方使用草案博文的尝试变为编译时错误。

    //我们仍然希望能够使用 Post::new 创建一个新的草案博文，并能够增加博文的内容。
    //不过不同于存在一个草案博文时返回空字符串的 content 方法，我们将使草案博文完全没有 content 方法。
    //这样如果尝试获取草案博文的内容，将会得到一个方法不存在的编译错误。
    //这使得我们不可能在生产环境意外显示出草案博文的内容，因为这样的代码甚至就不能编译。

    //Post
    pub struct Post2 {
        content: String,
    }

    impl Post2 {
        fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }
        pub fn content(&self) -> &str {
            &self.content
        }
    }

    struct DraftPost {
        content: String,
    }

    impl DraftPost {
        fn add_text(&mut self, text: &str) {
            self.content.push_str(text)
        }
        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }

    pub struct PendingReviewPost {
        content: String,
    }

    impl PendingReviewPost {
        pub fn approve(self) -> Post2 {
            Post2 {
                content: self.content,
            }
        }
    }

    //以下的Post 就是Post2
    //仍然有一个 Post::new 函数，不过不同于返回 Post 实例，它返回 DraftPost 的实例。现在不可能创建一个 Post 实例，因为 content 是私有的同时没有任何函数返回 Post。
    //DraftPost 上定义了一个 add_text 方法，这样就可以像之前那样向 content 增加文本，不过注意 DraftPost 并没有定义 content 方法！如此现在程序确保了所有博文都从草案开始，
    //同时草案博文没有任何可供展示的内容。任何绕过这些限制的尝试都会产生编译错误。
    //request_review 和 approve 方法获取 self 的所有权，因此会消费 DraftPost 和 PendingReviewPost 实例，并分别转换为 PendingReviewPost 和发布的 Post。
    //这样在调用 request_review 之后就不会遗留任何 DraftPost 实例，后者同理。
    //PendingReviewPost 并没有定义 content 方法，所以尝试读取其内容会导致编译错误，DraftPost 同理。
    //唯一得到定义了 content 方法的 Post 实例的途径是调用 PendingReviewPost 的 approve 方法，而得到 PendingReviewPost 的唯一办法是调用 DraftPost 的 request_review 方法，现在我们就将发博文的工作流编码进了类型系统。
    let mut post2 = Post2::new();

    post2.add_text("I ate a salad for lunch today");

    let post2 = post2.request_review();

    let post2 = post2.approve();

    assert_eq!("I ate a salad for lunch today", post2.content());
    //不得不修改 main 来重新赋值 post 使得这个实现不再完全遵守面向对象的状态模式：状态间的转换不再完全封装在 Post 实现中。
    //然而，得益于类型系统和编译时类型检查，我们得到了的是无效状态是不可能的！这确保了某些特定的 bug，比如显示未发布博文的内容，将在部署到生产环境之前被发现。
    //即便 Rust 能够实现面向对象设计模式，也有其他像将状态编码进类型这样的模式存在。这些模式有着不同的权衡取舍
    //虽然你可能非常熟悉面向对象模式，重新思考这些问题来利用 Rust 提供的像在编译时避免一些 bug 这样有益功能。
    // Rust 中面向对象模式并不总是最好的解决方案，因为 Rust 拥有像所有权这样的面向对象语言所没有的功能。
}
