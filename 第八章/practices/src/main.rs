/*
 * @Author: wulongjiang
 * @Date: 2022-12-10 12:39:02
 * @LastEditors: wulongjiang
 * @LastEditTime: 2022-12-10 17:15:33
 * @Description:第八章课后练习
 * @FilePath: \practices\src\main.rs
 */
use std::collections::HashMap;
use std::io;

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 5, 2, 3, 2];

    println!("{:?}", practice_1(nums));

    println!("{}", practice_2(&mut String::from("first")));
    println!("{}", practice_2(&mut String::from("apple")));

    //练习3
    //使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。
    //接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("练习3   输入Add {{员工}} to {{部门}}");
        let mut cmd = String::new(); //用户输入的命令值

        io::stdin().read_line(&mut cmd).expect("Fail to read"); //获取用户输入的命令
        let cmd_vec: Vec<&str> = cmd.split_whitespace().collect();

        if cmd_vec[0] != "Add" || cmd_vec[2] != "to" {
            println!("输入错误,请重新输入！");
            continue;
        }

        let ptr = company.entry(cmd_vec[3].to_string()).or_insert(Vec::new());
        ptr.push(cmd_vec[1].to_string());

        println!("{:?}", company);
    }
}

/**
 * @description:练习一：给定一系列数字，使用vector并返回这个列表的中位数（排列数组后位于中间的值）和众数(mode,出现次数最多的值；这里哈希map会很有帮助)
 * @param {Vec} nums
 * @return (中位数，众数)
 */
fn practice_1(nums: Vec<i32>) -> (i32, i32) {
    let mut nums = nums;
    nums.sort();
    let median = nums[nums.len() / 2]; //中位数

    let mut counts = HashMap::new();

    for &i in &nums {
        let count = counts.entry(i).or_insert(0); //获取引用地址
        *count += 1;
    }

    let mut mode = nums[0]; //众数
    let mut mode_count = 0; //计算众数出现的个数

    for (&num, &count) in &counts {
        if count > mode_count {
            mode_count = count;
            mode = num;
        }
    }

    return (median, mode);
}

/**
 * @description: 将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，所以 “first” 会变成 “irst-fay”。
 * 元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。牢记 UTF-8 编码！（元音字母有a e i o u)
 * @return {*}
 */
fn practice_2(word: &mut String) -> String {
    let vowels_arr = ["a", "e", "i", "o", "u"]; //元音数组
    let first_word = String::from(&word[0..1]);
    let mut other_word = String::from(&word[1..]);
    let mut result = String::new();
    for i in vowels_arr {
        if first_word == i {
            word.push_str("-hay");
            result = word.to_string();
            return result;
        }
    }

    other_word.push_str("-fay");
    result = other_word;
    return result;
}
