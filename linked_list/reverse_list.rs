/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [206] 反转链表
 * https://leetcode-cn.com/problems/reverse-linked-list/
 */

use crate::linked_list_utils::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev: Option<Box<ListNode>> = None;
    let mut curr = head;

    while let Some(mut node) = curr.take() {
        curr = node.next;
        node.next = prev;
        prev = Some(node);
    }

    prev
}

#[cfg(test)]
mod tests {
    use crate::linked_list_utils::generate_list_node;
    use crate::reverse_list::reverse_list;

    #[test]
    fn it_works_1() {
        let a = vec![1, 2];
        let b = vec![2, 1];

        let al = generate_list_node(a);
        let bl = generate_list_node(b);

        assert_eq!(reverse_list(al), bl);
    }
}