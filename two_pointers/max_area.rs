/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [11] 盛最多水的容器
 * https://leetcode-cn.com/problems/container-with-most-water/
 * - [双指针]
 */

use std::cmp::{max, min};

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_number = 0;
    let len = height.len();
    let mut i = 0;
    let mut k = len - 1 - i;

    while k > i {
        let ai = height[i];
        let ki = height[k];

        max_number = max(max_number, (k - i) as i32 * min(ai, ki));

        if ai < ki {
            i += 1;
        } else {
            k -= 1;
        }
    }

    max_number
}

#[cfg(test)]
mod tests {
    use crate::max_area::max_area;

    #[test]
    fn it_work_1() {
        assert_eq!(max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
        assert_eq!(max_area(vec![1,1]), 1);
        assert_eq!(max_area(vec![4,3,2,1,4]), 16);
        assert_eq!(max_area(vec![1,2,1]), 2);
    }
    #[test]
    fn it_work_2() {
        assert_eq!(max_area(vec![1, 2]), 1);
    }
}