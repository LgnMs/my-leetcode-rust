/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1486] 数组异或操作
 * https://leetcode-cn.com/problems/xor-operation-in-an-array/
 */

pub fn xor_operation(n: i32, start: i32) -> i32 {
    let mut sum = 0;

    for i in 0..n {
        sum ^= start + 2 * i;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::xor_operation::xor_operation;

    #[test]
    fn it_works_1() {
        assert_eq!(xor_operation(5, 0), 8);
        assert_eq!(xor_operation(4, 3), 8);
        assert_eq!(xor_operation(1, 7), 7);
        assert_eq!(xor_operation(10, 5), 2);
    }
}
