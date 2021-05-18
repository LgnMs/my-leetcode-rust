/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [136] 只出现一次的数字
 * https://leetcode-cn.com/problems/single-number/
 * - [位运算，异或运算符]
 * 根据题意，所有元素 异或运算出的结果就是唯一出现过一次的数字
 */
pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |x, y| {
        x ^ y
    })
}

#[cfg(test)]
mod tests {
    use crate::single_number::single_number;

    #[test]
    fn it_work_1() {
        assert_eq!(single_number(vec![2,2,1]), 1);
        assert_eq!(single_number(vec![4,1,2,1,2]), 4);
        assert_eq!(single_number(vec![1]), 1);
    }
}