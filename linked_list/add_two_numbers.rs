/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
 * https://leetcode-cn.com/problems/add-two-numbers/
 */


// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = ListNode::new(0);
    let mut tail= &mut head;
    let mut carry = 0;
    let mut p = l1;
    let mut q = l2;

    while p != None || q != None {
        let n1 = match &p {
            Some(x) => x.val,
            None => 0,
        };
        let n2 = match &q {
            Some(x) => x.val,
            None => 0,
        };

        let sum = n1 + n2 + carry;

        tail.next = Some(Box::new(ListNode::new(sum % 10)));
        tail = tail.next.as_mut().unwrap();

        p = p.and_then(|x| x.next);
        q = q.and_then(|x| x.next);

        // 功能与上面相同
        // p = match p {
        //     Some(l) => l.next,
        //     _ => None
        // };
        // q = match q {
        //     Some(l) => l.next,
        //     _ => None
        // };

        carry = sum / 10;
    }

    if carry > 0 {
        tail.next = Some(Box::new(ListNode::new(carry)));
    }
    head.next
}

/// 根据数组生成链表
fn generate_list_node(vecs: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Box::new(ListNode::new(0));
    let mut tail = &mut head;

    for x in vecs.iter() {
        tail.next = Some(Box::new(ListNode::new(*x)));
        tail = tail.next.as_mut().unwrap();
    }
    head.next
}

#[cfg(test)]
mod tests {
    use crate::add_two_numbers::{generate_list_node, add_two_numbers};

    #[test]
    fn it_works_1() {
        let a = vec![2,4,3];
        let b = vec![5,6,4];

        let al = generate_list_node(a);
        let bl = generate_list_node(b);
        let cl = add_two_numbers(al, bl);

        assert_eq!(cl, generate_list_node(vec![7,0,8]));
    }
    #[test]
    fn it_works_2() {
        let a = vec![0];
        let b = vec![0];

        let al = generate_list_node(a);
        let bl = generate_list_node(b);
        let cl = add_two_numbers(al, bl);

        assert_eq!(cl, generate_list_node(vec![0]));
    }
    #[test]
    fn it_works_3() {
        let a = vec![9,9,9,9,9,9,9];
        let b = vec![9,9,9,9];

        let al = generate_list_node(a);
        let bl = generate_list_node(b);
        let cl = add_two_numbers(al, bl);

        assert_eq!(cl, generate_list_node(vec![8,9,9,9,0,0,0,1]));
    }
}

