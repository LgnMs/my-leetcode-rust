/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [144] 二叉树的前序遍历
 * https://leetcode-cn.com/problems/binary-tree-preorder-traversal/
 * ps: 在rust中使用迭代法太折磨了，我就不写了
 */
use crate::tree_utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut nums = vec![];

    fn traver(node: Option<&Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
        let node = match node {
            None => {
                return;
            }
            Some(x) => x,
        };
        nums.push(node.borrow().val);

        if node.borrow().left.is_some() {
            traver(node.borrow().left.as_ref(), nums);
        }
        if node.borrow().right.is_some() {
            traver(node.borrow().right.as_ref(), nums);
        }
    }
    traver(root.as_ref(), &mut nums);
    nums
}

#[cfg(test)]
mod test {
    use crate::preorder_traversal::preorder_traversal;
    use crate::tree_utils::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn it_work_1() {
        let mut tree = TreeNode::new(1);
        tree.insert(2);

        let tree = Some(Rc::new(RefCell::new(tree)));

        assert_eq!(preorder_traversal(tree), vec![1, 2]);
    }
}
