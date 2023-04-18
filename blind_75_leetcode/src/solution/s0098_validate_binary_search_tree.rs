#![allow(dead_code)]
pub struct Solution {}

use crate::solution::s0104_maximum_depth_of_binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::validation_bst(&root).is_ok()
    }

    fn validation_bst(root: &Option<Rc<RefCell<TreeNode>>>) -> Result<Option<(i32, i32)>, ()> {
        if let Some(node) = root {
            let val = node.borrow().val;
            if let (Ok(l), Ok(r)) = (
                Solution::validation_bst(&node.borrow().left),
                Solution::validation_bst(&node.borrow().right),
            ) {
                match (l, r) {
                    (None, None) => return Ok(Some((val, val))),
                    (Some((l_min, l_max)), None) => {
                        if l_max < val {
                            return Ok(Some((l_min.min(val), val)));
                        }
                    }
                    (None, Some((r_min, r_max))) => {
                        if r_min > val {
                            return Ok(Some((val, r_max.max(val))));
                        }
                    }
                    (Some((l_min, l_max)), Some((r_min, r_max))) => {
                        if l_max < val && r_min > val {
                            return Ok(Some((l_min.min(val), r_max.max(val))));
                        }
                    }
                };
            }
            Err(())
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests_98 {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::is_valid_bst(TreeNode::from_vec(vec![Some(2), Some(1), Some(3)])),
            true
        );
        assert_eq!(
            Solution::is_valid_bst(TreeNode::from_vec(vec![Some(2147483647)])),
            true
        );
        assert_eq!(
            Solution::is_valid_bst(TreeNode::from_vec(vec![
                Some(5),
                Some(1),
                Some(4),
                None,
                None,
                Some(3),
                Some(6)
            ])),
            false
        );
        assert_eq!(
            Solution::is_valid_bst(TreeNode::from_vec(vec![
                Some(5),
                Some(4),
                Some(6),
                None,
                None,
                Some(3),
                Some(7)
            ])),
            false
        );
    }
}
