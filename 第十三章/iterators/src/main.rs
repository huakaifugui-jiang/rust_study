/*
 * @Author: wulongjiang
 * @Date: 2022-12-18 21:15:09
 * @LastEditors: wulongjiang
 * @LastEditTime: 2022-12-18 22:27:19
 * @Description: 使用迭代器处理元素序列
 * @see:https://kaisery.github.io/trpl-zh-cn/ch13-02-iterators.html
 * @FilePath: \iterators\src\main.rs
 */

//迭代器模式允许你对一个序列的项进行某些处理。迭代器（iterator）负责遍历序列中的每一项和决定序列何时结束的逻辑。当使用迭代器时，我们无需重新实现这些逻辑。

//在Rust中，迭代器时惰性的（lazy），这意味着在调用方法使用迭代器之前它都不会有效果。
fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); //调用定义于 Vec 上的 iter 方法在一个 vector v1 上创建了一个迭代器。

    for val in v1_iter {
        println!("Got: {}", val);
    }

    //在标准库中没有提供迭代器的语言中，我们可能会使用一个从 0 开始的索引变量，使用这个变量索引 vector 中的值，并循环增加其值直到达到 vector 的元素数量。

    //迭代器都实现了一个叫做 Iterator 的定义于标准库的 trait。
    // pub trait Iterator {
    //     type Item;

    //     fn next(&mut self) -> Option<Self::Item>;

    //     // 此处省略了方法的默认实现
    // }

    //这里有一个我们还未讲到的新语法：type Item 和 Selt::Item。 他们定义了trait的关联类型(associated type) 第十九章会深入讲解关联类型，
    //不过现在只需知道这段代码表明实现 Iterator trait 要求同时定义一个 Item 类型，这个 Item 类型被用作 next 方法的返回值类型。换句话说，Item 类型将是迭代器返回元素的类型。

    //next 是 Iterator 实现者被要求定义的唯一方法。next一次返回迭代器的一个项，封装在Some中，当迭代器结束时，它返回None
    let v2 = vec![1, 2, 3];
    let mut v2_iter = v2.iter();
    println!("{:?}", v2_iter);
    println!("{:?}", v2_iter.next()); //返回一个Some（1）
    assert_eq!(v2_iter.next(), Some(&2));
    assert_eq!(v2_iter.next(), Some(&3));
    assert_eq!(v2_iter.next(), None);
    //v2_iter是需要可变的:在迭代器上调用 next 方法改变了迭代器中用来记录序列位置的状态。
    //换句话说，代码 消费（consume）了，或使用了迭代器。每一个 next 调用都会从迭代器中消费一个项。
    println!("{:?}", v2_iter);

    //使用 for 循环时无需使 v1_iter 可变因为 for 循环会获取 v1_iter 的所有权并在后台使 v1_iter 可变
    //另外需要注意到从 next 调用中得到的值是 vector 的不可变引用。iter 方法生成一个不可变引用的迭代器。
    //如果我们需要一个获取 v2 所有权并返回拥有所有权的迭代器，则可以调用 into_iter 而不是 iter。类似的，
    //如果我们希望迭代可变引用，则可以调用 iter_mut 而不是 iter

    //消费迭代器的方法
    //Iterator trait 有一系列不同的由标准库提供默认实现的方法；你可以在 Iterator trait 的标准库 API 文档中找到所有这些方法
    //一些方法在其定义中调用了 next 方法，这也就是为什么在实现 Iterator trait 时要求实现 next 方法的原因

    //这些调用 next 方法的方法被称为 消费适配器（consuming adaptors），因为调用他们会消耗迭代器。
    //一个消费适配器的例子是 sum 方法。这个方法获取迭代器的所有权并反复调用 next 来遍历迭代器，因而会消费迭代器。
    //当其遍历每一个项时，它将每一个项加总到一个总和并在迭代完成时返回总和。
    let v3 = vec![1, 2, 3];
    let v3_iter = v3.iter();
    let total: i32 = v3_iter.sum(); //调用 sum 方法获取迭代器所有项的总和 获取迭代器v3_iter的所有权
    println!("{}", total);
    // println!("{:?}",v3_iter);//报错 因为它的所有权已经被获取了 value borrowed here after move

    //产生其他迭代器的方法
    //Iterator trait 中定义了另一类方法，被称为 迭代器适配器（iterator adaptors）。他们允许我们将当前迭代器变为不同类型的迭代器。可以链式调用多个迭代器适配器。
    //不过因为所有的迭代器都是惰性的，必须调用一个消费适配器方法以便获取迭代器适配器调用的结果。

    //如下展示了一个调用迭代器适配器的方法map例子，该map方法使用闭包来调用每个元素以生成新的迭代器。
    let v4 = vec![1, 2, 3];
    //map 获取一个闭包，可以指定任何希望在遍历的每个元素上执行的操作
    let v4_map = v4.iter().map(|x| x + 1); // 这里的闭包创建了一个新的迭代器，对其中 vector 中的每个元素都被加 1。不过这些代码会产生一个警告迭代器适配器是惰性的，而这里我们需要消费迭代器。
                                           //iterators are lazy and do nothing unless consumed

    //为了修复这个警告我们可以使用collect方法。这个方法消费迭代器并将结果收集到一个数据结构中。
    let v5: Vec<_> = v4_map.collect();
    println!("{:?}", v5);

    //使用闭包获取环境
    //现在我们介绍了迭代器，让我们展示一个通过使用 filter 迭代器适配器和捕获环境的闭包的常规用例。
    //迭代器的 filter 方法获取一个使用迭代器的每一个项并返回布尔值的闭包。
    //如果闭包返回 true，其值将会包含在 filter 提供的新迭代器中。如果闭包返回 false，其值不会包含在结果迭代器中。

    #[derive(Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let shoe_size = 10;
    let shoes_in_size: Vec<Shoe> = shoes.into_iter().filter(|x| x.size == shoe_size).collect();
    //shoes_in_my_size 函数体中调用了 into_iter 来创建一个获取 vector 所有权的迭代器。
    //接着调用 filter 将这个迭代器适配成一个只含有那些闭包返回 true 的元素的新迭代器。
    println!("{:?}", shoes_in_size);

    //实现 Iterator trait 来创建自定义迭代器
    //我们已经展示了可以通过在 vector 上调用 iter、into_iter 或 iter_mut 来创建一个迭代器。也可以用标准库中其他的集合类型创建迭代器，比如哈希 map。
    //另外，可以实现 Iterator trait 来创建任何我们希望的迭代器。正如之前提到的，定义中唯一要求提供的方法就是 next 方法。
    //一旦定义了它，就可以使用所有其他由 Iterator trait 提供的拥有默认实现的方法来创建自定义迭代器了

    //作为展示，让我们创建一个只会从1数到5的迭代器。首先，创建一个结构体来存放一些值，接着实现Iterator trait 将这个结构体放入迭代器中并在此实现中使用其值。
    //有一个 Counter 结构体定义和一个创建 Counter 实例的关联函数 new
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32; //这里将迭代器的关联类型 Item 设置为 u32，意味着迭代器会返回 u32 值集合。
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count = self.count + 1;
                Some(self.count) //我们希望迭代器对其内部状态加一，这也就是为何将 count 初始化为 0：我们希望迭代器首先返回 1。
            } else {
                None //如果 count 值小于 6，next 会返回封装在 Some 中的当前值，不过如果 count 大于或等于 6，迭代器会返回 None。
            }
        }
    }

    let mut counter = Counter::new();
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());

    //通过定义 next 方法实现 Iterator trait，我们现在就可以使用任何标准库定义的拥有默认实现的 Iterator trait 方法了
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
