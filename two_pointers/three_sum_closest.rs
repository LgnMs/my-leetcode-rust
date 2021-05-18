/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [16] 最接近的三数之和
 * https://leetcode-cn.com/problems/3sum-closest/
 * - [双指针，排序]
 */

pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort();
    let len = nums.len();
    let mut ans = 10_i32.pow(4);

    for i in 0..len {
        let mut j = i + 1;
        let mut k = len - 1;

        while j < k {
            let sum = nums[i] + nums[j] + nums[k];
            if sum == target {
                return sum;
            } else if sum < target {
                j += 1;
            } else {
                k -= 1;
            }
            ans = if (ans - target).abs() < (sum - target).abs() {
                ans
            } else {
                sum
            };
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use crate::three_sum_closest::three_sum_closest;

    #[test]
    fn it_work_1() {
        assert_eq!(three_sum_closest(vec![-1,2,1,-4], 1), 2);
    }
    #[test]
    fn it_work_2() {
        assert_eq!(three_sum_closest(vec![0, 2, 1, -3], 1), 0);
    }
    #[test]
    fn it_work_3() {
        assert_eq!(three_sum_closest(vec![1,1,1,0], -100), 2);
    }
    #[test]
    fn it_work_4() {
        assert_eq!(three_sum_closest(vec![1,1,-1,-1,3], -1), -1);
    }
}