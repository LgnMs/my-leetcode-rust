/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [122] 买卖股票的最佳时机II
 * https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-ii
 * - [贪心算法]
 */
use std::cmp::max;

pub fn max_profit2(prices: Vec<i32>) -> i32 {
    let mut ans = 0;

    for i in 1..prices.len() {
        ans += max(0, prices[i] - prices[i - 1]);
    }
    ans
}

#[cfg(test)]
mod tests {
    use crate::max_profit2::max_profit2;

    #[test]
    fn it_work_1() {
        assert_eq!(max_profit2(vec![7, 1, 5, 3, 6, 4]), 7);
    }
}
