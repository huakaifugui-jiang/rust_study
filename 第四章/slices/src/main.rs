/*
 * @Author: wlj
 * @Date: 2022-12-05 11:06:58
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-05 16:21:39
 * @Description: Slice类型
 * @see：https://kaisery.github.io/trpl-zh-cn/ch04-03-slices.html
 */

//此章节也可查看https://course.rs/basic/compound-type/string-slice.html#%E5%88%87%E7%89%87slice 更容易理解

//slice (中文翻译：片，部分，把...分割) slice允许你 引用 集合中的一段连续的元素序列，而不用引用整个集合。slice是一类引用，所以它没有 所有权
//这里有一个编程小习题：编写一个函数，该函数接收一个用空格分隔单词的字符串，并返回在该字符串中找到的第一个单词。
//如果函数在该字符串中并未找到空格，则整个字符串就是一个单词，所以应该返回整个字符串。

//如果不使用slice 可以做到吗？
//我们可以先试试获取 空格在字符串中的索引，如果没有空格就返回整个字符长度
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); //as_bytes 方法将 String 转化为字节数组（就是每个字符对应的编码， 如果一个字符有多个字节 比如中文 有三个字节 那么就会占据三个 此处暂时在考虑单字节的
    let empty_bytescode = b' '; //获取空格的编码32 具体http://liubin.org/blog/2022/04/01/rust-string-literals/
    println!("empty_bytescode : {empty_bytescode}");
    // for i in bytes {
    //     println!("i : {}", i);
    // } //这样是获取不到索引的

    //用iter方法在字节数组上创建一个迭代器（第十三章详解） iter方法返回集合中的每一个元素，而enumerate方法包装了iter，
    //enumerate 将这些元素作为元组的一部分来返回enumerate 返回的元组中，第一个元素是索引，第二个元素是集合中元素的引用 (index,&item)。 注意是引用 所以要用&
    //下面例子中的(i, &item)是对 元组的解构。
    for (i, &item) in bytes.iter().enumerate() {
        if item == empty_bytescode {
            return i;
        }
    }

    s.len()
}

//slice版本 “字符串 slice” 的类型声明写作 &str
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

//更好的定义  //如果有一个字符串 slice，可以直接传递它。
//如果有一个 String，则可以传递整个 String 的 slice 或对 String 的引用。这种灵活性利用了 deref coercions 的优势，这个特性我们将在“函数和方法的隐式 Deref 强制转换”章节中介绍。
//定义一个获取字符串 slice 而不是 String 引用的函数使得我们的 API 更加通用并且不会丢失任何功能：
fn first_word_slice_better(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("word : {}", word);
    s.clear(); // 这清空了字符串，使其等于 ""
               //到了这一步但是 我们还是解决不了我们的问题
               //并且 此时如果 s 发生了改变 例如 上面的s被清空了，但是word还是5 不会改变，那么是非常容易出问题的。word 的索引与 s 中的数据不再同步
               //现在我们要跟踪一个开始索引 和 一个结尾索引，同时有了更多从数据的某个特定状态计算而来的值，但都完全没有与这个状态相关联。现在有三个飘忽不定的不相关变量需要保持同步。
               //幸运的是，Rust 为这个问题提供了一个解决方法：字符串 slice。
    let s1 = String::from("hello world");
    //不同于整个String的引用，hello是一个部分（s）String的引用，由一个额外的[0..5]指定。可以使用一个由中括号[starting_index..ending_index]指定的range创建一个slice
    //其中starting_index 是 slice 的第一个位置，ending_index 则是 slice 最后一个位置的后一个值。在其内部，slice 的数据结构存储了 slice 的开始位置和长度，长度对应于 ending_index 减去 starting_index 的值。
    //注意slice 都是指的字节
    //所以对于 let world = &s[6..11]; 的情况，world 将是一个包含指向 s 索引 6 的指针和长度值 5 的 slice。具体查看slice.png
    let hello = &s1[0..5]; //不包含 5  意思是第0字节~第4字节
    let world = &s1[6..11]; //不包含 11 意思是第6字节~第10字节
    println!("hello : {} , world : {}", hello, world);

    //对于Rust的 .. range语法，如果想要从索引0开始，可以不写两个点号之前的值。换句话说，如下两个语句是相同的：
    // let s = String::from("hello");
    // let slice = &s[0..2];
    // let slice = &s[..2];
    // 依此类推，如果 slice 包含 String 的最后一个字节，也可以舍弃尾部的数字。这意味着如下也是相同的：
    //     let s = String::from("hello");

    // let len = s.len();

    // let slice = &s[3..len];
    // let slice = &s[3..];
    // 也可以同时舍弃这两个值来获取整个字符串的 slice。所以如下亦是相同的：
    // let s = String::from("hello");

    // let len = s.len();

    // let slice = &s[0..len];
    // let slice = &s[..];

    //注意：字符串 slice range 的索引必须位于有效的 UTF-8 字符边界内，如果尝试从一个多字节字符的中间位置创建字符串 slice，则程序将会因错误而退出。
    //出于介绍字符串 slice 的目的，本部分假设只使用 ASCII 字符集；第八章的 “使用字符串存储 UTF-8 编码的文本” 部分会更加全面的讨论 UTF-8 处理问题。
    let t = String::from("哈哈哈");
    let t1 = &t[0..3]; //例如中文在Utf-8中占用三个字节 如果写成 &t[0..2]会报错
    println!("t1 : {t1}");

    //slice版本
    let mut w = String::from("hello world");
    let w1 = first_word_slice(&w);
    //现在我们有了一个不易混淆且直观的 API 了，因为编译器会确保指向 String 的引用持续有效。
    //还记得那个当我们获取第一个单词结尾的索引后，接着就清除了字符串导致索引就无效的 bug 吗？那些代码在逻辑上是不正确的，但却没有显示任何直接的错误。
    //问题会在之后尝试对空字符串使用第一个单词的索引时出现。slice 就不可能出现这种 bug 并让我们更早的知道出问题了。
    // w.clear(); // 错误! mutable borrow occurs here
    //回忆一下借用的规则，当我们借用了一个不可变引用时，在使用它期间不能再获取一个可变引用。因为 clear 需要清空 String，它尝试获取一个可变引用。
    println!("w1 : {w1}");

    //&str类型
    let c = String::from("hello world"); //这里的c的类型就是&str，它是一个指向二进制程序特定位置的slice（硬编码进程序的，静态的）
    let c_word = first_word_slice_better(&c);
    //也可以  let c_word = first_word_slice_better(&c[0..6]); 使用引用 更通用
    //如果是 let c_word = first_word_slice(&c[0..6]) 则会类型报错 expected struct `String`, found `str`
    println!("c_word : {}", c_word);

    //其他类型 slice
    let a = [1, 2, 3, 4, 5];
    let a_slice = &a[1..3];
    assert_eq!(a_slice, &[2, 3]); //assert_eq! 宏，来自标准库，判断两个参数是否相等，类似 == 如果不一样程序会panic（恐慌）用于测试 https://doc.rust-lang.org/std/macro.assert_eq.html
                                  //这个 slice 的类型是 &[i32]。它跟字符串 slice 的工作方式一样，通过存储第一个集合元素的引用和一个集合总长度。你可以对其他所有集合使用这类 slice。第八章讲到 vector 时会详细讨论这些集合。
}
