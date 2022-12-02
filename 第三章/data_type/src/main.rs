/*
 * @Author: wlj
 * @Date: 2022-12-01 11:15:30
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-02 09:46:30
 * @Description: 数据类型
 * @see:https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html#%E6%A0%87%E9%87%8F%E7%B1%BB%E5%9E%8B
 */
//在 Rust 中，每一个值都属于某一个 数据类型（data type），这告诉 Rust 它被指定为何种数据。

//Rust 是静态类型语言，所以我们在编译时需要知道所有变量的类型。同时他也有自己的类型推测
fn main() {
    //第二章的猜数字游戏中 如果不给guess类型就会报错 consider giving `guess` an explicit type。 请给guess一个详细的类型
    //let guess = "33".parse().expect("Not a number!");

    //这说明编译器需要我们提供更多的信息来了解我们想要的类型：
    let guess: usize = "42".parse().expect("Not a number!");
    println!("the guess value is: {guess}");

    // Rust 有两种数据类型子集：标量（scalar） 和 复合（compound）

    // 标量类型
    //Rust 有四种基本的标量类型：整型、浮点型、布尔型、字符串类型。

    //整形 是一个没有小数部分的数字。
    //Rust 内建的整数类型有：
    // 长度	   有符号	无符号
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize
    //数字类型默认是i32
    //符号数 以补码的形式存储。
    //比如 i8 有符号位 最大数就是 0111 1111  -127~127 2的7次方 -1
    //u8 最大数 1111 1111 0~255 2的8次方-1
    //另外，isize 和 usize 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的。
    let num1 = 45u8; //我们也可以用这样来设置数字的类型

    // Rust 中的整型字面值

    // 数字字面值	        例子
    // Decimal (十进制) 	98_222
    // Hex (十六进制)	    0xff
    // Octal (八进制)	    0o77
    // Binary (二进制)	    0b1111_0000
    // Byte (单字节字符)    (仅限于u8)	b'A'   备注：A在unicode码中是65

    let num2 = 98_222;
    let num3 = 0xff;
    let num4 = 0o77;
    let num5 = 0b1111_0000;
    let num6 = b'A';
    //数字可以用_作为分隔符方便读数98_222 = 98222
    println!("num1:{num1},num2:{num2},num3:{num3},num4:{num4},num5:{num5},num6:{num6}");

    //整形溢出 （“integer overflow” ）
    //  let num7: i8 = 128; //报错literal out of range for `i8`

    //当整形溢出时会导致下面两种行为之一的发生
    //1.当在 debug 模式编译时，Rust 检查这类问题并使程序 panic，这个术语被 Rust 用来表明程序因错误而退出。第九章 “panic! 与不可恢复的错误” 部分会详细介绍 panic。
    //2.在 release 构建中，Rust 不检测溢出，相反会进行一种被称为二进制补码回绕（two’s complement wrapping）的操作。
    // 简而言之，比此类型能容纳最大值还大的值会回绕到最小值，值 256 变成 0，值 257 变成 1，依此类推。
    // 依赖整型回绕被认为是一种错误，即便可能出现这种行为。如果你确实需要这种行为，标准库中有一个类型显式提供此功能，Wrapping。
    // 为了显式地处理溢出的可能性，你可以使用标准库在原生数值类型上提供的以下方法:
    // 所有模式下都可以使用 wrapping_* 方法进行回绕，如 wrapping_add
    // 如果 checked_* 方法出现溢出，则返回 None值
    // 用 overflowing_* 方法返回值和一个布尔值，表示是否出现溢出
    // 用 saturating_* 方法在值的最小值或最大值处进行饱和处理

    //浮点型
    //浮点数类型分为两种
    //1. f32    占32位
    //2. f64    占64位
    //默认类型时f64   因为在现代 CPU 中，它与 f32 速度几乎一样，不过精度更高。所有的浮点型都是有符号的
    let float1 = 2.0;
    let float2 = 30.2f32;
    println!("float1:{float1},float2:{float2}");
    //浮点数采用 IEEE-754 标准表示。f32 是单精度浮点数，f64 是双精度浮点数。

    //数值运算
    //Rust 中的所有数字类型都支持基本数学运算：加法、减法、乘法、除法和取余。整数除法会向下舍入到最接近的整数。

    //addition 加法
    let sum = 5 + 10;
    let sum2 = 0.1 + 0.2; //0.30000000000000004 因为0.1在二进制中是表示不出来的
    println!("sum:{sum},sum2:{sum2}");
    //subtraction 减法
    let difference = 95.5 - 4.3;
    println!("difference:{difference}");
    //multiplication 乘法
    let product = 4 * 30;
    println!("product:{product}");
    // division 除法
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; //0
    let floored2 = 3 / 2; //1
    println!("quotient:{quotient},floored:{floored},floored2:{floored2}");
    //remainder 余数
    let remainder = 43 % 5; //3
    println!("remainder:{remainder}");

    //布尔型
    //正如其他大部分编程语言一样，Rust 中的布尔类型有两个可能的值：true 和 false。Rust 中的布尔类型使用 bool 表示。
    let t = true;
    let f: bool = false;
    println!("t:{t},f:{f}");

    //字符类型
    //Rust的 char 类型是语言中最原生的字母类型。下面是一些声明 char 值的例子：
    let char1 = 'z';
    let char2: char = 'ℤ';
    let char3: char = '😻';
    // let char3: char = "😻2";报错 char是一个字符 用单引号表示 字符串才是双引号
    //Rust 的 char 类型的大小为四个字节(four bytes)，并代表了一个 Unicode 标量值（Unicode Scalar Value），这意味着它可以比 ASCII 表示更多内容。
    // 在 Rust 中，带变音符号的字母（Accented letters），中文、日文、韩文等字符，emoji（绘文字）以及零长度的空白字符都是有效的 char 值。
    // Unicode 标量值包含从 U+0000 到 U+D7FF 和 U+E000 到 U+10FFFF 在内的值。不过，“字符” 并不是一个 Unicode 中的概念，所以人直觉上的 “字符”
    // 可能与 Rust 中的 char 并不符合
    println!("char1:{char1},char2:{char2},char3:{char3}");

    //复合类型(Compound types) :可以将多个值合成一个类型。
    //Rust 有两个复合类型 元组（tuple） 和 数组 （array）

    //元组 元组长度固定：一旦声明，其长度不会增大或缩小
    let tup1: (i32, f64, char) = (500, 6.4, '1');
    //为了从元组中获取单个值，可以使用模式匹配（pattern matching）来解构（destructure）元组值 将一个元组拆成三个部分
    let (x, y, z) = tup1;
    println!("x:{x},y:{y},z:{z}");

    //我们也可以使用点号（.）后跟值的索引来直接访问它们
    let five_hundred = tup1.0; //跟大多数编程语言一样，元组的第一个索引值是 0
    println!("five_hundred:{five_hundred}");

    //我们可以通过{:?}来打印出元组
    println!("tup1={:?}",tup1);

    //不带任何值的元组有个特殊的名称，叫做 单元（unit） 元组。这种值以及对应的类型都写作 ()，表示空值或空的返回类型。如果表达式不返回任何其他值，则会隐式返回单元值。

    //数组类型
    //与元组不同，数组中的每个元素的类型必须相同。Rust 中的数组与一些其他语言中的数组不同，Rust中的数组长度是固定的。
    let arr = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = [3, 4, 5, 6, 7]; //显示类型表示
    let arr3 = [3; 5]; //等于[3,3,3,3,3]

    //当你想要在栈（stack）而不是在堆（heap）上为数据分配空间（第四章将讨论栈与堆的更多内容），或者是想要确保总是有固定数量的元素时，数组非常有用。
    //但是数组并不如 vector 类型灵活。vector 类型是标准库提供的一个 允许 增长和缩小长度的类似数组的集合类型。
    //当不确定是应该使用数组还是 vector 的时候，那么很可能应该使用 vector。第八章会详细讨论 vector。

    //然而，当你确定元素个数不会改变时，数组会更有用。例如，当你在一个程序中使用月份名字时，你更应趋向于使用数组而不是 vector，因为你确定只会有12个元素。
    let months_arr = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    //数组是可以在栈(stack)上分配的已知固定大小的单个内存块。可以使用索引来访问数组的元素，
    let first = arr[0];
    let second = arr[1];

    println!("first:{first},second:{second}");

    //备注：如果看到报错 help: if this is intentional, prefix it with an underscore: `_arr2` 如果在变量面前_可以消除这个错误（最好不要 一般都会将没有使用的变量删除
    //因为这是提示我们创建了变量但是没有使用的警告

    //如果访问数组结尾之后的元素会发生什么呢？比如执行以下代码
    let arr2 = [1, 2, 3, 4, 5];

    println!("please enter an array index");

    let mut index = String::new();
    //stdin(标准输入)
    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr2[index];

    println!("The value of the element at index {index} is: {element}")

    //如果输入了超过数组长度的值会看到如下错误
    //thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 7', src\main.rs:187:19
    //note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    //     程序在索引操作中使用一个无效的值时导致 运行时 错误。程序带着错误信息退出，并且没有执行最后的 println! 语句。
    //     当尝试用索引访问一个元素时，Rust 会检查指定的索引是否小于数组的长度。
    //     如果索引超出了数组长度，Rust 会 panic，这是 Rust 术语，它用于程序因为错误而退出的情况。
    //     这种检查必须在运行时进行，特别是在这种情况下，因为编译器不可能知道用户在以后运行代码时将输入什么值。

    // 这是第一个在实战中遇到的 Rust 安全原则的例子。
    // 在很多底层语言中，并没有进行这类检查，这样当提供了一个不正确的索引时，就会访问无效的内存。
    // 通过立即退出而不是允许内存访问并继续执行，Rust 让你避开此类错误。第九章会更详细地讨论 Rust 的错误处理机制，以及如何编写可读性强而又安全的代码
    // ，使程序既不会 panic 也不会导致非法内存访问。
}
