#![allow(dead_code)]
pub struct Solution {}

use super::s0104_maximum_depth_of_binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // p == q
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                let mut a = p.borrow_mut();
                let mut b = q.borrow_mut();
                a.val == b.val
                    && Solution::is_same_tree(a.left.take(), b.left.take())
                    && Solution::is_same_tree(a.right.take(), b.right.take())
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests_100 {
    use super::*;

    #[test]
    fn it_works() {
        let p = TreeNode::from_vec(vec![Some(1), Some(2), Some(3)]);
        let q = TreeNode::from_vec(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(Solution::is_same_tree(p, q), true);
    }
}
