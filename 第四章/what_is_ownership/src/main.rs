/*
 * @Author: wlj
 * @Date: 2022-12-02 15:02:10
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-02 17:09:37
 * @Description: 什么是所有权？
 * @see：https://kaisery.github.io/trpl-zh-cn/ch04-01-what-is-ownership.html
 */

//所有程序都必须管理其运行时使用计算机内存的方式。 在Rust没出来之前一共有两个方式对内存管理
//一些语言中具有垃圾回收机制(GC)通过一些算法（如引用计数法等）对内存进行管理 如js，java，go
//在另一些语言，程序员必须亲自分配和释放内存， 如C C++。

//Rust 另辟蹊径 选择了第三种方式：通过所有权（ownership）系统来管理内存，编译器在编译时会根据一系列的规则进行检查。
//如果违反了任何这些规则，程序都不能编译。并且在运行时，所有权系统的任何功能都不会减慢程序。

//什么是所有权系统呢？
//我们得先来简单的了解一些堆与栈。
//堆和栈都是运行时可供使用的内存，但是他们的结构不太一样

//栈 stack
// 栈以放入值的顺序存储并且以相反的顺序取出值。这也被称作后进先出（LILO:first in last out).
// 可以简单想象就比如 子弹弹夹 先装入的子弹都是最后射出的。  或者叠盘子：当增加更多的盘子时都是放在盘子的顶部，当需要的时候拿出盘子，也是先从顶部拿走。！不能从中间也不能从底部增加或拿走盘子！
// 这样增加的数据叫做  进栈 (pushing onto the stack)或压栈，而移出数据叫做出栈（popping off the stack）.
// 要注意的是 栈中的所有数据都必须占用 已知 且 固定 的大小，是静态的。 因为栈的存储容量是有限的

//堆 heap
// 所以 因为 栈只能存放固定大小的数据，那么一些不确定大小的数据要存放在哪里呢？ 答案就是堆。
// 当向堆放入数据时，你需要请求一定大小的空间。内存分配器（memory allocator）在堆的某处找到一块足够大的空位，把它标记为已使用，
//！！！并且返回一个表示该位置地址的指针（pinter）。这个过程称作 在堆上分配内存（allocating on the heap）有时简称为 “分配”（allocating）。（将数据推入栈中并不被认为是分配）。这是动态的。
// 因为 指向堆中数据的 指针 是已知的且大小固定的，你可以讲该指针存储在 栈 上。 当需要实际数据时，必须访问指针
// 想象一下去餐馆就座吃饭。当进入时，你说明有几个人，餐馆员工会找到一个够大的空桌子并领你们过去。如果有人来迟了，他们也可以通过询问来找到你们坐在哪。

//入栈比在堆上分配内存要快，因为（入栈时）分配器不需要去 为存储新的数据而去搜索内存空间。它的位置总是在栈顶。
//相比之下，在堆上分配内存则需要更多的工作，这是因为分配器必须首先找到一块足够存放数据的内存空间，并接着做一些记录为下一次分配做准备

//同理的 访问堆上的数据比访问栈上的数据慢，因为它必须先到栈中去 获取 指针 来访问。

//当你的代码调用一个函数时，传递给函数的值（包括可能指向堆上数据的指针）和函数的局部变量被压入栈中。当函数结束时，这些值被移出栈。

//跟踪 哪部分代码 正在使用堆上的 哪些数据，最大限度的减少堆上的重复数据的数量，以及清理堆上 不再使用的数据 确保不会耗尽空间，这些问题正是所有权系统需要处理的。
//一旦理解了所有权，你就不需要经常考虑栈和堆了，不过明白了所有权的主要目的就是为了管理堆数据，能够帮助解释为什么所有权要以这种方式工作。


// 所有权的规则：
//1.Rust中的每一个值都有一个所有者（owner）。
//2.值在任一时刻有且只有一个所有者。
//3.当所有者（变量）离开作用域，这个值将被丢弃。

//变量作用域
fn learn_variables_scope() {
    let s = "hello"
}
