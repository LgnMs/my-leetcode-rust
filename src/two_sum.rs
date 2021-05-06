/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 * https://leetcode-cn.com/problems/two-sum/
 */

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = vec![];
    let len = nums.len();

    for i in 0..len {
        for k in i + 1..len {
            if nums[i] + nums[k] == target {
                result = vec![i as i32, k as i32];
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::two_sum::{two_sum};

    #[test]
    fn it_works_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        assert_eq!(vec![0, 1], two_sum(nums, target));
    }
    #[test]
    fn it_works_2() {
        let nums = vec![3,2,4];
        let target = 6;

        assert_eq!(vec![1, 2], two_sum(nums, target));
    }
    #[test]
    fn it_works_3() {
        let nums = vec![3,3];
        let target = 6;

        assert_eq!(vec![0, 1], two_sum(nums, target));
    }
}