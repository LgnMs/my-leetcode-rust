/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [189] 旋转数组
 * https://leetcode-cn.com/problems/rotate-array/submissions/
 */
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let old_nums = nums.clone();
    let k = k as usize;

    for i in 0..len {
        nums[(i + k) % len] = old_nums[i];
    }
}

#[cfg(test)]
mod tests {
    use crate::rotate::rotate;

    #[test]
    fn it_work_1() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7];
        let mut v2 = vec![-1, -100, 3, 99];
        rotate(&mut v, 3);
        rotate(&mut v2, 2);
        assert_eq!(v, vec![5, 6, 7, 1, 2, 3, 4]);
        assert_eq!(v2, vec![3, 99, -1, -100]);
    }
}
