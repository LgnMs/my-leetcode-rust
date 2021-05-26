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

    #[test]
    fn it_work_1() {
        let mut tree = TreeNode::new(3);
        tree.insert(9);
        tree.insert(20);
        // let mut right_node = match tree.right {
        //     Some(node) => {
        //         let temp = *node.borrow_mut();
        //         *temp.right.unwrap().borrow_mut()
        //     },
        //     None => (),
        // };
        let right_node = *tree.right.unwrap().borrow();
        let mut right_node = *right_node.right.unwrap().borrow();
        right_node.insert(15);
        right_node.insert(7);
        println!("{:?}", tree);
    }
}
