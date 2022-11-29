/*
 * @Author: wlj
 * @Date: 2022-11-29 14:49:43
 * @LastEditors: wlj
 * @LastEditTime: 2022-11-29 17:38:49
 * @Description: 
 */
use std::io; //将io(输入/输出)库引入当前作用域。io库来自于标准库(Standard library)，也被称作std

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin() //如果不引入use std::io的话 可以写作 std::io::stdin
    .read_line(&mut guess)  //调用read_line方法（获取用户输入）并且将&mut guess 作为参数传递给read_line()函数，让其将用户输入储存到这个字符串中
    .expect("Failed to read line");

    println!("You guessed:{guess}");
}  
