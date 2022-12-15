/*
 * @Author: wlj
 * @Date: 2022-12-15 14:35:02
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-15 14:57:09
 * @Description: 集成测试
 */
use organization_test;  //与单元测试不同，我们需要在文件顶部添加 use organization_test。这是因为每一个 tests 目录中的测试文件都是完全独立的 crate，所以需要在每一个文件中导入库。

//并不需要将 tests/integration_test.rs 中的任何代码标注为 #[cfg(test)]。 tests 文件夹在 Cargo 中是一个特殊的文件夹，\
// Cargo 只会在运行 cargo test 时编译这个目录中的文件。现在就运行 cargo test 试试：

//现在有了三个部分的输出：单元测试、集成测试和文档测试。第一部分单元测试与我们之前见过的一样：每个单元测试一行（ internal 的测试），接着是一个单元测试的摘要行。

//集成测试部分以行 Running target/debug/deps/integration-test-ce99bcc2479f4607（在输出最后的哈希值可能不同）开头
//。接下来每一行是一个集成测试中的测试函数，以及一个位于 Doc-tests adder 部分之前的集成测试的摘要行
//我们仍然可以通过指定测试函数的名称作为 cargo test 的参数来运行特定集成测试。也可以使用 cargo test 的 --test 后跟文件的名称来运行某个特定集成测试文件中的所有测试
//cargo test --test integration_test

//集成测试中的子模块
//随着集成测试的增加，你可能希望在 tests 目录增加更多文件以便更好的组织他们，例如根据测试的功能来将测试分组。正如我们之前提到的，每一个 tests 目录中的文件都被编译为单独的 crate

//将每个集成测试文件当作其自己的 crate 来对待，这更有助于创建单独的作用域，这种单独的作用域能提供更类似与最终使用者使用 crate 的环境。然而，正如你在第七章中学习的如何将代码分为模块和文件的知识，tests 目录中的文件不能像 src 中的文件那样共享相同的行为
//当你有一些在多个集成测试文件都会用到的帮助函数，而你尝试按照第七章 “将模块移动到其他文件” 部分的步骤将他们提取到一个通用的模块中时， tests 目录中不同文件的行为就会显得很明显。
//例如，如果我们可以创建 一个tests/common.rs 文件并创建一个名叫 setup 的函数，我们希望这个函数能被多个测试文件的测试函数调用
mod common;
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, organization_test::add_two(2));
}

//二进制 crate 的集成测试
//如果项目是二进制 crate 并且只包含 src/main.rs 而没有 src/lib.rs，这样就不可能在 tests 目录创建集成测试并使用 extern crate 导入 src/main.rs 中定义的函数。
//只有库 crate 才会向其他 crate 暴露了可供调用和使用的函数；二进制 crate 只意在单独运行。
//这就是许多 Rust 二进制项目使用一个简单的 src/main.rs 调用 src/lib.rs 中的逻辑的原因之一。
//因为通过这种结构，集成测试 就可以 通过 extern crate(外部crate) 测试库 crate 中的主要功能了，而如果这些重要的功能没有问题的话，src/main.rs 中的少量代码也就会正常工作且不需要测试。


//总结
//Rust 的测试功能提供了一个确保即使你改变了函数的实现方式，也能继续以期望的方式运行的途径。单元测试独立地验证库的不同部分，也能够测试私有函数实现细节。
//集成测试则检查多个部分是否能结合起来正确地工作，并像其他外部代码那样测试库的公有 API。
//即使 Rust 的类型系统和所有权规则可以帮助避免一些 bug，不过测试对于减少代码中不符合期望行为的逻辑 bug 仍然是很重要的。