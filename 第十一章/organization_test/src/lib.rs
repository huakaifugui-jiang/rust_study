/*
 * @Author: wlj
 * @Date: 2022-12-15 10:40:23
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-15 14:39:20
 * @Description: 测试的组织结构
 * @see:https://kaisery.github.io/trpl-zh-cn/ch11-03-test-organization.html
 */

//测试的组织结构
//本章一开始就提到，测试是一个复杂的概念，而且不同的开发者也采用不同的技术和组织。
//Rust 社区倾向于根据测试的两个主要分类来考虑问题：单元测试（unit tests）与 集成测试（integration tests）

//单元测试倾向于更小而更集中，在隔离的环境中一次测试一个模块，或者是测试私有接口。
//单元测试的目的是在与其他部分隔离的环境中测试每一个单元的代码，以便于快速而准确的某个单元的代码功能是否符合预期
//单元测试与他们要测试的代码共同存放在位于 src 目录下相同的文件中。规范是在每个文件中创建包含测试函数的 tests 模块，并使用 cfg(test) 标注模块。

//测试模块和 #[cfg(test)]
//测试模块的 #[cfg(test)] 注解告诉 Rust 只在执行 cargo test 时才编译和运行测试代码，
//而在运行 cargo build 时不这么做。这在只希望构建库的时候可以节省编译时间，并且因为它们并没有包含测试，所以能减少编译产生的文件的大小。
//与之对应的集成测试因为位于另一个文件夹，所以它们并不需要 #[cfg(test)] 注解
//。然而单元测试位于与源码相同的文件中，所以你需要使用 #[cfg(test)] 来指定他们不应该被包含进编译结果中。
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}
//测试私有函数
//测试社区中一直存在关于是否应该对私有函数直接进行测试的论战，而在其他语言中想要测试私有函数是一件困难的，甚至是不可能的事
//不过无论你坚持哪种测试意识形态，Rust 的私有性规则确实允许你测试私有函数。
fn internal_adder(a: i32, b: i32) -> i32 {
    //注意 internal_adder 函数并没有标记为 pub。测试也不过是 Rust 代码，同时 tests 也仅仅是另一个模块。
    //正如 “路径用于引用模块树中的项” 部分所说，子模块的项可以使用其上级模块的项。在测试中，我们通过 use super::* 将 test 模块的父模块的所有项引入了作用域，接着测试调用了 internal_adder
    //如果你并不认为应该测试私有函数，Rust 也不会强迫你这么做.
    a + b
}

#[cfg(test)] //cfg 属性代表 configuration ，它告诉 Rust 其之后的项只应该被包含进特定配置选项中,在这个例子中，配置选项是 test，即 Rust 所提供的用于编译和运行测试的配置选项
             //通过使用 cfg 属性，Cargo 只会在我们主动使用 cargo test 运行测试时才编译测试代码。这包括测试模块中可能存在的帮助函数， 以及标注为 #[test] 的函数。
mod tests {
    use super::*;
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
//集成测试
//而集成测试对于你的库来说则完全是外部的。它们与其他外部代码一样，通过相同的方式使用你的代码，只测试公有接口而且每个测试都有可能会测试多个模块。
//在 Rust 中，集成测试对于你需要测试的库来说完全是外部的。同其他使用库的代码一样使用库文件，也就是说它们只能调用一部分库中的公有 API 。
//集成测试的目的是测试库的多个部分能否一起正常工作。一些单独能正确运行的代码单元集成在一起也可能会出现问题，所以集成测试的覆盖率也是很重要的。
//为了创建集成测试，你需要先创建一个 tests 目录。
//为了编写集成测试，需要在项目根目录创建一个 tests 目录，与 src 同级。Cargo 知道如何去寻找这个目录中的集成测试文件。接着可以随意在这个目录中创建任意多的测试文件，Cargo 会将每一个文件当作单独的 crate 来编译。
