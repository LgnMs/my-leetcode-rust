/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [98] 验证二叉搜索树
 * https://leetcode-cn.com/problems/validate-binary-search-tree/
 */

use crate::tree_utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

// 递归方法解决
pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn is_bst(node: Option<&Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        match node {
            None => true,
            Some(x) => {
                if x.borrow().val as i64 <= min || x.borrow().val as i64 >= max {
                    return false;
                }
                is_bst(x.borrow().left.as_ref(), min, x.borrow().val as i64)
                    && is_bst(x.borrow().right.as_ref(), x.borrow().val as i64, max)
            }
        }
    }

    is_bst(root.as_ref(), i64::MIN, i64::MAX)
}
// 中序遍历
pub fn is_valid_bst2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut min = i64::MIN;
    fn is_bst(node: Option<&Rc<RefCell<TreeNode>>>, min: &mut i64) -> bool {
        if node.is_none() {
            return true;
        }
        let mut bl = is_bst(node.unwrap().borrow().left.as_ref(), min);
        println!("{}", node.unwrap().borrow().val);
        if !bl {
            return false;
        }
        if (node.unwrap().borrow().val as i64) < *min {
            return false;
        }
        *min = node.unwrap().borrow().val as i64;
        bl = is_bst(node.unwrap().borrow().right.as_ref(), min);

        bl
    }
    is_bst(root.as_ref(), &mut min)
}
#[cfg(test)]
mod test {
    use crate::is_valid_bst::{is_valid_bst, is_valid_bst2};
    use crate::tree_utils::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn it_work_1() {
        let mut tree = TreeNode::new(2);
        tree.insert(1);
        tree.insert(3);

        let params = Some(Rc::new(RefCell::new(tree)));
        let params2 = params.clone();

        assert_eq!(is_valid_bst(params), true);
        assert_eq!(is_valid_bst2(params2), true);
    }
    #[test]
    fn it_work_2() {
        let mut tree = TreeNode::new(5);
        tree.insert(1);
        tree.insert(4);
        let right = tree.get_right();
        right.borrow_mut().insert(3);
        right.borrow_mut().insert(6);
        let params = Some(Rc::new(RefCell::new(tree)));
        let params2 = params.clone();

        assert_eq!(is_valid_bst(params), false);
        assert_eq!(is_valid_bst2(params2), false);
    }
    #[test]
    fn it_work_3() {
        //      5
        //     / \
        //    4   6
        //       / \
        //      3   7
        let mut tree = TreeNode::new(5);
        tree.insert(4);
        tree.insert(6);
        let right = tree.get_right();
        right.borrow_mut().insert(3);
        right.borrow_mut().insert(7);

        let params = Some(Rc::new(RefCell::new(tree)));
        let params2 = params.clone();

        assert_eq!(is_valid_bst(params), false);
        assert_eq!(is_valid_bst2(params2), false);
    }
}
