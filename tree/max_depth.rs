use std::rc::Rc;
use std::cell::RefCell;
use crate::tree_utils::TreeNode;
use std::cmp::max;

type Rt = Rc<RefCell<TreeNode>>;

pub fn max_depth(root: Option<Rt>) -> i32 {
    fn search(node: Option<&Rt>) -> i32 {
        match node {
            Some(x) => {
                1 + max(search(x.borrow().left.as_ref()), search(x.borrow().right.as_ref()))
            },
            None => 0,
        }
    }
    match root {
        Some(_) => {
            search(root.as_ref())
        },
        None => 0,
    }

    0
}

#[cfg(test)]
mod test {
    #[test]
    fn it_work_1() {

    }
}