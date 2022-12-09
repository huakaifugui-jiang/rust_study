/*
 * @Author: wlj
 * @Date: 2022-12-09 09:49:23
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-09 14:40:10
 * @Description: 使用字符串储存 UTF-8 编码的文本
 * @see:https://kaisery.github.io/trpl-zh-cn/ch08-02-strings.html
 */

//什么是字符串？
//在开始深入这些方面之前，我们需要讨论一下术语字符串的具体意义。Rust的核心语言中只有一种字符串类型：字符串slice str，
//它通常以被借用的形式出现，&str。第四章降到了字符串slices:它们是一些对存储在别处的UTF-8编码的字符串数据的引用。
//举例来说，由于字符串字面量被存储在程序的二进制输出中，因此字符串字面量也是字符串slices。

//称作String的类型是由标准库提供的，而没有核心语言部分，它是可增长的、可变的、在所有权的、UTF-8编码的字符串类型。
//当Rustacean们谈到Rust的"字符串"时，它们通常指的是String或者字符串slice(&str)类型，而不特指其中某一个。虽然本部分
//大多是关于String的，不过这两个类型在Rust标准库中都被广泛使用，String和字符串slices都是UTF-8编码的。

//新建字符串
// 很多Vec可用的操作在String中同样可用
fn learn_new_string() {
    let mut s = String::new(); //使用new 函数创建一个空的字符串。

    //通常字符串会有初始数据，因为我们希望一开始就有这个字符串。为此，可以使用to_string方法,它能用于任何实现了
    //Display trait的类型，字符串字面量也实现了它

    let data = "initial contents";
    let s2 = data.to_string();

    //或者
    let s3 = "initial contents2".to_string();

    //或者
    let s4 = String::from("initial contents3"); //等同于使用 to_string
    println!(
        " learn_new_string  s:{} , s2:{} , s3:{},s4:{}",
        s, s2, s3, s4
    );

    //因为它是UTF-8（unicode）编码的，所以可以包含任何可以正确编码的数据
    let hello1 = String::from("こんにちは");
    let hello2 = String::from("안녕하세요");
    let hello3 = String::from("你好");
    println!(
        " learn_new_string  h:{} , h1:{} , h2:{}",
        hello1, hello2, hello3
    );
}

//更新字符串
//String的大小可以增加，其内容也可以改变，就像可以放入更多数据来改变Vec的内容一样。
//另外，可以方便的使用 + 运算符 或者 format!宏来拼接String值。
fn learn_update_string() {
    //使用push_str和push附加字符串
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("learn_update_string s : {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2); //从这里我们可以看出 push_str方法并不会获取参数的所有权.

    let mut s3 = String::from("lo");
    s3.push('l'); //可以使用push方法获取一个 ！！！单独的字符作为参数！！并附加到String中。
    println!("s3 is {}", s3);

    //使用 + 运算符
    let a1 = String::from("hello,");
    let a2 = String::from("world");
    let a3 = a1 + &a2; //注意 此时 a1被移动了，不能再继续使用
                       //s1 在相加后不再有效的原因，和使用 a2 的引用的原因，与使用 + 运算符时调用的函数签名有关。+ 运算符使用了 add 函数，这个函数签名看起来像这样：
                       //fn add(self, s: &str) -> String {
                       //首先，a2使用了&，意味着我们使用第二个字符串的引用与第一个字符串相加。这是因为add函数的s参数：只能将&str和String相加
                       //不过你会看到我们a2的类型是&String而不是&str，那么为什么还能运行呢？
                       //之所以能够在add调用是因为&String类型是可以被 强转（coerced）成&str。当add函数被调用时，Rust使用了一个被称为
                       //Deref强制转换（deref coercion）的技术，你可以将其理解为它吧&a2 变成了 &a2[...]。第十五章会更深入的讨论Deref强制转换。
                       //因为add没有获取参数的所有权，所以a2在这个操作后仍然是有效的String。

    //其次，可以发现签名中 add 获取了 self 的所有权，因为 self 没有 使用 &。这意味着示例 8-18 中的 s1 的所有权将被移动到 add 调用中，之后就不再有效。
    //    所以虽然 let s3 = s1 + &s2; 看起来就像它会复制两个字符串并创建一个新的字符串，而实际上这个语句会获取 s1 的所有权，
    //    附加上从 s2 中拷贝的内容，并返回结果的所有权。
    //    换句话说，它看起来好像生成了很多拷贝，不过实际上并没有：这个实现比拷贝要更高效。
    println!("a3 is {}", a3);
    // println!("a1 is {}", a1); 所以报错  value borrowed here after move

    //使用format!宏  我们先来看一段荔枝
    let f1 = String::from("tic");
    let f2 = String::from("tac");
    let f3 = String::from("toe");

    let f = f1 + "-" + &f2 + "-" + &f3; //此时在有那么多+ 和 “ 字符的情况下 很难理解具体发生了什么。对于更为复杂的链接，我们可以使用format!宏。
    let f4 = format!("{}-{}", f2, f3); //  它使用的是引用 所以它并不会获取任何参数的所有权。
    println!("f : {}", f);
}

//索引字符串
fn learn_index_string() {
    //在很多语言中，通过索引来引用字符串中的单独字符是有效且常见的操作。
    //然而在Rust中,如果你尝试使用索引语法访问String的一部分,会出现一个错误
    let s1 = String::from("hello");
    //let h = s1[0]; //`String` cannot be indexed by `{integer}` Rust字符串不支持索引。
    //那么为什么不支持索引呢? 为了回答这个问题,我们必须先聊一聊Rust是如何在内存中存储字符串的.
    //内部表现
    //String是一个Vec<u8>的封装.
    let hello = String::from("Hola");
    println!("hello len is {}", hello.len()); //4

    //在这里，len的值是4，这意味着存储字符串“Hola”的Vec的长度是四个字节：这里的每一个字母的UTF-8编码都占用一个字节。
    //那么下面这个呢？
    let hello2 = String::from("你好");
    println!("hello2 len is {}", hello2.len()); //6

    let hello3 = "Здравствуйте"; //这是一个slice 也就是str

    // let answer = &hello3[0]; З这玩意看起来像 数字3其实不是
    //我们已经知道answer不是第一个字符З.当使用UTF-8编码时，З的第一个字节 208，第二个是151，他占有两个字节，所以answer实际上是208
    //不过208自身并不是一个有效的字母。返回208可不是一个请求字符串第一个字母的人所希望看到的。不过它是RUST在字节索引0位置所能提供的唯一数据。
    //用户通常不会想要一个字节值的返回，即便这个字符串只有拉丁字母：即便是 &"hello"[0]是返回字节值的有效代码，他也应当返回104而不是h。
    //为了避免返回意外的值并造成不能立刻发现的bug，Rust根本不会编译这些代码，并在开发过程中及早的杜绝了误会的发生。

    //字节、标量值和字形簇（读作cu）！天呐！
    //这引起了关于UTF-8的另一个问题：从RUST的角度来讲，事实上有三种相关方式可以理解字符串：字节、标量值和字形簇（最接近人们眼中字母的概念）
    //比如这个用梵文书写的印度单词“नमस्ते”，最终它储存在 vector 中的 u8 值看起来像这样：
    //[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,224, 165, 135]
    //这里有18个字节，也就是计算机最终会存储的数据。如果从Unicode标量值的角度理解它们，也就像Rust的char可续那样，这些字节看起来像
    //['न', 'म', 'स', '्', 'त', 'े']
    //这里有六个char，不过第四个和第六个都不是字母，它们的发音符号本身是没有任何意义的。最后如果以字形簇的角度理解，就会得到人们所说的
    //构成这个单词的四个字母 :["न", "म", "स्", "ते"]
    //Rust提供了很多种不同的方式来解释计算机存储的原始字符串的数据（为了跨端？），这样程序就可以选择它需要的表现方式，而无所谓是何种人类语言。
    //最后一个Rust不允许使用索引获取String字符的原因是，索引的操作预期总是需要常熟时间（O(1)）。但是对于String不可能保证这样的性能，
    //因为Rust必须从头到索引位置遍历来确定有多少有效的字符。
}

//字符串slice
//索引字符串通常是一个坏点子,因为字符串索引应该返回的类型是不明确的：字节值、字符、字形簇或者字符串slice。因此，如果你真的希望
//使用索引创建字符串slice时，Rust会要求你更明确一些。为了更明确索引并表示你需要一个字符串slice，相比使用[]和单个值的索引，可以使用
//[]和一个range来创建含特定字节的字符串slice：
fn learn_slice_string() {
    let hello = "Здравствуйте".to_string(); //这些字母都是2个字节的长度

    println!("{}", &hello[0..2]); //所以这里会得到З

    //但是如果获取 第一个字节呢？
    // println!("{}", &hello[0..1]); //程序在运行时就会panic就跟访问 vector 中的无效索引时一样

    //遍历字符串的方法
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    //从字符串中获取字形簇是很复杂的，所以标准库并没有提供这个功能。crates.io 上有些提供这样功能的 crate。
}

// 字符串并不简单
// 总而言之，字符串还是很复杂的。不同的语言选择了不同的向程序员展示其复杂性的方式。
//Rust 选择了以准确的方式处理 String 数据作为所有 Rust 程序的默认行为，这意味着程序员们必须更多的思考如何预先处理 UTF-8 数据。
//这种权衡取舍相比其他语言更多的暴露出了字符串的复杂性，不过也使你在开发生命周期后期免于处理涉及非 ASCII 字符的错误。

// 现在让我们转向一些不太复杂的集合：哈希 map！

fn main() {
    learn_new_string();
    learn_update_string();
    learn_index_string();
    learn_slice_string();
}
