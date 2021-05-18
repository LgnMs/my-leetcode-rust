/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [217] 存在重复元素
 * https://leetcode-cn.com/problems/contains-duplicate/
 */
pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
    nums.sort();
    nums.windows(2).fold(false, |res, y| {
        (y[0] == y[1]) | res
    })
}

#[cfg(test)]
mod tests {
    use crate::contains_duplicate::contains_duplicate;

    #[test]
    fn it_work_1() {
        assert_eq!(contains_duplicate(vec![2,14,18,22,22]), true);
        assert_eq!(contains_duplicate(vec![3,3]), true);
        assert_eq!(contains_duplicate(vec![1]), false);
    }
}