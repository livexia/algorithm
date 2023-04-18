#![allow(dead_code)]
pub struct Solution {}

use crate::solution::s0104_maximum_depth_of_binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::validation_bst(&root).is_ok()
    }

    fn validation_bst(root: &Option<Rc<RefCell<TreeNode>>>) -> Result<Option<i32>, ()> {
        if let Some(node) = root {
            let val = node.borrow().val;
            match (
                Solution::validation_bst(&node.borrow().left),
                Solution::validation_bst(&node.borrow().right),
            ) {
                (Err(_), _) | (_, Err(_)) => Err(()),
                (Ok(r1), Ok(r2)) => {
                    let l_v = r1.unwrap_or(val - 1);
                    let r_v = r2.unwrap_or(val + 1);
                    if l_v < val && val < r_v {
                        Ok(Some(val))
                    } else {
                        Err(())
                    }
                }
            }
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
