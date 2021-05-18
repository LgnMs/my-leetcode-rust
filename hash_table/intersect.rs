/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [350] 两个数组的交集 II
 * https://leetcode-cn.com/problems/intersection-of-two-arrays-ii/
 * - [哈希表]
 */
use std::collections::HashMap;

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut res = vec![];

    for x in nums1 {
        let count = match map.get_mut(&x) {
            Some(i) => *i + 1,
            None => 1,
        };
        map.insert(x, count);
    }

    for x in nums2 {
        match map.get_mut(&x) {
            None => {}
            Some(i) => {
                if *i != 0 {
                    res.push(x);
                    *i = *i - 1;
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::intersect::intersect;

    #[test]
    fn it_work_1() {
        // TODO 测试方法需要重写
        assert_eq!(intersect(vec![1,2,2,1], vec![2,2]), vec![2, 2]);
        assert_eq!(intersect(vec![4,9,5], vec![9,4,9,8,4]), vec![9, 4]);
    }
}