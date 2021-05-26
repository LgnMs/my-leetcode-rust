/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [7] 整数反转
 * https://leetcode-cn.com/problems/reverse-integer/
 * - [数学]
 */

pub fn reverse(x: i32) -> i32 {
    let res = x
        .abs()
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<i32>()
        .unwrap_or(0)
        * x.signum();

    if res < i32::MIN || res > i32::MAX {
        return 0;
    }
    res
}

#[cfg(test)]
mod test {
    use crate::reverse::reverse;

    #[test]
    fn it_work_1() {
        assert_eq!(reverse(-123), -321);
    }
}
