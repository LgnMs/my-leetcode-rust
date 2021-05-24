/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * 234 回文链表
 * https://leetcode-cn.com/problems/palindrome-linked-list/
 */

use crate::linked_list_utils::ListNode;

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut nums = vec![];
    let mut curr = head;

    while let Some(node) = curr.take() {
        nums.push(node.val);
        curr = node.next;
    }
    let len = nums.len();

    for i in 0..len {
        let k = len - 1 - i;

        if nums[i] != nums[k] {
            return false    ;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::linked_list_utils::generate_list_node;
    use crate::is_palindrome::is_palindrome;

    #[test]
    fn it_work_1() {
        let a = vec![1, 2];
        let al = generate_list_node(a);

        assert_eq!(is_palindrome(al), false);
    }
    #[test]
    fn it_work_2() {
        let a = vec![1, 2, 2, 1];
        let al = generate_list_node(a);

        assert_eq!(is_palindrome(al), true);
    }
}