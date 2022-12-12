/*
 * @Author: wlj
 * @Date: 2022-12-12 09:48:27
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-12 10:13:20
 * @Description: 泛型数据类型
 * @see:https://kaisery.github.io/trpl-zh-cn/ch10-01-syntax.html
 */

//我们可以使用泛型为像函数签名 或者结构体这样的项 创建定义，这样它们就可以用于多种不同的具体数据类型。
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest_i32(&number_list);
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest_char(&char_list);
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

//在函数中使用泛型 像下面两种函数，他们除了 函数名称与签名类型不同其他完全相同 我们就可以使用泛型

//最大的数字函数
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &i in list {
        if largest < i {
            largest = i;
        }
    }

    largest
}

//最大的字符函数
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//泛型版 寻找最大数 Rust类型的命名规范是骆驼命名法（CamelCase）T 作为 “type” 的缩写是大部分 Rust 程序员的首选。
fn largest<T>(list: &[T]) -> T {
    //函数largest 有泛型T。它有个参数list，其类型是元素为T的slice，largest函数的返回值类型也是T
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
