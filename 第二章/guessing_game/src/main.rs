/*
 * @Author: wlj
 * @Date: 2022-11-29 14:49:43
 * @LastEditors: wulongjiang
 * @LastEditTime: 2022-11-30 22:43:25
 * @Description: 猜数字游戏
 */
// use 声明
use std::io; //将io(输入/输出)库引入当前作用域。io库来自于标准库(Standard library)，也被称作std
use rand::Rng; //Rng（Random Numeral Generator）随机数生成器 是一个trait（）它定义了随机数生成器应实现的方法
use std::cmp::Ordering; //从标准库引入了一个叫做 std::cmp::Ordering 的类型到作用域中 标准库::比较::排序  Ordering是一个枚举(它的成员有[Less(小),Greater(大),Equal(相等)])

fn main() {
    println!("Guess the number!!");

    let secret_number = rand::thread_rng()  //调用rand库的随机数生成器，它位于当前执行线程的本地环境中，并从操作系统获取seed （意思是从操作系统中随机获取一个种子
    .gen_range(1..=100);//方法获取一个范围表达式（range expression）作为参数，并生成一个在此范围之间的随机数。这里使用的这类范围表达式使用了 start..=end 这样的形式
    //gen_range方法是由use rand::Rng引入到作用域的Rng trait定义
    println!("The secret number is: {secret_number}");

    //创建一个循环来允许多次猜测 强行跳出循环的方法1.在终端种按ctrl + c 
    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new();
    
        io::stdin() //如果不引入use std::io的话 可以写作 std::io::stdin
        .read_line(&mut guess)  //调用read_line方法（获取用户输入）并且将&mut guess 作为参数传递给read_line()函数，让其将用户输入储存到这个字符串中
        .expect("Failed to read line");
    
        // String 实例的 trim 方法会去除字符串开头和结尾的空白字符
        //为什么要先trim呢？ 因为：用户必须输入enter键才能让read_line返回他们的猜想 这时候字符串会增加一个回车和换行符(\r\n) 所以需要消除这个
        //parse方法将字符串转为其他类型, 这里通过 let guess: u32(无符号的32位整数) 指定,guess 后面的冒号（:）告诉 Rust 我们指定了变量的类型 它会返回一个Result类型
        // 最后使用expect类型对Resut类型进行处理如果 parse 不能从字符串生成一个数字，返回一个 Result 的 Err 成员时，expect 会使游戏崩溃并打印附带的信息。如果 parse 成功地将字符串转换为一个数字，它会返回 Result 的 Ok 成员，然后 expect 会返回 Ok 值中的数字
        // let guess:u32 = guess.trim().parse().expect("Please type a number!");

        //处理无效输出 改善一下游戏
        //一个 match 表达式由 分支（arms） 构成。一个分支包含一个 模式（pattern）和表达式开头的值与分支模式相匹配时应该执行的代码。
        //Rust 获取提供给 match 的值并挨个检查每个分支的模式。
        //比如guess.trim().parse() 会返回一个Result类型 Result类型拥有[Ok 或 Err]枚举 个人感觉有点像switch(Result类型的某个成员)
        let guess:u32 =match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            //_ 是一个通配符值，本例中用来匹配所有 Err 值，不管其中有何种信息。所以程序会执行第二个分支的动作，
            //continue 意味着进入 loop 的下一次循环，请求另一个猜测。这样程序就有效的忽略了 parse 可能遇到的所有错误！
        };
    
        println!("You guessed:{guess}");
    
        //没有将guess(32行)进行强制类型转换之前的注解：
        //此处要注意因为rust是一个静态强类型语言(编写代码的时候要明确确定变量的数据类型，并且一旦指定类型不可更改（如果要改变需要强制类型转换）)
        //rust也有类型推断 因为 secret_number 没有明确类型，所以类型推断出他是数字类型 并且默认是i32(32位数字)类型,所以此时会报错（expected struct `String`, found integer）
        
        //将guess强制类型转换后：
        //因为secret_number没有指明类型 所以 rust会根据guess的类型 推断出他是 u32数字类型 现在可以使用相同类型比较两个值了！
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;//跳出循环
            },
        }
    }


}  
