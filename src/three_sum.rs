/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [15] 三数之和
 * https://leetcode-cn.com/problems/two-sum/
 * - [双指针]
 * 双指针问题一般都要先进行排序，本题也一样，而且涉及到三个数的和，就要多了一层循环来判断
 */
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums_sort = nums.clone();
    nums_sort.sort();
    let len = nums_sort.len();
    let mut i = 0;
    let mut vec_s: Vec<Vec<i32>> = vec![];

    if len < 3 {
        return vec_s;
    }

    // [-4, -1, -1, 0, 1, 2]
    while  i < len - 1 {
        let mut j = i + 1;
        let mut k = len - 1;
        while j < k {
            let ai = nums_sort[i];
            let bi = nums_sort[j];
            let ki = nums_sort[k];
            let sum = ai + bi + ki;

            if sum == 0 {
                let vec_item = vec![ai, bi, ki];
                if !vec_s.contains(&vec_item) {
                    vec_s.push(vec_item);
                }
            }
            if sum > 0 {
                k -= 1;
            } else {
                j += 1;
            }
        }
        i += 1;
    }

    vec_s
}

#[cfg(test)]
mod tests {
    use crate::three_sum::three_sum;

    #[test]
    fn it_work_1() {
        assert_eq!(three_sum(vec![-1,0,1,2,-1,-4]), vec![[-1,-1,2],[-1,0,1]]);
    }
    #[test]
    fn it_work_2() {
        assert_eq!(three_sum(vec![0, 0, 0]), vec![[0, 0, 0]]);
        assert_eq!(three_sum(vec![0, 0, 0, 0]), vec![[0, 0, 0]]);
        assert_eq!(three_sum(vec![-1,0,1,0]), vec![[-1,0,1]]);
    }

    #[test]
    fn it_work_3() {
        assert_eq!(three_sum(vec![-2,0,1,1,2]), vec![[-2,0,2],[-2,1,1]]);
    }
    #[test]
    fn it_work_4() {
        assert_eq!(three_sum(vec![-1,0,1,2,-1,-4,-2,-3,3,0,4]), vec![[-4,0,4],[-4,1,3],[-3,-1,4],[-3,0,3],[-3,1,2],[-2,-1,3],[-2,0,2],[-1,-1,2],[-1,0,1]]);
    }
}