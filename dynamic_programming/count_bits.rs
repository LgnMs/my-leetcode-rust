use core::num;

/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * 338. 比特位计数
 * https://leetcode-cn.com/problems/counting-bits/
 */
pub fn count_bits_1(n: i32) -> Vec<i32> {
    fn count_one(mut x: i32) -> i32 {
        let mut ones = 0;
        while x > 0 {
            x &= x - 1;
            ones += 1;
        }
        ones
    }
    let mut nums = vec![];
    for i in 0..n + 1 {
        nums.push(count_one(i));
    }

    nums
}

pub fn count_bits(n: i32) -> Vec<i32> {
    fn count_one(mut x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        if x % 2 == 1 {
            return count_one(x - 1) + 1;
        }
        return count_one(x / 2);
    }
    let mut nums = vec![];

    for i in 0..n + 1 {
        nums.push(count_one(i));
    }
    nums
}

#[cfg(test)]
mod test {
    use crate::count_bits::count_bits;

    #[test]
    fn it_work_1() {
        assert_eq!(count_bits(2), vec![0, 1, 1]);
    }
}
