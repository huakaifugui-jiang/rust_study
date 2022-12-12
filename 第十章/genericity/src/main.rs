/*
 * @Author: wlj
 * @Date: 2022-12-12 09:48:27
 * @LastEditors: wulongjiang
 * @LastEditTime: 2022-12-12 20:52:54
 * @Description: 泛型数据类型
 * @see:https://kaisery.github.io/trpl-zh-cn/ch10-01-syntax.html
 */

//我们可以使用泛型为像函数签名 或者结构体这样的项 创建定义，这样它们就可以用于多种不同的具体数据类型。

//结构体泛型
// struct Point<T> {
//     x: T,
//     y: T,
// } //此时x，y都是同一类型
struct Point<T, U> {
    x: T,
    y: U,
} //使用两个泛型的 Point，这样 x 和 y 可能是不同类型

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

//此方法只有在两个类型都是f32的时候才能拥有
impl Point<f64, f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

//结构体定义的泛型类型参数并不总是和结构体方法签名中使用的泛型是同一类型。
//Point2结构体使用了泛型类型X1和Y1,为mixup方法签名使用了X2和Y2来使得示例更加清楚。
//这个方法用self和Point2类型的x值(类型X1)和参数Point2类型的y值（类型Y2）来创建一个新的Point2
struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

//这里的泛型参数X1和Y1声名于impl之后，因为他们于结构体定义相对应。而泛型参数X和Y2声名于fn mixup之后，因为他们知识相对于方法本身的
impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    // let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    // let result = largest(&char_list);
    println!("The largest char is {}", result);

    let a = Point { x: 1, y: 1.1 };
    let b = Point { x: 1.1, y: 1.1 };
    println!("{}", a.x());
    // println!("{}", a.distance_from_origin()); //method not found in `Point<{integer}, {float}>` 报错
    println!("{}", b.distance_from_origin());

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
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
// fn largest<T>(list: &[T]) -> T {
//     //函数largest 有泛型T。它有个参数list，其类型是元素为T的slice，largest函数的返回值类型也是T
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             //此时会报错因为largest不能适用于T的所有可能的类型，需要比较T类型的值 标准库中定义的 std::cmp::PartialOrd trait 可以实现类型的比较功能。
//             largest = item;
//         }
//     }

//     largest
// }
