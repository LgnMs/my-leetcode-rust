/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [344] 反转字符串
 * https://leetcode-cn.com/problems/reverse-string/
 */
pub fn reverse_string(s: &mut Vec<char>) {
    let len = s.len();
    for i in 0..len / 2 {
        let k = len - 1 - i;
        let temp = s[i];
        s[i] = s[k];
        s[k] = temp;
    }
}

#[cfg(test)]
mod tests {
    use crate::reverse_string::reverse_string;

    #[test]
    fn it_work_1() {
        let mut v = vec!['h','e','l','l','o'];
        let mut k = v.clone();
        reverse_string(&mut v);
        k.reverse();
        assert_eq!(v, k);
    }
}