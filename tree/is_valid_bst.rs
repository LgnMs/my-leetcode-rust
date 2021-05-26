/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [98] 验证二叉搜索树
 * https://leetcode-cn.com/problems/validate-binary-search-tree/
 */

use std::rc::Rc;
use std::cell::RefCell;
use crate::tree_utils::TreeNode;

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn is_bst(node: Option<&Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        match node {
            None => {
                true
            },
            Some(x) => {
                if x.borrow().val as i64 <= min || x.borrow().val as i64 >= max {
                    return false;
                }
                is_bst(x.borrow().left.as_ref(), min, x.borrow().val as i64) && is_bst(x.borrow().right.as_ref(), x.borrow().val as i64, max)
            },
        }
    }

    is_bst(root.as_ref(), i64::MIN, i64::MAX)
}

#[cfg(test)]
mod test {
    use crate::tree_utils::TreeNode;
    use crate::is_valid_bst::is_valid_bst;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn it_work_1() {
        let mut tree = TreeNode::new(2);
        tree.insert(1);
        tree.insert(3);

        assert_eq!(is_valid_bst(Some(Rc::new(RefCell::new(tree)))), true);
    }
    #[test]
    fn it_work_2() {
        let mut tree = TreeNode::new(5);
        tree.insert(1);
        tree.insert(4);
        let right = tree.get_right();
        right.borrow_mut().insert(3);
        right.borrow_mut().insert(6);

        assert_eq!(is_valid_bst(Some(Rc::new(RefCell::new(tree)))), false);
    }
    #[test]
    fn it_work_3() {
        let mut tree = TreeNode::new(5);
        tree.insert(4);
        tree.insert(6);
        let right = tree.get_right();
        right.borrow_mut().insert(3);
        right.borrow_mut().insert(7);

        assert_eq!(is_valid_bst(Some(Rc::new(RefCell::new(tree)))), false);
    }
}