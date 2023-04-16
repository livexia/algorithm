#![allow(dead_code)]
pub struct Solution {}

use super::s0104_maximum_depth_of_binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Solution::is_subtree_ref(&root, &sub_root)
    }

    fn is_subtree_ref(
        root: &Option<Rc<RefCell<TreeNode>>>,
        sub_root: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root, sub_root) {
            (_, None) => true,
            (None, _) => false,
            _ => {
                Solution::is_identical(root, sub_root)
                    || Solution::is_subtree_ref(&root.as_ref().unwrap().borrow().left, sub_root)
                    || Solution::is_subtree_ref(&root.as_ref().unwrap().borrow().right, sub_root)
            }
        }
    }

    fn is_identical(
        t1: &Option<Rc<RefCell<TreeNode>>>,
        t2: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (t1, t2) {
            (None, None) => true,
            (Some(n1), Some(n2)) => {
                n1.borrow().val == n2.borrow().val
                    && Solution::is_identical(&n1.borrow().left, &n2.borrow().left)
                    && Solution::is_identical(&n1.borrow().right, &n2.borrow().right)
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests_572 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::is_subtree(
                TreeNode::from_vec(vec![Some(3), Some(4), Some(5), Some(1), Some(2)]),
                TreeNode::from_vec(vec![Some(4), Some(1), Some(2)])
            ),
            true
        );
        assert_eq!(
            Solution::is_subtree(
                TreeNode::from_vec(vec![
                    Some(3),
                    Some(4),
                    Some(5),
                    Some(1),
                    Some(2),
                    None,
                    None,
                    None,
                    None,
                    Some(0)
                ]),
                TreeNode::from_vec(vec![Some(4), Some(1), Some(2)])
            ),
            false
        );
    }
}
