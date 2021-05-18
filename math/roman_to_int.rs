/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [13] 罗马数字转整数
 * https://leetcode-cn.com/problems/roman-to-integer/
 * - [rust:模式匹配]
 */


pub fn roman_to_int(s: String) -> i32 {
    let mut sum = 0;
    let b = s.as_bytes();

    for i in 0..b.len() {
        let num = match b[i] {
            b'I' => match b.get(i + 1) {
                Some(b'V') | Some(b'X') => -1,
                _ => 1,
            },
            b'V' => 5,
            b'X' => match b.get(i + 1) {
                Some(b'L') | Some(b'C') => -10,
                _ => 10
            },
            b'L' => 50,
            b'C' => match b.get(i + 1) {
                Some(b'D') | Some(b'M') => -100,
                _ => 100,
            },
            b'D' => 500,
            b'M' => 1000,
            _ => 0,
        };
        sum += num;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::roman_to_int::roman_to_int;

    #[test]
    fn it_works_1() {

        assert_eq!(roman_to_int(String::from("III")), 3);
        assert_eq!(roman_to_int(String::from("IV")), 4);
        assert_eq!(roman_to_int(String::from("IX")), 9);
        assert_eq!(roman_to_int(String::from("LVIII")), 58);
        assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
    }
}