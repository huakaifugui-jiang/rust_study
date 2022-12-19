/*
 * @Author: wlj
 * @Date: 2022-12-19 14:27:55
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-19 14:36:03
 * @Description: 二进制crate
 */
use add_one;
fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
}
