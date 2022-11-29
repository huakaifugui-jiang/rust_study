/*
 * @Author: wlj
 * @Date: 2022-11-25 14:19:44
 * @LastEditors: wlj
 * @LastEditTime: 2022-11-29 11:24:30
 * @Description: https://kaisery.github.io/trpl-zh-cn/ch01-02-hello-world.html
 */

/*
1.首先定义了一个名为main的函数，它没有参数也没有返回值，如果有参数的话它们的名称应该出现在小括号中
2.函数体被包裹在{  }中Rust要求所有函数体都要用花括号包裹起来，
一般来说，将左花括号与函数声明置于同一行并以空格分隔，是良好的代码风格
*/
fn main() {
	println!("hello,world!");
}
//！！！要注意的是 main函数是一个特殊的函数：在可执行的rust程序中，它是最先运行的代码。