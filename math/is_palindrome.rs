/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [9] 回文数
 * https://leetcode-cn.com/problems/palindrome-number/
 */

pub fn is_palindrome(x: i32) -> bool {
    if x > 0 && x < 10 {
        return true;
    }
    if x.to_string().chars().rev().collect::<String>() == x.to_string() {
        true
    } else {
        false
    }
}

#[cfg(test)]

mod tests {
    use super::is_palindrome;

    #[test]
    fn it_work_1() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(-121), false);
        assert_eq!(is_palindrome(10), false);
        assert_eq!(is_palindrome(-101), false);
    }
}
