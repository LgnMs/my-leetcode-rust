/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * 88. 合并两个有序数组
 * https://leetcode-cn.com/problems/merge-sorted-array/
 */
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut m = m as usize;
    let mut n = n as usize;
    let mut right = nums1.len();
    while n > 0 {
        right -= 1;
        if m == 0 || nums1[m - 1] < nums2[n - 1] {
            nums1[right] = nums2[n - 1];
            if n > 0 { n -= 1 }
        } else {
            nums1.swap(m - 1, right);
            if m > 0 { m -= 1 }
        }
    }
}

#[cfg(test)]
mod test {
    use super::merge;
    #[test]
    fn it_work_1() {
        let mut nums1 = vec![1,2,3,0,0,0];
        let mut nums2 = vec![2,5,6];
        merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1,2,2,3,5,6]);
    }
}