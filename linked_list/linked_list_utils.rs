// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// 根据数组生成链表
pub fn generate_list_node(vecs: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Box::new(ListNode::new(0));
    let mut tail = &mut head;

    for x in vecs.iter() {
        tail.next = Some(Box::new(ListNode::new(*x)));
        tail = tail.next.as_mut().unwrap();
    }
    head.next
}
