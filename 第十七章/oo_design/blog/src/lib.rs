/*
 * @Author: wlj
 * @Date: 2022-12-22 10:50:14
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-22 15:22:02
 * @Description: Blog lib库
 */

pub struct Post {
    state: Option<Box<dyn State>>, //创建一个State的 对象trait
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
        //当创建新的 Post 时，我们将其 state 字段设置为一个存放了 Box 的 Some 值。这个 Box 指向一个 Draft 结构体新实例。这确保了无论何时新建一个 Post 实例，它都会从草案开始。
        //因为 Post 的 state 字段是私有的，也就无法创建任何其他状态的 Post 了！。Post::new 函数中将 content 设置为新建的空 String。
    }
    //现在需要更新 Post 的 content 方法。我们希望 content 根据 Post 的当前状态返回值，所以需要 Post 代理一个定义于 state 上的 content 方法
    pub fn content(&self) -> &str {
        //因为目标是将所有像这样的规则保持在实现了 State 的结构体中，我们将调用 state 中的值的 content 方法并传递博文实例（也就是 self）作为参数。
        //接着返回 state 值的 content 方法的返回值。
        //这里调用 Option 的 as_ref 方法是因为需要 Option 中值的引用而不是获取其所有权。
        //因为 state 是一个 Option<Box<dyn State>>，调用 as_ref 会返回一个 Option<&Box<dyn State>>。
        //如果不调用 as_ref，将会得到一个错误，因为不能将 state 移动出借用的 &self 函数参数。
        //接着调用 unwrap 方法，这里我们知道它永远也不会 panic，因为 Post 的所有方法都确保在他们返回时 state 会有一个 Some 值。
        //这就是一个第十二章 “当我们比编译器知道更多的情况” 部分讨论过的我们知道 None 是不可能的而编译器却不能理解的情况。
        self.state.as_ref().unwrap().content(self)
    }
    //存放博文内容的文本
    //我们希望能够调用一个叫做 add_text 的方法并向其传递一个 &str 来将文本增加到博文的内容中。选择实现为一个方法而不是将 content 字段暴露为 pub
    pub fn add_text(&mut self, text: &str) {
        //add_text 获取一个 self 的可变引用，因为需要改变调用 add_text 的 Post 实例。
        //接着调用 content 中的 String 的 push_str 并传递 text 参数来保存到 content 中。
        //这不是状态模式的一部分，因为它的行为并不依赖博文所处的状态。add_text 方法完全不与 state 状态交互，不过这是我们希望支持的行为的一部分。
        self.content.push_str(text);
    }

    // 请求审核博文来改变其状态
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review()); //在 Post 的当前状态下调用内部的 request_review 方法 并且第二个 request_review 方法会消费当前的状态并返回一个新状态。
        }
    }
    //同理增加改变 content 行为的 approve 方法
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}
//定义了所有不同状态的博文的所共享的行为。同时 Draft、PendingReview 和 Published 状态都会实现 State 状态。

trait State {
    //这里给 State trait 增加了 request_review 方法；所有实现了这个 trait 的类型现在都需要实现 request_review 方法。
    //注意不同于使用 self、 &self 或者 &mut self 作为方法的第一个参数，这里使用了 self: Box<Self>。
    //这个语法意味着该方法只可在持有这个类型的 Box 上被调用。这个语法获取了 Box<Self> 的所有权使老状态无效化，以便 Post 的状态值可转换为一个新状态。
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    //同理增加改变 content 行为的 approve 方法
    fn approve(self: Box<Self>) -> Box<dyn State>;
    //这里增加了一个 content 方法的默认实现来返回一个空字符串 slice。这意味着无需为 Draft 和 PendingReview 结构体实现 content 了。Published 结构体会覆盖 content 方法并会返回 post.content 的值。
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {} //同时开始将只定义 Draft 状态因为这是我们希望博文的初始状态。

impl State for Draft {
    //在参数中将之前的状态移出 Box<Self>
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        //为了消费老状态，request_review 方法需要获取状态值的所有权。
        //这就是 Post 的 state 字段中 Option 的来历：调用 take 方法将 state 字段中的 Some 值取出并留下一个 None，因为 Rust 不允许结构体实例中存在值为空的字段。
        //这使得我们将 state 的值移出 Post 而不是借用它。接着我们将博文的 state 值设置为这个操作的结果。
        //我们需要将 state 临时设置为 None 来获取 state 值，即老状态的所有权，而不是使用 self.state = self.state.request_review()这样的代码直接更新状态值。这确保了当 Post 被转换为新状态后不能再使用老 state 值。
        Box::new(PendingReview {})
        //接着我们将博文的 state 值设置为这个操作的结果。
    }

    //如果对 Draft 调用 approve 方法，并没有任何效果，因为它会返回 self
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

//Draft 的 request_review 方法需要返回一个新的，装箱的 PendingReview 结构体的实例，其用来代表博文处于等待审核状态。
struct PendingReview {}
impl State for PendingReview {
    //结构体 PendingReview 同样也实现了 request_review 方法，不过它不进行任何状态转换。
    //相反它返回自身，因为当我们请求审核一个已经处于 PendingReview 状态的博文，它应该继续保持 PendingReview 状态。
    //现在我们能看出状态模式的优势了：无论 state 是何值，Post 的 request_review 方法都是一样的。每个状态只负责它自己的规则。
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    //当对 PendingReview 调用 approve 时，它返回一个新的、装箱的 Published 结构体的实例。
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    //Published 结构体实现了 State trait，同时对于 request_review 和 approve 两方法来说，它返回自身，因为在这两种情况博文应该保持 Published 状态。
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    //注意这个方法需要生命周期注解，如第十章所讨论的。这里获取 post 的引用作为参数，并返回 post 一部分的引用，所以返回的引用的生命周期与 post 参数相关。
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
