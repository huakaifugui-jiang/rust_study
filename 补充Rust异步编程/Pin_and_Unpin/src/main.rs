/*
 * @Author: wulongjiang
 * @Date: 2023-01-03 21:01:29
 * @LastEditors: wulongjiang
 * @LastEditTime: 2023-01-03 21:28:50
 * @Description: 定海神针 Pin 和 Unpin
 * @FilePath: \Pin_and_Unpin\src\main.rs
 */

//再Rust中，所有的类型可以分为两类：
//类型的值可以在内存中安全地被移动，例如数值、字符串、布尔值、结构体、枚举，总之你能想到的几乎所有类型都可以落入到此范畴内
//自引用类型，大魔王来了，大家快跑，在之前章节我们已经见识过它的厉害。
//下面就是一个自引用类型
struct SelfRef {
    value: String,
    pointer_to_value: *mut String,
}
//在上面结构体中，pointer_to_value是一个裸指针，指向第一个字段value持有的字符串String。那如果String 被移动了怎么办？
//此时一个致命的问题就出现了：新的字符串的内存地址变了，而 pointer_to_value 依然指向之前的地址，一个重大 bug 就出现了！
//灾难发生，英雄在哪？只见 Pin 闪亮登场，它可以防止一个类型在内存中被移动。
//再来回忆下之前在 Future 章节中，我们提到过在 poll 方法的签名中有一个 self: Pin<&mut Self> ，
//那么为何要在这里使用 Pin 呢？
fn main() {
    //为何需要Pin
    //其实Pin还有一个小伙伴UnPin，与前者相反，后者是表示类型可以在内存中安全地移动
}
