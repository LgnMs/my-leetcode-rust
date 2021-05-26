/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [242] 有效的字母异位词
 * https://leetcode-cn.com/problems/valid-anagram/
 */
pub fn is_anagram(s: String, t: String) -> bool {
    let mut s_b = s.into_bytes();
    let mut t_b = t.into_bytes();
    if s_b.len() != t_b.len() {
        return false;
    }
    s_b.sort();
    t_b.sort();

    s_b == t_b
}

#[cfg(test)]
mod test {
    use crate::is_anagram::is_anagram;

    #[test]
    fn it_work_1() {
        assert_eq!(
            is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
    }
}
