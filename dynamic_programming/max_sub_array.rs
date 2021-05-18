/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [53] 最大子序和
 * https://leetcode-cn.com/problems/maximum-subarray/
 * - [动态规划]
 */
use std::cmp::max;

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut ans = nums[0];
    let mut pre = 0;

    for x in nums {
        pre = max(x, pre + x);
        ans = max(ans, pre);
    }
    ans
}

#[cfg(test)]
mod tests {
    use crate::max_sub_array::max_sub_array;

    #[test]
    fn it_work_1() {
        assert_eq!(max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
    }
}