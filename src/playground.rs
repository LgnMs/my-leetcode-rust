pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() as i32 - 1;
    let mut i = -1;

    while left <= right {

        let mid = left + (right - left) / 2;
        if nums[mid as usize] == target  {
            i = mid;
            break;
        } else if nums[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    i as i32
}


#[cfg(test)]
mod tests {
    use crate::playground::{search};

    #[test]
    fn it_works_1() {
        let nums = vec![5];

        assert_eq!(-1, search(nums, -5));
    }
}