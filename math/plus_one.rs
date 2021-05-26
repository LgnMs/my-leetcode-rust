/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [66] 加一
 * https://leetcode-cn.com/problems/single-number/
 * - [数学]
 */
pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let mut temp = 1;
    let mut i = digits.len();

    while i > 0 as usize {
        let sum = digits[i - 1] + temp;
        digits[i - 1] = sum % 10;
        temp = sum / 10;
        if temp == 0 {
            break;
        }

        i -= 1;
    }
    if temp > 0 {
        digits.insert(0, temp);
    }
    digits
}

#[cfg(test)]
mod tests {
    use crate::plus_one::plus_one;

    #[test]
    fn it_work_1() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(plus_one(vec![9]), vec![1, 0]);
    }
}
