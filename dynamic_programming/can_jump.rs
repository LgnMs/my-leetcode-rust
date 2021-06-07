/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * 55. 跳跃游戏
 * https://leetcode-cn.com/problems/climbing-stairs/
 * - [动态规划]
 */
use std::{cmp::max};

pub fn can_jump(nums: Vec<i32>) -> bool {
    let len = nums.len();
    let mut rightmost = 0;

    for i in 0..len {
        if i <= rightmost {
            rightmost = max(rightmost, i + nums[i] as usize);
            if rightmost >= len - 1 {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod test {
    use crate::can_jump::can_jump;

    #[test]
    fn it_work_1() {
        assert_eq!(can_jump(vec![0, 1]), false);
    }
    #[test]
    fn it_work_2() {
        assert_eq!(can_jump(vec![2, 0]), true);
    }
    #[test]
    fn it_work_3() {
        assert_eq!(can_jump(vec![2,3,1,1,4]), true);
    }
    #[test]
    fn it_work_4() {
        assert_eq!(can_jump(vec![3,2,1,0,4]), false);
    }
    #[test]
    fn it_work_5() {
        assert_eq!(can_jump(vec![3,0,8,2,0,0,1]), true);
    }
}