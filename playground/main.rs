use std::cell::RefCell;
use std::rc::Rc;
use tree::tree_utils::TreeNode;

fn main() {
    let mut tree = TreeNode::new(0);
    tree.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    tree.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    //
    tree.left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    tree.left.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));

    println!("{:?}", tree);
}
