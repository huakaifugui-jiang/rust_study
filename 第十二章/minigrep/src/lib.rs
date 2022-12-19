/*
 * @Author: wulongjiang
 * @Date: 2022-12-15 21:52:02
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-19 08:43:15
 * @Description:
 * @FilePath: \minigrep\src\lib.rs
 */
use std::{
    env::{self},
    error::Error,
    fs,
};
//目前只需知道 Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型，不过无需指定具体将会返回的值的类型
//这提供了在不同的错误场景可能有不同类型的错误返回值的灵活性。这也就是 dyn，它是 “动态的”（“dynamic”）的缩写
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool, //是否敏感大小写
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        //这里创建了一个新变量 case_sensitive。为了设置它的值，需要调用 env::var 函数并传递我们需要寻找的环境变量名称
        //env::var 返回一个 Result，它在环境变量被设置时返回包含其值的 Ok 成员，并在环境变量未被设置时返回 Err 成员
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); //我们使用 Result 的 is_err 方法来检查其是否是一个 error（也就是环境变量未被设置的情况）
                                                                    //如果CASE_INSENSITIVE 环境变量被设置为任何值，is_err 会返回 false 并将进行大小写不敏感搜索。我们并不关心环境变量所设置的 值，
                                                                    //只关心它是否被设置了，所以检查 is_err 而不是 unwrap、expect 或任何我们已经见过的 Result 的方法

        Ok(Config {
            query,
            filename,
            case_sensitive, //我们将变量 case_sensitive 的值传递给 Config 实例，这样 run 函数可以读取其值并决定是否调用 search
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //读取文件
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

//因为返回值跟contents是关联的 所以contens一定要加'a注解
//遍历内容的每一行文本。 lines
// 查看这一行是否包含要搜索的字符串。
// 如果有，将这一行加入列表返回值中。
// 如果没有，什么也不做。
// 返回匹配到的结果列表
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // //lines 方法返回一个迭代器。
    // for line in contents.lines() {
    //     if compile_error!() {
    //         results.push(line);
    //     }
    // }
    // results

    //改进 闭包加迭代器  迭代器的性能要高于for循环版本
    //迭代器，作为一个高级的抽象，被编译成了与手写的底层代码大体一致性能代码
    //迭代器是 Rust 的 零成本抽象（zero-cost abstractions）之一，它意味着抽象并不会引入运行时开销，它与本贾尼·斯特劳斯特卢普（C++ 的设计和实现者）在 “Foundations of C++”（2012） 中所定义的 零开销（zero-overhead）如出一辙
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    //lines 方法返回一个迭代器。
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three."; //（注意双引号之后的反斜杠，这告诉 Rust 不要在字符串字面值内容的开头加入换行符）
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    //大小写不敏感的测试函数
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
