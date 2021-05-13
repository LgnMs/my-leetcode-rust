/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [70] 爬楼梯
 * https://leetcode-cn.com/problems/climbing-stairs/
 * - [动态规划]
 */
pub fn climb_stairs(n: i32) -> i32 {
    // 状态转移方程：f(x) = f(x - 1) + f(x - 2)
    if n < 3 {
        return n;
    }
    let mut nums = vec![1, 2];
    let len = n as usize;


    for x in 2..len {
        nums.push(nums[x - 2] + nums[x - 1]);
    }
    nums[len - 1]
}

#[cfg(test)]
mod tests {
    use crate::climb_stairs::climb_stairs;

    #[test]
    fn it_work_1() {
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
    }
}