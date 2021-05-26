use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    pub fn insert(&mut self, val: i32) {
        let node = Some(Rc::new(RefCell::new(TreeNode::new(val))));

        if self.left.is_none() {
            self.left = node;
        } else if self.right.is_none() {
            self.right = node;
        }
    }
    pub fn get_left(&self) -> &Rc<RefCell<TreeNode>> {
        self.left.as_ref().unwrap()
    }
    pub fn get_right(&self) -> &Rc<RefCell<TreeNode>> {
        self.right.as_ref().unwrap()
    }
}
