/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [125] 验证回文串
 * https://leetcode-cn.com/problems/valid-palindrome/
 * - [数学]
 */

pub fn is_palindrome(s: String) -> bool {
    let mut i = 0;
    let mut k = s.len() - 1;
    let chars = s.to_lowercase().into_bytes();

    fn is_letter_or_number(c: u8) -> bool {
        (c >= 97 && c <= 122) || (c >= 48 && c <= 57)
    }

    while i < k {
        if !is_letter_or_number(chars[i]) {
            i += 1;
            continue;
        }
        if !is_letter_or_number(chars[k]) {
            k -= 1;
            continue;
        }

        if chars[i] != chars[k] {
            return false;
        }
        i += 1;
        k -= 1;
    }
    true
}

#[cfg(test)]
mod test {
    use crate::is_palindrome::is_palindrome;

    #[test]
    fn it_work_1() {
        assert_eq!(
            is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
    }
    #[test]
    fn it_work_2() {
        assert_eq!(is_palindrome("OP".to_string()), false);
    }
    #[test]
    fn it_work_3() {
        assert_eq!(is_palindrome("0P".to_string()), false);
    }
}
