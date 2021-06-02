/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [101] 对称二叉树
 * https://leetcode-cn.com/problems/symmetric-tree/
 */

use super::tree_utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub type Node = Rc<RefCell<TreeNode>>;

pub fn is_symmetric(root: Option<Node>) -> bool {
    fn seartch(p: Option<&Node>, q: Option<&Node>) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }
        if p.is_none() || q.is_none() {
            return false;
        }

        p.unwrap().borrow().val == q.unwrap().borrow().val
        && seartch(
            p.unwrap().borrow().left.as_ref(),
            q.unwrap().borrow().right.as_ref(),
        )
        && seartch(
            p.unwrap().borrow().right.as_ref(),
            q.unwrap().borrow().left.as_ref(),
        )
    }
    seartch(
        root.as_ref().unwrap().borrow().left.as_ref(),
        root.as_ref().unwrap().borrow().right.as_ref(),
    )
}


#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::{is_symmetric::is_symmetric, tree_utils::TreeNode};
    
    #[test]
    fn it_work_1() {
        let mut tree = TreeNode::new(1);
        tree.insert(2);
        tree.insert(2);
        let left = tree.get_left();
        left.borrow_mut().insert(3);
        left.borrow_mut().insert(4);
        let right = tree.get_right();
        right.borrow_mut().insert(4);
        right.borrow_mut().insert(3);

        assert_eq!(is_symmetric(Some(Rc::new(RefCell::new(tree)))), true);
    }
}