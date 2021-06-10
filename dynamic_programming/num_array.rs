struct NumArray {
    nums: Vec<i32>,
}

/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * 303. 区域和检索 - 数组不可变
 * https://leetcode-cn.com/problems/range-sum-query-immutable/
 */

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let mut arr = vec![0];
        let mut pre = 0;
        for i in nums {
            pre += i;
            arr.push(pre);
        }
        NumArray {
            nums: arr,
        }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.nums[(right + 1) as usize] - self.nums[left as usize]
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */
#[cfg(test)]
mod test {
    #[test]
    fn it_work_1() {

    }
}