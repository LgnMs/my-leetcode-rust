/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [21] 合并两个有序链表
 * https://leetcode-cn.com/problems/merge-two-sorted-lists/
 */
use crate::linked_list_utils::ListNode;

pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(node), None) => Some(node), // 当其中一个链表为空的时候直接返回另一个链表
        (None, Some(node)) => Some(node),
        (Some(mut node1), Some(mut node2)) => {
            if node1.val < node2.val {
                let n = node1.next.take(); // 先将下一个节点的值绑定到n上，这时node.next的值已经变为了None
                node1.next = merge_two_lists(n, Some(node2)); // 重新为next绑定值，绑定的就是两个链表中较小的那一个链表
                Some(node1) // 返回较小的这个链表
            } else {
                let n = node2.next.take();
                node2.next = merge_two_lists(Some(node1), n);
                Some(node2)
            }
        }
        _ => None,
    }
}
#[cfg(test)]
mod tests {
    use crate::linked_list_utils::generate_list_node;
    use crate::merge_two_lists::merge_two_lists;

    #[test]
    fn it_work_1() {
        let a = vec![1, 2, 4];
        let b = vec![1, 3, 4];
        let al = generate_list_node(a);
        let bl = generate_list_node(b);
        let cl = generate_list_node(vec![1, 1, 2, 3, 4, 4]);

        assert_eq!(merge_two_lists(al, bl), cl);
    }
}
