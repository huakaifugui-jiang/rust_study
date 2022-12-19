/*
 * @Author: wlj
 * @Date: 2022-12-19 10:06:13
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-19 10:25:16
 * @Description: 采用发布配置自定义构建
 * @see:https://kaisery.github.io/trpl-zh-cn/ch14-01-release-profiles.html
 */

//在 Rust 中 发布配置（release profiles）是预定义的、可定制的带有不同选项的配置，他们允许程序员更灵活地控制代码编译的多种选项。每一个配置都彼此相互独立。

//Cargo 有两个主要的配置：运行 cargo build 时采用的 dev 配置和运行 cargo build --release 的 release 配置。
//dev 配置被定义为开发时的好的默认配置，release 配置则有着良好的发布构建的默认配置。

//这些配置名称可能很眼熟，因为它们出现在构建的输出中
// ```
//  cargo build
//     Finished dev [unoptimized + debuginfo] target(s) in 0.0s
//  cargo build --release
//     Finished release [optimized] target(s) in 0.0s
// ```

//构建输出中的 dev 和 release 表明编译器在使用不同的配置。
//当项目的 Cargo.toml 文件中没有任何 [profile.*] 部分的时候，Cargo 会对每一个配置都采用默认设置
//通过增加任何希望定制的配置对应的 [profile.*] 部分，我们可以选择覆盖任意默认设置的子集。
//看根目录下的cargo.toml文件
fn main() {
    println!("Hello, world!");
}
