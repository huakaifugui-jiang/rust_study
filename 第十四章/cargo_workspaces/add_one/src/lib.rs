/*
 * @Author: wlj
 * @Date: 2022-12-19 14:33:22
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-19 14:40:17
 * @Description:
 */

use rand;
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
