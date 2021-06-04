/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * 198. 打家劫舍
 * https://leetcode-cn.com/problems/house-robber/
 */

use std::cmp::max;

pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    if nums.len() == 2 {
        return max(nums[0], nums[1]);
    }
    let mut dp = vec![nums[0], max(nums[0], nums[1])];
    let len = nums.len();

    for i in 2..len  {
        dp.push(max(nums[i] + dp[i - 2], dp[i - 1]));
    }

    dp[dp.len() - 1]
}

#[cfg(test)]
mod test {
    use super::rob;

    #[test]
    fn it_work_1() {
        assert_eq!(rob(vec![1,2,3,1]), 4);
    }
    #[test]
    fn it_work_2() {
        assert_eq!(rob(vec![2,7,9,3,1]), 12);
    }
}
