/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [14] 罗马数字转整数
 * https://leetcode-cn.com/problems/longest-common-prefix/
 */

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 0 {
        return String::from("");
    }
    if strs.len() == 1 {
        return  strs[0].to_string();
    }
    let mut s = "";
    let mut min_len = 0;

    for i in 0..strs.len() {
        if strs[i] == "" {
            return String::from("");
        }
        if min_len == 0 {
            min_len = strs[i].len();
        } else if strs[i].len() < min_len {
            min_len = strs[i].len();
        }
    }
    for i in 0..min_len {
        s = &strs[0][0..i + 1];
        for k in 1..strs.len() {
            let slice = &strs[k][0..i + 1];
            if s != slice {
                if i == 0 {
                    return "".to_string();
                }
                return strs[0][0..i].to_string();
            }
        }
    }
    s.to_string()
}

#[cfg(test)]
mod tests {
    use crate::longest_common_prefix::longest_common_prefix;

    #[test]
    fn it_work_1() {
        assert_eq!(longest_common_prefix(vec!["flower", "flow", "flight"].iter().map(|x| x.to_string()).collect()), "fl");
        assert_eq!(longest_common_prefix(vec!["dog", "racecar", "car"].iter().map(|x| x.to_string()).collect()), "");
    }
    #[test]
    fn it_work_2() {
        assert_eq!(longest_common_prefix(vec!["ab", "a"].iter().map(|x| x.to_string()).collect()), "a");
        assert_eq!(longest_common_prefix(vec!["", "b"].iter().map(|x| x.to_string()).collect()), "");
    }
}