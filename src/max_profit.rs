/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [121] 买卖股票的最佳时机
 * https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock/
 * - [动态规划]
 */
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min = prices[0];
    let mut max_sum = 0;

    for x in prices {
        if x < min {
            min = x;
        } else  if x - min > max_sum {
            max_sum = x - min;
        }
    }

    max_sum
}

#[cfg(test)]
mod tests {
    use crate::max_profit::max_profit;

    #[test]
    fn it_work_1() {
        assert_eq!(max_profit(vec![2,4,1]), 2);
    }
}