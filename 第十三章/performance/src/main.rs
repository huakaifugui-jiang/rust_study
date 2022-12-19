/*
 * @Author: wlj
 * @Date: 2022-12-19 08:44:58
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-19 09:59:47
 * @Description: 性能对比：循环 VS 迭代器
 * @see:https://kaisery.github.io/trpl-zh-cn/ch13-04-performance.html
 */
// 性能对比：循环 VS 迭代器

//迭代器的性能比for循环高。
//迭代器，作为一个高级的抽象，被编译成了与手写的底层代码大体一致性能代码。
//迭代器是 Rust 的 零成本抽象（zero-cost abstractions）之一，它意味着抽象并不会引入运行时开销，它与本贾尼·斯特劳斯特卢普（C++ 的设计和实现者）在 “Foundations of C++”（2012） 中所定义的 零开销（zero-overhead）如出一辙

//作为另一个例子，这里有一些取自于音频解码器的代码。解码算法使用线性预测数学运算（linear prediction mathematical operation）来根据之前的样本的线性函数预测将来的值。
//这些代码使用迭代器链来对作用域的三个变量进行了某种数学计算：一个叫 buffer 的数据slice、一个有12个元素的数组 coefficient 、和一个代表位移位数的qlp_shift。
//例子中声明了这些变量但并没有提供任何值；虽然这些代码在其上下文之外没有什么意义，不过仍是一个简明的现实中的例子，来展示 Rust 如何将高级概念转换为底层代码：
fn main() {
    let buffer: &mut [i32];
    let coefficients: [i64; 12];
    let qlp_shift: i16;
    
    for i in 12..buffer.len() {//从buffer第十二个开始到结束
        let prediction = coefficients.iter()
                                     .zip(&buffer[i - 12..i])
                                     .map(|(&c, &s)| c * s as i64)
                                     .sum::<i64>() >> qlp_shift;
                                     //为了计算prediction的值，这些代码遍历了 coefficients 中的 12 个值，使用 zip 方法将系数与 buffer 的前 12 个值组合在一起
                                     //接着将每一对值相乘，再将所有结果相加，然后将总和右移 qlp_shift 位
        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }

    //像音频解码器这样的程序通常最看重计算的性能。这里，我们创建了一个迭代器，使用了两个适配器，接着消费了其值。
    //Rust 代码将会被编译为什么样的汇编代码呢？好吧，在编写本书的这个时候，它被编译成与手写的相同的汇编代码。遍历 coefficients 的值完全用不到循环：
    //Rust 知道这里会迭代 12 次，所以它“展开”（unroll）了循环。
    //展开是一种移除循环控制代码的开销并替换为每个迭代中的重复代码的优化。

    //所有的系数都被储存在了寄存器中，这意味着访问他们非常快。
    //这里也没有运行时数组访问边界检查。所有这些 Rust 能够提供的优化使得结果代码极为高效。
    //，请放心大胆的使用迭代器和闭包吧！他们使得代码看起来更高级，但并不为此引入运行时性能损失。
    //闭包和迭代器是 Rust 受函数式编程语言观念所启发的功能。他们对 Rust 以底层的性能来明确的表达高级概念的能力有很大贡献。
    //闭包和迭代器的实现达到了不影响运行时性能的程度。这正是 Rust 竭力提供零成本抽象的目标的一部分。
}
