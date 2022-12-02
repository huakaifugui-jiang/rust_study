/*
 * @Author: wlj
 * @Date: 2022-12-02 10:30:28
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-02 11:10:11
 * @Description: 第三章练习
 */
fn main() {
    println!("生成 n 阶斐波那契数列 -------start")
    let f4: u32 = fibonacci(4);
    println!("f4:{f4}");

    let f11: u32 = fibonacci(11);
    println!("f11:{f11}");
    println!("生成 n 阶斐波那契数列 -------end")
}

// 生成 n 阶斐波那契数列 Fibonacci   下面Fn就表示第几个斐波那契数
//用文字来说，就是斐波那契数列由0和1开始，之后的斐波那契数就是由之前的两数相加而得出。首几个斐波那契数是
//1、 1、 2、 3、 5、 8、 13、 21、 34、 55、 89、 144、 233、 377、 610、 987……（OEIS数列A000045）
fn fibonacci(n: u32) -> u32 {
    //F0=0 和 F1 =0+1 固定的
    if n <= 1 {
        return n;
    }
    //F2开始递归 F2=F1+F0;
    return fibonacci(n - 1) + fibonacci(n - 2);
}