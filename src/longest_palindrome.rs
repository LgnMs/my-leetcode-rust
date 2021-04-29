
/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [5] 最长回文子串
 * https://leetcode-cn.com/problems/two-sum/
 * - [动态规划]
 */

pub fn is_palindrome(s: &[char]) -> bool {
    for i in 0..s.len() / 2 {
        if s[i] != s[s.len() - i - 1]{
            return false;
        }
    }
    return true;
}
pub fn longest_palindrome(s: String) -> String {
    if s == "" {
        return String::new();
    }
    let s:Vec<char>=s.chars().collect();
    for n in (0..s.len() + 1).rev() {
        for i in 0..s.len() - n + 1 {
            if s[i] == s[i + n - 1 ]{
                if is_palindrome(&s[i..i + n]) {
                    return s[i..i + n].into_iter().collect();
                }

            }
        }
    }
    return String::new();
}
#[cfg(test)]
mod tests {
    use crate::longest_palindrome::{longest_palindrome};

    #[test]
    fn it_works_1() {
        let s = "babad";

        assert_eq!("bab".to_string(), longest_palindrome(s.to_string()));
    }
    #[test]
    fn it_works_2() {
        let s = "cbbd";

        assert_eq!("bb".to_string(), longest_palindrome(s.to_string()));
    }
    #[test]
    fn it_works_3() {
        let s = "a";

        assert_eq!("a".to_string(), longest_palindrome(s.to_string()));
    }

    #[test]
    fn it_works_4() {
        let s = "ac";

        assert_eq!("a".to_string(), longest_palindrome(s.to_string()));
    }

}