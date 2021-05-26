/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [104] 二叉树的最大深度
 * https://leetcode-cn.com/problems/maximum-depth-of-binary-tree/
 */
use crate::tree_utils::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

type Rt = Rc<RefCell<TreeNode>>;

pub fn max_depth(root: Option<Rt>) -> i32 {
    fn search(node: Option<&Rt>) -> i32 {
        match node {
            Some(x) => {
                1 + max(
                    search(x.borrow().left.as_ref()),
                    search(x.borrow().right.as_ref()),
                )
            }
            None => 0,
        }
    }
    match root {
        Some(_) => search(root.as_ref()),
        None => 0,
    }
}

#[cfg(test)]
mod test {
    use crate::tree_utils::TreeNode;
    use crate::max_depth::max_depth;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn it_work_1() {
        let mut tree = TreeNode::new(3);
        tree.insert(9);
        tree.insert(20);
        let right_node = tree.get_right();
        right_node.borrow_mut().insert(15);
        right_node.borrow_mut().insert(7);

        let tree = Some(Rc::new(RefCell::new(tree)));

        assert_eq!(max_depth(tree), 3);
    }
}
