#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
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
        let node = Some(Box::new(TreeNode::new(val)));

        if self.left.is_none() {
            self.left = node;
        } else if self.right.is_none() {
            self.right = node;
        }
    }
}
