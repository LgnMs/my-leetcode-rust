use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub key: usize,
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32, key: usize) -> Self {
    TreeNode {
      key,
      val,
      left: None,
      right: None
    }
  }
}
