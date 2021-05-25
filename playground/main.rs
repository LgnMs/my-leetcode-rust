use tree::tree_utils::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let mut tree = TreeNode::new(0, 0);
    tree.left = Some(Rc::new(RefCell::new(TreeNode::new(1, 1))));
    tree.right = Some(Rc::new(RefCell::new(TreeNode::new(2, 2))));
    //
    tree.left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3, 3))));
    tree.left.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4, 4))));

    println!("{:?}", tree);
}
