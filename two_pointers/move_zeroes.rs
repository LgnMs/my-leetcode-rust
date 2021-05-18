/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [283] 移动零
 * https://leetcode-cn.com/problems/move-zeroes
 */

pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut i = 0;
    let mut len = nums.len();
    while i < len - 1 {
        if nums[i] == 0 {
            nums.remove(i);
            nums.push(0);
            len -= 1;
        } else {
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::move_zeroes::move_zeroes;

    #[test]
    fn it_work_1() {
        let mut v = vec![0,1,0,3,12];
        move_zeroes(&mut v);
        assert_eq!(v, vec![1,3,12,0,0]);
    }
}