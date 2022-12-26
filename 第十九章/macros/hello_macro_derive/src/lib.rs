/*
 * @Author: wlj
 * @Date: 2022-12-26 10:24:45
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-26 10:51:03
 * @Description: 
 */

 extern crate proc_macro;//proc_macro 包是 Rust 自带的，因此无需在 Cargo.toml 中引入依赖，它包含了相关的编译器 API，可以用于读取和操作 Rust 源代码。

 use proc_macro::TokenStream;
 use quote::quote;
 use syn;
 
 #[proc_macro_derive(HelloMacro)]
 //由于我们为 hello_macro_derive 函数标记了 #[proc_macro_derive(HelloMacro)]，当用户使用 #[derive(HelloMacro)] 标记了他的类型后，hello_macro_derive 函数就将被调用
 //这里的秘诀就是特征名 HelloMacro，它就像一座桥梁，将用户的类型和过程宏联系在一起。
 pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
     // 基于 input 构建 AST 语法树
     let ast = syn::parse(input).unwrap();//syn 将字符串形式的 Rust 代码解析为一个 AST 树的数据结构，
    //该数据结构可以在随后的 impl_hello_macro 函数中进行操作。最后，操作的结果又会被 quote 包转换回 Rust 代码。
    //这些包非常关键，可以帮我们节省大量的精力，否则你需要自己去编写支持代码解析和还原的解析器，这可不是一件简单的任务！-
    //大家可能会注意到在 hello_macro_derive 函数中有 unwrap 的调用，也许会以为这是为了演示目的，没有做错误处理，实际上并不是的。
    //由于该函数只能返回 TokenStream 而不是 Result，那么在报错时直接 panic 来抛出错误就成了相当好的选择。
    // /当然，这里实际上还是做了简化，在生产项目中，你应该通过 panic! 或 expect 抛出更具体的报错信息。

     // 构建特征实现代码
     impl_hello_macro(&ast)
 }
//至此，这个函数大家应该已经基本理解了，下面来看看如何构建特征实现的代码，也是过程宏的核心目标:
fn impl_hello_macro(ast:&syn::DeriveInput)->TokenStream{
    let name = &ast.ident;//首先，将结构体的名称赋予给 name，也就是 name 中会包含一个字段，它的值是字符串 "Sunfei"。
    let gen = quote!{//其次，使用 quote! 可以定义我们想要返回的 Rust 代码。
        impl HelloMacro for #name {//大家注意到 #name 的使用了吗？这也是 quote! 提供的功能之一，如果想要深入了解 quote，可以看看官方文档。
            fn hello_macro(){//特征的 hell_macro() 函数只有一个功能，就是使用 println! 打印一行欢迎语句。
                println!("Hello, Macro! My name is {}!", stringify!(#name)); 
                //其中 stringify! 是 Rust 提供的内置宏，可以将一个表达式(例如 1 + 2)在编译期转换成一个字符串字面值("1 + 2")，该字面量会直接打包进编译出的二进制文件中，具有 'static 生命周期。
                //而 format! 宏会对表达式进行求值，最终结果是一个 String 类型。在这里使用 stringify! 有两个好处:
                //#name 可能是一个表达式，我们需要它的字面值形式
                //可以减少一次 String 带来的内存分配
            }
        }
    };
    gen.into()//由于编译器需要的内容和 quote! 直接返回的不一样，因此还需要使用 .into 方法其转换为 TokenStream。
}


 //注意我们将代码分成了hello_macro_derive 和 impl_macro_derive 两个函数，前者负责解析 TokenStream，后者负责转换语法树：这使得编写过程宏更方便。
//  这个函数的签名我们在之前已经介绍过，总之，这种形式的过程宏定义是相当通用的，下面来分析下这段代码。
//  首先有一点，对于绝大多数过程宏而言，这段代码往往只在 impl_hello_macro(&ast) 中的实现有所区别，对于其它部分基本都是一致的，例如包的引入、宏函数的签名、语法树构建等。
//syn::parse 调用会返回一个 DeriveInput 结构体来代表解析后的 Rust 代码:
// DeriveInput {
//     // --snip--

//     ident: Ident {
//         ident: "Sunfei",
//         span: #0 bytes(95..103)
//     },
//     data: Struct(
//         DataStruct {
//             struct_token: Struct,
//             fields: Unit,
//             semi_token: Some(
//                 Semi
//             )
//         }
//     )
// }
// 以上就是源代码 struct Sunfei; 解析后的结果，里面有几点值得注意:
// fields: Unit 说明源代码是一个单元结构体
// ident: "Sunfei" 说明类型名称为 Sunfei， ident 是标识符 identifier 的简写
// 如果想要了解更多的信息，可以查看 syn 文档。