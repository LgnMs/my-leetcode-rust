/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [3] 无重复字符的最长子串
 * https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/
 * - [滑动窗口] [哈希表]
 */

use std::collections::{HashSet, VecDeque};

pub fn length_of_longest_substring(s: String) -> i32 {
    let s = s.into_bytes();
    let mut max = 0;
    let mut q: VecDeque<u8> = VecDeque::new();
    let mut set: HashSet<u8> = HashSet::new();
    for i in 0..s.len() {
        if set.contains(&s[i]) {
            // q.pop_front()返回option类型的数据，当返回为None的时候循环停止
            while let Some(x) = q.pop_front() {
                set.remove(&x);
                if x == s[i] {
                    break;
                }
            }
        }
        set.insert(s[i]);
        q.push_back(s[i]);
        max = max.max(q.len());
    }
    max as i32
}

#[cfg(test)]
mod tests {
    use crate::length_of_longest_substring::length_of_longest_substring;

    #[test]
    fn it_works_1() {
        let s = String::from("abcabcbb");

        assert_eq!(3, length_of_longest_substring(s));
    }
    #[test]
    fn it_works_2() {
        let s = String::from("bbbbb");

        assert_eq!(1, length_of_longest_substring(s));
    }
    #[test]
    fn it_works_3() {
        let s = String::from("pwwkew");

        assert_eq!(3, length_of_longest_substring(s));
    }
    #[test]
    fn it_works_4() {
        let s = String::from(" ");

        assert_eq!(1, length_of_longest_substring(s));
    }
    #[test]
    fn it_works_5() {
        let s = String::from("au");

        assert_eq!(2, length_of_longest_substring(s));
    }
}
