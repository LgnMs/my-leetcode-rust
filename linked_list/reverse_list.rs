/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [206] 反转链表
 * https://leetcode-cn.com/problems/reverse-linked-list/
 */

use crate::linked_list_utils::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut result = ListNode::new(0);
    let mut nums:Vec<i32> = vec![];

    fn traverse(node: Option<Box<ListNode>>, nums: &mut Vec<i32>) {
        match node {
            None => (),
            Some(node) => {
                nums.push(node.val);
                if node.next != None {
                    traverse(node.next, nums);
                }
            },

        }
    }
    traverse(head, &mut nums);
    nums.reverse();

    let mut t = &mut result;
    for val in nums {
        t.next = Some(Box::new(ListNode::new(val)));
        t = t.next.as_mut().unwrap();
    }

    result.next
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